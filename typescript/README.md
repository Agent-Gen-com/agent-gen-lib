# agentgen · TypeScript / JavaScript SDK

TypeScript client for the [AgentGen API](https://www.agent-gen.com) — HTML → PDF and HTML → Image generation.

- **Zero runtime dependencies** — uses the native `fetch` API
- **Works everywhere** — Node.js 18+, Bun, Deno, and all modern browsers
- **Fully typed** — complete TypeScript types for all requests and responses
- **ESM-only** — ships as a pure ES module

---

## Installation

```bash
npm install agentgen
# or
pnpm add agentgen
# or
yarn add agentgen
```

> Requires **Node.js 18+** (or any runtime with native `fetch`).

---

## Quick start

```ts
import { AgentGenClient } from 'agentgen';

const client = new AgentGenClient({ apiKey: process.env.AGENTGEN_API_KEY! });

// Render HTML to a PNG image
const image = await client.generateImage({
  html: '<h1 style="font-family: sans-serif;">Hello, world!</h1>',
  width: 1200,
  height: 630,
});
console.log(image.url); // https://…/output.png

// Render HTML to a PDF
const pdf = await client.generatePdf({
  html: '<h1>Invoice #42</h1><p>Amount due: $99.00</p>',
  format: 'A4',
});
console.log(pdf.url); // https://…/output.pdf

// Check your token balance
const { tokens } = await client.getBalance();
console.log(`Remaining tokens: ${tokens}`);
```

---

## Constructor

```ts
const client = new AgentGenClient({
  apiKey: 'agk_...',   // required — your AgentGen API key
  baseUrl: '...',      // optional — override the API base URL
});
```

Get your API key at [agent-gen.com](https://www.agent-gen.com).

---

## Methods

### `generateImage(options)` → `Promise<GenerateImageResult>`

Renders an HTML string to a screenshot image. **Costs 1 token.**

```ts
const result = await client.generateImage({
  html: '<div style="background:#6366f1;color:#fff;padding:40px">Hello</div>',
  width: 1200,          // px, default 1200
  height: 630,          // px, default 630
  format: 'png',        // 'png' | 'jpeg' | 'webp', default 'png'
  device_scale_factor: 2, // 1–3, default 2 (retina-quality)
});

console.log(result.url);         // public URL, valid for 24 h
console.log(result.width);       // actual rendered width
console.log(result.height);      // actual rendered height
console.log(result.format);      // 'png' | 'jpeg' | 'webp'
console.log(result.tokens_used); // always 1
console.log(result.request_id);  // unique request identifier
```

Only `html` is required. All other fields have defaults.

---

### `generatePdf(options)` → `Promise<GeneratePdfResult>`

Renders HTML to a PDF. **Costs 2 tokens per page.**

#### Single-page PDF

```ts
const result = await client.generatePdf({
  html: `
    <html>
      <body style="font-family: sans-serif; padding: 40px;">
        <h1>Invoice #42</h1>
        <p>Amount due: $99.00</p>
      </body>
    </html>
  `,
  format: 'A4',            // 'A4' | 'Letter' | 'A3' | 'Legal', default 'A4'
  landscape: false,        // default false
  print_background: true,  // default true
  margin: {
    top: '20mm',
    bottom: '20mm',
    left: '15mm',
    right: '15mm',
  },
});

console.log(result.url);         // public URL pointing to the PDF
console.log(result.pages);       // number of pages generated
console.log(result.tokens_used); // pages × 2
console.log(result.request_id);
```

#### Multi-page PDF

Pass `{ pages: [...] }` instead of a single page object. Each entry in `pages` is an independent page with its own HTML and settings.

```ts
const result = await client.generatePdf({
  pages: [
    {
      html: '<h1 style="padding:40px">Page 1 — Cover</h1>',
      format: 'A4',
    },
    {
      html: '<h1 style="padding:40px">Page 2 — Content</h1>',
      format: 'A4',
      landscape: true,
    },
    {
      html: '<h1 style="padding:40px">Page 3 — Appendix</h1>',
      format: 'A4',
      margin: { top: '10mm', bottom: '10mm', left: '10mm', right: '10mm' },
    },
  ],
});
```

Up to 100 pages per request. Each page is rendered independently.

---

### `uploadTemp(file, filename?)` → `Promise<UploadTempResult>`

Uploads a file to temporary storage so you can reference it by URL inside your HTML. **Free — no tokens consumed.** Files are auto-deleted after **24 hours**.

This is useful for embedding local images or fonts in your HTML before calling `generateImage` or `generatePdf`.

```ts
// In Node.js — read a file from disk
import { readFileSync } from 'fs';

const upload = await client.uploadTemp(
  new Blob([readFileSync('./logo.png')], { type: 'image/png' }),
  'logo.png',
);

console.log(upload.url);        // public URL valid for 24 h
console.log(upload.key);        // storage key
console.log(upload.size);       // file size in bytes
console.log(upload.expires_at); // ISO 8601 expiry timestamp

// Now use upload.url inside your HTML
const image = await client.generateImage({
  html: `<img src="${upload.url}" style="width:200px" />`,
  width: 400,
  height: 200,
});
```

```ts
// In a browser — use a File from a file input
const fileInput = document.querySelector<HTMLInputElement>('input[type="file"]')!;
const file = fileInput.files![0];

const upload = await client.uploadTemp(file);
```

**Accepted file types:** `image/png`, `image/jpeg`, `image/webp`, `image/gif`, `image/svg+xml`. Max size: 10 MB.

---

### `getBalance()` → `Promise<BalanceResult>`

Returns the current token balance for the API key owner.

```ts
const { tokens } = await client.getBalance();
console.log(`You have ${tokens} tokens remaining.`);
```

---

### `createOrigin()` → `Promise<CreateOriginResult>`

Provisions a new public subdomain (`<id>.agent-gen.com`) for hosting files. Use this to obtain a stable origin URL for third-party integrations that require a specific allowed origin (e.g. Tesla virtual key setup).

```ts
const { id, origin } = await client.createOrigin();
console.log(origin); // https://abc123xyz.agent-gen.com
```

---

### `uploadOriginPublicKey(originId, pem)` → `Promise<UploadOriginPublicKeyResult>`

Uploads an EC public key (PEM format) to an origin subdomain. The key is stored at the standard Tesla virtual key path `/.well-known/appspecific/com.tesla.3p.public-key.pem`.

```ts
import { readFileSync } from 'fs';

const { id } = await client.createOrigin();
const pem = readFileSync('./public-key.pem', 'utf8');
const { url } = await client.uploadOriginPublicKey(id, pem);
console.log(url); // https://abc123xyz.agent-gen.com/.well-known/appspecific/com.tesla.3p.public-key.pem
```

---

## Error handling

All methods throw on non-2xx responses. Import the error classes to handle specific cases:

```ts
import { AgentGenClient, AgentGenError, InsufficientTokensError } from 'agentgen';

const client = new AgentGenClient({ apiKey: '...' });

try {
  const result = await client.generateImage({ html: '<h1>Hello</h1>' });
  console.log(result.url);
} catch (err) {
  if (err instanceof InsufficientTokensError) {
    console.error(
      `Not enough tokens. Have ${err.balance}, need ${err.required}.`,
      `Buy more at: ${err.buyMoreUrl}`,
    );
  } else if (err instanceof AgentGenError) {
    console.error(`API error ${err.status}: ${err.message}`);
    if (err.detail) console.error('Detail:', err.detail);
  } else {
    throw err; // network error, etc.
  }
}
```

### Error classes

#### `AgentGenError`

Thrown for all API errors (4xx / 5xx) except HTTP 402.

| Property | Type | Description |
|----------|------|-------------|
| `message` | `string` | Human-readable error message from the API |
| `status` | `number` | HTTP status code |
| `detail` | `string \| undefined` | Optional additional detail |
| `details` | `object \| undefined` | Validation error details (present on HTTP 422) |

#### `InsufficientTokensError` extends `AgentGenError`

Thrown when the account has too few tokens (HTTP 402).

| Property | Type | Description |
|----------|------|-------------|
| `balance` | `number` | Current token balance |
| `required` | `number` | Tokens needed for the request |
| `buyMoreUrl` | `string` | Direct link to purchase more tokens |

---

## TypeScript types reference

```ts
import type {
  // Options (inputs)
  GenerateImageOptions,
  GeneratePdfOptions,   // = PdfPage | { pages: PdfPage[] }
  PdfPage,
  PdfMargin,

  // Results (outputs)
  GenerateImageResult,
  GeneratePdfResult,
  UploadTempResult,
  BalanceResult,
  CreateOriginResult,
  UploadOriginPublicKeyResult,

  // Enums
  ImageFormat,  // 'png' | 'jpeg' | 'webp'
  PdfFormat,    // 'A4' | 'Letter' | 'A3' | 'Legal'
} from 'agentgen';
```

---

## Building from source

```bash
cd typescript
npm install
npm run build   # outputs to dist/
```

---

## Token pricing

| Operation | Cost |
|-----------|------|
| Generate image | 1 token |
| Generate PDF (per page) | 2 tokens |
| Upload temp file | Free |
| Check balance | Free |
| Create origin | Free |
| Upload origin public key | Free |

Purchase tokens at [agent-gen.com](https://www.agent-gen.com).
