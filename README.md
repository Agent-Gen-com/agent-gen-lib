# AgentGen SDKs

Official client libraries and CLI for the [AgentGen API](https://www.agent-gen.com) — an HTML → PDF and HTML → Image generation service built for AI agents and developers.

## CLI Installation

The `agentgen` CLI lets you call the API from any shell script or CI pipeline without writing code.

### One-liner (Linux & macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/Agent-Gen-com/agent-gen-lib/main/install.sh | sh
```

Installs to `~/.local/bin/agentgen`. Set `AGENTGEN_INSTALL=/usr/local/bin` to override.

### Homebrew

```sh
brew tap Agent-Gen-com/agentgen
brew install agentgen
```

### cargo (build from source)

```sh
cargo install agentgen-cli
```

### Quick start

```sh
export AGENTGEN_API_KEY=your_key_here
agentgen --help
```

Get your API key at [agent-gen.com](https://www.agent-gen.com).

---

## What's available

| Package | Language | Location |
|---------|----------|----------|
| `agentgen` | TypeScript / JavaScript | [`typescript/`](./typescript/) |
| `agentgen` | Python | [`python/`](./python/) |
| `agentgen` (lib) + `agentgen` (CLI) | Rust | [`rust/`](./rust/) |

---

## The API in one sentence

Send an HTML string → get back a publicly accessible URL pointing to a rendered PDF or image. That's it.

---

## API endpoints

All endpoints are authenticated with an `X-API-Key` header. Get your key at [agent-gen.com](https://www.agent-gen.com).

| Method | Endpoint | What it does | Cost |
|--------|----------|--------------|------|
| `POST` | `/v1/generate/image` | Render HTML → PNG / JPEG / WebP screenshot | 1 token |
| `POST` | `/v1/generate/pdf` | Render HTML → PDF (single or multi-page) | 2 tokens / page |
| `POST` | `/v1/upload/temp` | Upload a file for use inside HTML templates | Free |
| `GET` | `/v1/balance` | Check current token balance | Free |

### Generate image — key options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `html` | string | **required** | HTML to render (max 500 KB) |
| `width` | integer | 1200 | Viewport width in px (1–5000) |
| `height` | integer | 630 | Viewport height in px (1–5000) |
| `format` | `png` \| `jpeg` \| `webp` | `png` | Output format |
| `device_scale_factor` | number | 2 | Device pixel ratio (1–3) |

### Generate PDF — key options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `html` | string | **required** | HTML for a single-page PDF |
| `pages` | array | — | Array of page objects for multi-page PDFs |
| `format` | `A4` \| `Letter` \| `A3` \| `Legal` | `A4` | Paper size |
| `landscape` | boolean | false | Landscape orientation |
| `margin` | object | — | `{ top, bottom, left, right }` — CSS length strings, e.g. `"20mm"` |
| `print_background` | boolean | true | Whether to render CSS backgrounds |

Pass a single page object for a 1-page PDF, or `{ pages: [...] }` for multi-page.

### Upload temp file

Upload images, fonts, or other assets and reference them by URL inside your HTML before calling generate. Files are **publicly accessible** for **24 hours** and then automatically deleted. Max file size is **10 MB**.

---

## Error handling

Every SDK raises/throws a typed error on non-2xx responses. The most important one is the token-insufficient error (HTTP 402), which carries the current balance, the required amount, and a direct link to buy more tokens.

---

## Choosing a library

- **TypeScript / JavaScript** — zero dependencies, uses native `fetch`, works in Node.js 18+ and all modern browsers.
- **Python** — uses `httpx`, ships both a synchronous client and a fully `async`/`await`-compatible client.
- **Rust** — async-first with `tokio` + `reqwest`, builder-pattern request types, `thiserror`-based errors.
- **CLI** — the Rust binary `agentgen` lets you use the API from any shell script or CI pipeline without writing code.

See each sub-directory for a detailed README with installation and usage examples.
