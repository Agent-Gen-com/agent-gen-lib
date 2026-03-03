# n8n-nodes-agentgen

[![npm version](https://img.shields.io/npm/v/n8n-nodes-agentgen.svg)](https://www.npmjs.com/package/n8n-nodes-agentgen)

This is an [n8n](https://n8n.io) community node for **[AgentGen](https://www.agent-gen.com)** — an HTML → PDF and HTML → Image generation service built for AI agents and developers.

Send an HTML string, get back a publicly accessible URL pointing to a rendered PDF or image.

[n8n](https://n8n.io/) is a [fair-code licensed](https://docs.n8n.io/reference/license/) workflow automation platform.

---

## Installation

Follow the [n8n community node installation guide](https://docs.n8n.io/integrations/community-nodes/installation/). Search for `n8n-nodes-agentgen`.

**Manual install:**
```bash
# In your n8n root directory
npm install n8n-nodes-agentgen
```

---

## Operations

| Operation | Description | Token cost |
|-----------|-------------|------------|
| **Generate Image** | Render HTML → PNG / JPEG / WebP screenshot | 1 token |
| **Generate PDF** | Render HTML → PDF (single or multi-page) | 2 tokens / page |
| **Upload Temp File** | Upload an asset for use inside HTML templates | Free |
| **Get Balance** | Check current token balance | Free |

### Generate Image

Renders an HTML string to an image file.

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| HTML | ✅ | — | HTML content to render (max 500 KB) |
| Width | — | 1200 | Viewport width in px (1–5000) |
| Height | — | 630 | Viewport height in px (1–5000) |
| Format | — | `png` | Output format: `png`, `jpeg`, or `webp` |
| Device Scale Factor | — | 2 | Pixel ratio (1–3). `2` = retina-quality |

**Output:** `{ url, width, height, format, tokens_used, request_id }`

### Generate PDF — Single Page

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| HTML | ✅ | — | HTML content (max 500 KB) |
| Format | — | `A4` | Paper size: `A4`, `A3`, `Letter`, `Legal` |
| Landscape | — | false | Landscape orientation |
| Margin Top/Bottom/Left/Right | — | — | CSS lengths e.g. `20mm`, `1in` |
| Print Background | — | true | Render CSS backgrounds |

### Generate PDF — Multi-Page

Accepts a JSON array of page objects. Each page can have its own layout:

```json
[
  { "html": "<h1>Cover</h1>", "format": "A4" },
  { "html": "<p>Content</p>", "format": "A4", "landscape": true }
]
```

**Output:** `{ url, pages, tokens_used, request_id }`

### Upload Temp File

Uploads a binary file from the workflow for use inside HTML `<img src="...">` or CSS references. Files are publicly accessible for **24 hours**, then auto-deleted.

Connect any node that produces binary data (e.g. **HTTP Request**, **Read Binary File**) before this node, then set the **Input Binary Field** to the binary property name.

**Output:** `{ url, key, size, expires_in, expires_at }`

### Get Balance

Returns the token balance for your API key.

**Output:** `{ tokens: number }`

---

## Credentials

Add an **AgentGen API** credential and paste your API key. Get one at [agent-gen.com](https://www.agent-gen.com).

n8n will automatically test the credential against the `/v1/balance` endpoint when you click **Test credential**.

---

## Compatibility

- **n8n** ≥ 1.0.0
- Tested against n8n-workflow 2.x

---

## Resources

- [AgentGen documentation](https://www.agent-gen.com)
- [n8n community nodes documentation](https://docs.n8n.io/integrations/community-nodes/)
- [Source code](https://github.com/Agent-Gen-com/agent-gen-lib/tree/main/n8n)

---

## License

[MIT](https://github.com/Agent-Gen-com/agent-gen-lib/blob/main/LICENSE)
