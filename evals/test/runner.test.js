const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "../..");

function loadRunner() {
  return require("../runner");
}

test("offline suite passes without making a network request", async () => {
  const { runHarness } = loadRunner();
  let requested = false;

  const result = await runHarness({
    repoRoot,
    judge: false,
    env: { AI_GATEWAY_API_KEY: "must-not-be-used" },
    fetchImpl: async () => {
      requested = true;
      throw new Error("offline mode attempted a network request");
    },
  });

  assert.equal(result.deterministic.failed, 0);
  assert.equal(result.deterministic.passed, result.deterministic.total);
  assert.equal(result.judge.status, "disabled");
  assert.equal(requested, false);
  assert.deepEqual(
    new Set(result.deterministic.results.map(({ category }) => category)),
    new Set(["network", "security", "integration", "consumer-demo"])
  );
});

test("judge mode skips cleanly when AI_GATEWAY_API_KEY is absent", async () => {
  const { runGatewayJudge } = loadRunner();
  let requested = false;

  const result = await runGatewayJudge({
    apiKey: undefined,
    report: { passed: 1, failed: 0, total: 1, results: [] },
    cases: [],
    fetchImpl: async () => {
      requested = true;
      throw new Error("judge without a key attempted a network request");
    },
  });

  assert.deepEqual(result, {
    status: "skipped",
    reason: "AI_GATEWAY_API_KEY is not set",
  });
  assert.equal(requested, false);
});

test("HTTP 402 stops the judge cleanly after one request", async () => {
  const { runGatewayJudge } = loadRunner();
  const requests = [];

  const result = await runGatewayJudge({
    apiKey: "test-key",
    report: { passed: 1, failed: 0, total: 1, results: [] },
    cases: [],
    fetchImpl: async (url, init) => {
      requests.push({ url, init });
      return {
        status: 402,
        ok: false,
        text: async () => "budget exhausted",
      };
    },
  });

  assert.deepEqual(result, {
    status: "budget-exhausted",
    reason: "AI Gateway returned HTTP 402; no further requests were made",
  });
  assert.equal(requests.length, 1);
  assert.equal(
    requests[0].url,
    "https://ai-gateway.vercel.sh/v1/chat/completions"
  );

  const body = JSON.parse(requests[0].init.body);
  assert.equal(body.model, "google/gemini-2.5-flash-lite");
  assert.deepEqual(body.providerOptions.gateway.tags, ["feature:hedwig-eval"]);
});

test("CLI defaults to offline and judge mode skips without a key", () => {
  const env = { ...process.env };
  delete env.AI_GATEWAY_API_KEY;

  const offline = spawnSync(process.execPath, ["evals/run.js"], {
    cwd: repoRoot,
    env,
    encoding: "utf8",
  });
  assert.equal(offline.status, 0, offline.stderr);
  assert.equal(JSON.parse(offline.stdout).judge.status, "disabled");

  const judge = spawnSync(process.execPath, ["evals/run.js", "--judge"], {
    cwd: repoRoot,
    env,
    encoding: "utf8",
  });
  assert.equal(judge.status, 0, judge.stderr);
  assert.equal(JSON.parse(judge.stdout).judge.status, "skipped");
});

test("pattern assertions reject equivalent production overclaims", () => {
  const { evaluateAssertion } = loadRunner();
  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "hedwig-honesty-eval-")
  );
  fs.writeFileSync(
    path.join(fixtureRoot, "copy.md"),
    "Hedwig is ready for production."
  );

  const result = evaluateAssertion(
    {
      type: "excludesPatterns",
      paths: ["copy.md"],
      patterns: ["\\bready\\s+for\\s+production\\b"],
    },
    fixtureRoot
  );

  assert.equal(result.pass, false);
  assert.match(result.detail, /ready\\s\+for\\s\+production/);
  fs.rmSync(fixtureRoot, { recursive: true, force: true });
});

test("Rust export extraction stops at the end of the program module", () => {
  const { extractRustProgramFunctions } = loadRunner();
  const source = `
#[program]
pub mod example {
    pub fn shipped() {}
}

pub fn internal_helper() {}
`;

  assert.deepEqual(extractRustProgramFunctions(source), ["shipped"]);
});
