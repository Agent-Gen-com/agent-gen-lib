# Publish AgentGen Packages

You are helping publish updated versions of the AgentGen client libraries (TypeScript/npm, Python/PyPI, Rust/crates.io).

Follow these steps in order:

---

## Step 1 — Fetch the latest OpenAPI schema

The OpenAPI schema is located online. Ask the user for the URL if not already known, then fetch it with WebFetch.

Use the schema to:
- Identify any new or changed endpoints, parameters, or types
- Update the client code in `typescript/src/`, `python/agentgen/`, and `rust/agentgen/src/` accordingly

---

## Step 2 — Update the README files

Update the root `README.md` and each library's own `README.md` (in `typescript/`, `python/`, `rust/`) to reflect:
- Any new endpoints or methods added
- Changed request/response types
- Updated usage examples matching the new API

---

## Step 3 — Bump versions consistently

Bump the patch version (or minor if there are new features) in all three manifests:
- `typescript/package.json` → `"version"`
- `python/pyproject.toml` → `version`
- `rust/agentgen/Cargo.toml` → `version` (also updates `rust/Cargo.lock` automatically on publish)

Keep all three versions in sync.

---

## Step 4 — Publish to npm (TypeScript)

```bash
cd typescript && npm publish --access public
```

Requires an npm Automation token set via:
```bash
npm set //registry.npmjs.org/:_authToken <token>
```

---

## Step 5 — Publish to PyPI (Python)

```bash
cd python && rm -rf dist && python -m build && twine upload dist/*
```

Requirements:
- `pip install build twine` (use `twine>=6` — older versions don't support Metadata-Version 2.4)
- PyPI credentials configured (token in `~/.pypirc` or via `TWINE_PASSWORD` env var)

---

## Step 6 — Publish to crates.io (Rust)

```bash
cd rust/agentgen && cargo publish
```

Requirements:
- `cargo login <token>` must have been run, or `CARGO_REGISTRY_TOKEN` env var set
- crates.io account must have a verified email address

---

## Step 7 — Commit and push

```bash
git add typescript/package.json python/pyproject.toml rust/agentgen/Cargo.toml rust/Cargo.lock
git add typescript/README.md python/README.md rust/README.md README.md
git commit -m "Release vX.Y.Z — <summary of changes>"
git push
```

---

## Package details reference

| Library | Registry | Package name | Manifest |
|---|---|---|---|
| TypeScript | npmjs.com | `agentgen` | `typescript/package.json` |
| Python | pypi.org | `agentgen` | `python/pyproject.toml` |
| Rust | crates.io | `agentgen` | `rust/agentgen/Cargo.toml` |

GitHub repo: https://github.com/Agent-Gen-com/agent-gen-lib

---

## Known gotchas

- **npm**: Cannot republish the same version — always bump before publishing
- **PyPI**: Same — versions are immutable once uploaded
- **crates.io**: Same, plus requires verified email on account
- **Python build**: `twine` must be v6+; hatchling generates Metadata-Version 2.4 which older twine rejects
- **Rust README**: Uses `readme = "../README.md"` (relative path from `rust/agentgen/` to the root README)
- **TOML structure**: In `pyproject.toml`, `[project.urls]` must be its own section after `[project]`, not nested inside it
