#!/usr/bin/env node
"use strict";

const path = require("node:path");
const { DEFAULT_MODEL, runHarness } = require("./runner");

function parseArguments(argv) {
  let judge = false;
  let model = DEFAULT_MODEL;
  let explicitOffline = false;

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--judge") {
      judge = true;
    } else if (argument === "--offline" || argument === "--dry-run") {
      explicitOffline = true;
    } else if (argument === "--model") {
      model = argv[index + 1];
      if (!model) {
        throw new Error("--model requires a provider/model value");
      }
      index += 1;
    } else if (argument === "--help" || argument === "-h") {
      return { help: true };
    } else {
      throw new Error(`Unknown argument: ${argument}`);
    }
  }

  if (judge && explicitOffline) {
    throw new Error("--judge cannot be combined with --offline or --dry-run");
  }
  return { help: false, judge, model };
}

function printHelp() {
  process.stdout.write(`Usage: node evals/run.js [options]

Options:
  --offline, --dry-run  Run deterministic checks only (default)
  --judge               Opt in to one AI Gateway judge request
  --model <id>          Judge model (default: ${DEFAULT_MODEL})
  -h, --help            Show this help
`);
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  if (options.help) {
    printHelp();
    return;
  }

  const result = await runHarness({
    repoRoot: path.resolve(__dirname, ".."),
    judge: options.judge,
    model: options.model,
  });
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);

  if (
    result.deterministic.failed > 0 ||
    (result.judge.status === "completed" && !result.judge.verdict.pass)
  ) {
    process.exitCode = 1;
  }
}

if (require.main === module) {
  main().catch((error) => {
    process.stderr.write(`hedwig-evals: ${error.message}\n`);
    process.exitCode = 2;
  });
}

module.exports = { main, parseArguments };
