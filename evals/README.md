# Hedwig honesty evals

This harness checks public claims against evidence that is present in this
repository. The default path is deterministic, uses only Node.js built-ins,
and makes no network requests.

The fixtures in [`cases/honesty.json`](cases/honesty.json) cover:

- devnet, pilot, adoption, and mainnet boundaries;
- security wording from `SECURITY.md` and `THREAT-MODEL.md`;
- the six Anchor instructions and repository-local TypeScript SDK exports; and
- the builder-owned consumer demo's devnet-only evidence boundary.

HyperBots documentation is intentionally outside this suite.

## Offline use

Run the suite and its unit tests from the repository root:

```bash
npm run evals:offline
npm run evals:test
```

`npm run evals:offline` is equivalent to `node evals/run.js --offline`.
`--dry-run` is also accepted. Offline mode is the default even when
`AI_GATEWAY_API_KEY` exists in the environment, so these commands never spend
Gateway credits.

Each case produces a pass/fail result and assertion details as JSON. Add or
update a fixture whenever public product copy or an exported integration
surface changes.

## Optional AI Gateway judge

The live judge is strictly opt-in:

```bash
AI_GATEWAY_API_KEY=... npm run evals:judge
```

If the variable is absent, judge mode reports `skipped` and exits without a
request. The key must be the capped `hedwig-evals-8` key: **$8 budget, no
refresh**. Never use an uncapped key for this evaluation. Keep the key in
the environment or a GitHub Actions secret; do not commit or print it.

The default model is `google/gemini-2.5-flash-lite`. The only supported
alternative is:

```bash
node evals/run.js --judge --model openai/gpt-5-nano
```

Requests use the OpenAI-compatible endpoint
`https://ai-gateway.vercel.sh/v1/chat/completions` and carry the Gateway tag
`feature:hedwig-eval`. The harness makes at most one request per invocation.
HTTP 402 is treated as a hard budget stop: it reports `budget-exhausted`,
makes no retry or fallback request, and exits cleanly.

The manual `Hedwig honesty evals` workflow always runs the offline suite. It
only attempts the live judge when the workflow input is enabled and the
`AI_GATEWAY_API_KEY` Actions secret is present.
