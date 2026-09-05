"use strict";

const fs = require("node:fs");
const path = require("node:path");

const GATEWAY_URL =
  "https://ai-gateway.vercel.sh/v1/chat/completions";
const DEFAULT_MODEL = "google/gemini-2.5-flash-lite";
const ALLOWED_MODELS = new Set([
  DEFAULT_MODEL,
  "openai/gpt-5-nano",
]);

function readRepositoryFile(repoRoot, relativePath) {
  const resolvedRoot = path.resolve(repoRoot);
  const resolvedPath = path.resolve(resolvedRoot, relativePath);
  if (
    resolvedPath !== resolvedRoot &&
    !resolvedPath.startsWith(`${resolvedRoot}${path.sep}`)
  ) {
    throw new Error(`Case path escapes the repository: ${relativePath}`);
  }
  return fs.readFileSync(resolvedPath, "utf8");
}

function normalizeWhitespace(value) {
  return value.replace(/\s+/g, " ").trim();
}

function extractRustProgramFunctions(source) {
  const programStart = source.indexOf("#[program]");
  if (programStart === -1) {
    return [];
  }

  const programSource = source.slice(programStart);
  return [...programSource.matchAll(/\bpub\s+fn\s+([a-zA-Z0-9_]+)\s*\(/g)].map(
    (match) => match[1],
  );
}

function extractTypeScriptExports(source) {
  const exports = new Set();
  for (const match of source.matchAll(/export(?:\s+type)?\s*\{([\s\S]*?)\}/g)) {
    for (const entry of match[1].split(",")) {
      const identifier = entry
        .trim()
        .replace(/^type\s+/, "")
        .split(/\s+as\s+/)[0]
        .trim();
      if (identifier) {
        exports.add(identifier);
      }
    }
  }
  return exports;
}

function evaluateAssertion(assertion, repoRoot) {
  switch (assertion.type) {
    case "contains": {
      const source = normalizeWhitespace(
        readRepositoryFile(repoRoot, assertion.path),
      );
      const missing = assertion.values.filter(
        (value) => !source.includes(normalizeWhitespace(value)),
      );
      return {
        pass: missing.length === 0,
        detail:
          missing.length === 0
            ? `${assertion.path} contains ${assertion.values.length} required evidence string(s)`
            : `${assertion.path} is missing: ${missing.join(" | ")}`,
      };
    }

    case "excludes": {
      const findings = [];
      for (const relativePath of assertion.paths) {
        const source = readRepositoryFile(repoRoot, relativePath).toLowerCase();
        for (const value of assertion.values) {
          if (source.includes(value.toLowerCase())) {
            findings.push(`${relativePath}: ${value}`);
          }
        }
      }
      return {
        pass: findings.length === 0,
        detail:
          findings.length === 0
            ? `${assertion.paths.length} file(s) exclude unsupported positive claims`
            : `unsupported claim text found: ${findings.join(" | ")}`,
      };
    }

    case "rustProgramFunctionsEqual": {
      const source = readRepositoryFile(repoRoot, assertion.path);
      const actual = extractRustProgramFunctions(source).sort();
      const expected = [...assertion.expected].sort();
      const pass = JSON.stringify(actual) === JSON.stringify(expected);
      return {
        pass,
        detail: pass
          ? `${assertion.path} exports exactly ${actual.join(", ")}`
          : `${assertion.path} exports [${actual.join(", ")}], expected [${expected.join(", ")}]`,
      };
    }

    case "documentedExports": {
      const documentation = readRepositoryFile(repoRoot, assertion.docsPath);
      const source = readRepositoryFile(repoRoot, assertion.sourcePath);
      const exported =
        assertion.language === "rust"
          ? new Set(extractRustProgramFunctions(source))
          : extractTypeScriptExports(source);
      const missingDocs = assertion.expected.filter(
        (name) => !documentation.includes(name),
      );
      const missingExports = assertion.expected.filter(
        (name) => !exported.has(name),
      );
      const pass = missingDocs.length === 0 && missingExports.length === 0;
      return {
        pass,
        detail: pass
          ? `${assertion.docsPath} names ${assertion.expected.length} export(s) present in ${assertion.sourcePath}`
          : `missing from docs [${missingDocs.join(", ")}]; missing from source exports [${missingExports.join(", ")}]`,
      };
    }

    case "exportsInclude": {
      const source = readRepositoryFile(repoRoot, assertion.path);
      const exported =
        assertion.language === "rust"
          ? new Set(extractRustProgramFunctions(source))
          : extractTypeScriptExports(source);
      const missing = assertion.expected.filter((name) => !exported.has(name));
      return {
        pass: missing.length === 0,
        detail:
          missing.length === 0
            ? `${assertion.path} exports ${assertion.expected.length} expected symbol(s)`
            : `${assertion.path} is missing exports: ${missing.join(", ")}`,
      };
    }

    default:
      throw new Error(`Unknown assertion type: ${assertion.type}`);
  }
}

function loadCases(repoRoot) {
  const casesPath = path.join(repoRoot, "evals", "cases", "honesty.json");
  const parsed = JSON.parse(fs.readFileSync(casesPath, "utf8"));
  if (!Array.isArray(parsed) || parsed.length === 0) {
    throw new Error("evals/cases/honesty.json must contain a non-empty array");
  }
  return parsed;
}

function runDeterministicSuite({ repoRoot, cases = loadCases(repoRoot) }) {
  const results = cases.map((testCase) => {
    const assertions = testCase.assertions.map((assertion) =>
      evaluateAssertion(assertion, repoRoot),
    );
    return {
      id: testCase.id,
      category: testCase.category,
      claim: testCase.claim,
      pass: assertions.every(({ pass }) => pass),
      assertions,
    };
  });
  const passed = results.filter(({ pass }) => pass).length;
  return {
    passed,
    failed: results.length - passed,
    total: results.length,
    results,
  };
}

async function runGatewayJudge({
  apiKey,
  report,
  cases,
  model = DEFAULT_MODEL,
  fetchImpl = globalThis.fetch,
}) {
  if (!apiKey) {
    return {
      status: "skipped",
      reason: "AI_GATEWAY_API_KEY is not set",
    };
  }
  if (!ALLOWED_MODELS.has(model)) {
    throw new Error(
      `Unsupported judge model "${model}". Use ${[...ALLOWED_MODELS].join(" or ")}`,
    );
  }
  if (typeof fetchImpl !== "function") {
    throw new Error("This Node runtime does not provide fetch");
  }

  const response = await fetchImpl(GATEWAY_URL, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${apiKey}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model,
      messages: [
        {
          role: "system",
          content:
            "You are a strict release-claim reviewer. Judge only the supplied claims and deterministic evidence. Reject overclaims. Return JSON with keys pass (boolean) and findings (array of short strings).",
        },
        {
          role: "user",
          content: JSON.stringify({
            task: "Check whether Hedwig's documented claims stay within its shipped devnet, security, integration, and pilot evidence.",
            cases: cases.map(({ id, category, claim }) => ({
              id,
              category,
              claim,
            })),
            deterministicReport: report,
          }),
        },
      ],
      temperature: 0,
      max_tokens: 400,
      response_format: { type: "json_object" },
      providerOptions: {
        gateway: {
          tags: ["feature:hedwig-eval"],
        },
      },
    }),
  });

  if (response.status === 402) {
    return {
      status: "budget-exhausted",
      reason: "AI Gateway returned HTTP 402; no further requests were made",
    };
  }
  if (!response.ok) {
    const detail = await response.text();
    throw new Error(
      `AI Gateway returned HTTP ${response.status}${detail ? `: ${detail}` : ""}`,
    );
  }

  const payload = await response.json();
  const content = payload?.choices?.[0]?.message?.content;
  if (typeof content !== "string") {
    throw new Error("AI Gateway response did not contain message content");
  }
  const verdict = JSON.parse(content);
  if (typeof verdict.pass !== "boolean" || !Array.isArray(verdict.findings)) {
    throw new Error("AI Gateway judge returned an invalid verdict");
  }
  return { status: "completed", model, verdict };
}

async function runHarness({
  repoRoot,
  judge = false,
  model = DEFAULT_MODEL,
  env = process.env,
  fetchImpl = globalThis.fetch,
}) {
  const cases = loadCases(repoRoot);
  const deterministic = runDeterministicSuite({ repoRoot, cases });
  const judgeResult = judge
    ? await runGatewayJudge({
        apiKey: env.AI_GATEWAY_API_KEY,
        report: deterministic,
        cases,
        model,
        fetchImpl,
      })
    : { status: "disabled" };
  return { deterministic, judge: judgeResult };
}

module.exports = {
  ALLOWED_MODELS,
  DEFAULT_MODEL,
  GATEWAY_URL,
  extractRustProgramFunctions,
  extractTypeScriptExports,
  loadCases,
  runDeterministicSuite,
  runGatewayJudge,
  runHarness,
};
