# n8n-nodes-agentgen

[![npm version](https://img.shields.io/npm/v/n8n-nodes-agentgen.svg)](https://www.npmjs.com/package/n8n-nodes-agentgen)

This is an [n8n](https://n8n.io) community node for **[AgentGen](https://www.agent-gen.com)** — an HTML → PDF and HTML → Image generation service built for AI agents and developers.

Send an HTML string, get back a publicly accessible URL pointing to a rendered PDF or image.

[n8n](https://n8n.io/) is a [fair-code licensed](https://docs.n8n.io/reference/license/) workflow automation platform.

---

## Installation (after n8n approval)

Once the package is listed in the n8n community registry, install it directly from the n8n UI:

1. Open n8n → **Settings** → **Community Nodes**
2. Click **Install a community node**
3. Enter `n8n-nodes-agentgen` and click **Install**

---

## Manual installation (before approval)

The node can be installed manually on any **self-hosted** n8n instance right now, without waiting for n8n's approval process. n8n Cloud does not support manual community node installation.

### Option A — Install from npm (recommended)

> Requires the package to be published to npm first (`npm publish` from the `n8n/` directory).

**Step 1 — Locate your n8n data directory**

This is the folder n8n uses for custom nodes. Default locations:

| How you run n8n | Default path |
|-----------------|--------------|
| npm global | `~/.n8n` |
| Docker | the volume mounted to `/home/node/.n8n` |
| n8n desktop | `~/.n8n` |

**Step 2 — Install the package**

```bash
cd ~/.n8n          # or your custom data directory
npm install n8n-nodes-agentgen
```

**Step 3 — Restart n8n**

```bash
# npm
n8n start

# Docker
docker restart <your-n8n-container>
```

**Step 4 — Verify** — Open n8n, add a new node, and search for **AgentGen**. It should appear under the Community section.

---

### Option B — Install from a local build (no npm publish needed)

Use this during development or to share the node privately.

**Step 1 — Build and pack**

```bash
cd n8n
npm run build
npm pack
# Produces: n8n-nodes-agentgen-0.1.0.tgz
```

**Step 2 — Install the tarball into n8n**

```bash
cd ~/.n8n          # your n8n data directory
npm install /path/to/n8n-nodes-agentgen-0.1.0.tgz
```

**Step 3 — Restart n8n** (same as Option A, Step 3)

---

## Using the node in a workflow

### 1. Set up your credential

1. In n8n, open **Credentials** → **Add Credential**
2. Search for **AgentGen API**
3. Paste your API key (get one at [agent-gen.com](https://www.agent-gen.com))
4. Click **Test credential** — n8n will call `/v1/balance` to confirm the key works
5. **Save**

### 2. Add the AgentGen node to a workflow

1. Open a workflow (or create a new one)
2. Click the **+** button to add a node
3. Search for **AgentGen** — select it
4. In the node panel, pick your **Operation** from the dropdown

---

### Walkthrough: Generate an image from HTML

1. Add an **AgentGen** node
2. Set **Operation** → `Generate Image`
3. In the **HTML** field, enter your markup:
   ```html
   <div style="font-family:sans-serif;padding:40px;background:#6C47FF;color:white;border-radius:12px">
     <h1>Hello from AgentGen 👋</h1>
     <p>Rendered by n8n</p>
   </div>
   ```
4. (Optional) expand **Image Options** to set width, height, or format
5. Select your **AgentGen API** credential
6. Click **Test step** — the output will contain a `url` field with a direct link to the rendered PNG

---

### Walkthrough: Generate a PDF from HTML

1. Add an **AgentGen** node, set **Operation** → `Generate PDF`
2. **PDF Mode** → `Single Page`
3. Enter your HTML in the **HTML** field
4. (Optional) expand **PDF Options** to set paper size, margins, or landscape mode
5. Run the node — output contains `url` (the PDF link) and `pages` (page count)

**Multi-page PDF:**

1. Set **PDF Mode** → `Multi-Page`
2. In the **Pages (JSON)** field, provide an array:
   ```json
   [
     { "html": "<h1 style='font-family:sans-serif'>Cover Page</h1>", "format": "A4" },
     { "html": "<p style='font-family:sans-serif'>Page two content</p>", "format": "A4" }
   ]
   ```
3. Run — you get a single PDF with one page per array element

---

### Walkthrough: Upload a temp file for use in HTML

> Use this when your HTML needs to reference a local image or font.

1. Add an **HTTP Request** node (or **Read Binary File**) before AgentGen to load your file into the workflow as binary data
2. Add an **AgentGen** node, set **Operation** → `Upload Temp File`
3. Set **Input Binary Field** to the name of the binary property from the previous node (usually `data`)
4. Run the node — the output contains a `url` valid for 24 hours
5. Use that URL in an `<img src="...">` tag in a subsequent **Generate Image** or **Generate PDF** node

---

### Walkthrough: Check your token balance

1. Add an **AgentGen** node, set **Operation** → `Get Balance`
2. Run — output: `{ "tokens": 42 }`

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
