# agentgen · Python SDK

Python client for the [AgentGen API](https://www.agent-gen.com) — HTML → PDF and HTML → Image generation.

- **Sync + async** — `AgentGenClient` for regular code, `AsyncAgentGenClient` for `asyncio` / `async`/`await`
- **Typed** — dataclass-based request/response types with full type annotations
- **Lightweight** — single dependency: [`httpx`](https://www.python-httpx.org/)

---

## Installation

```bash
pip install agentgen
```

Requires **Python 3.9+** and `httpx >= 0.25`.

---

## Quick start

```python
from agentgen import AgentGenClient, GenerateImageOptions, PdfPage

client = AgentGenClient(api_key="agk_...")

# Render HTML to a PNG image
image = client.generate_image(GenerateImageOptions(
    html="<h1 style='font-family:sans-serif'>Hello, world!</h1>",
    viewport_width=1200,
))
print(image.url)  # https://…/output.png

# Render HTML to a PDF
pdf = client.generate_pdf(PdfPage(
    html="<h1>Invoice #42</h1><p>Amount due: $99.00</p>",
    format="A4",
))
print(pdf.url)   # https://…/output.pdf

# Check token balance
balance = client.get_balance()
print(f"Remaining tokens: {balance.tokens}")
```

---

## Client classes

### `AgentGenClient` (synchronous)

```python
from agentgen import AgentGenClient

client = AgentGenClient(
    api_key="agk_...",                        # required
    base_url="https://www.agent-gen.com/api", # optional — default shown
)
```

### `AsyncAgentGenClient` (async / await)

```python
from agentgen import AsyncAgentGenClient

client = AsyncAgentGenClient(api_key="agk_...")

async def main():
    image = await client.generate_image(...)
```

Both classes expose exactly the same methods with the same signatures. The only difference is that `AsyncAgentGenClient` returns coroutines that must be awaited.

---

## Methods

### `generate_image(options)` → `GenerateImageResult`

Renders an HTML string to a screenshot image. **Costs 1 token.**

```python
from agentgen import GenerateImageOptions

result = client.generate_image(GenerateImageOptions(
    html="<div style='background:#6366f1;color:#fff;padding:40px'>Hello</div>",
    viewport_width=1200,    # px, default 1200
    viewport_height=800,    # px, default 800
    selector="#card",       # optional CSS selector to capture
    format="png",           # "png" | "jpeg" | "webp", default "png"
    device_scale_factor=2,  # 1–3, default 2 (retina-quality)
))

print(result.url)          # public URL, valid for 24 h
print(result.width)        # actual rendered width
print(result.height)       # actual rendered height
print(result.format)       # "png" | "jpeg" | "webp"
print(result.tokens_used)  # always 1
print(result.request_id)   # unique request identifier
```

Only `html` is required — all other fields default to `None` (server applies defaults).

---

### `generate_pdf(options)` → `GeneratePdfResult`

Renders HTML to a PDF document. **Costs 2 tokens per page.**

Pass a single `PdfPage` object for a one-page PDF, or a **list** of `PdfPage` objects for a multi-page PDF.

#### Single-page PDF

```python
from agentgen import PdfPage, PdfMargin

result = client.generate_pdf(PdfPage(
    html="""
    <html>
      <body style="font-family:sans-serif;padding:40px">
        <h1>Invoice #42</h1>
        <p>Amount due: $99.00</p>
      </body>
    </html>
    """,
    page_size_source="css", # prefer CSS @page size, fallback to format
    format="A4",            # "A4" | "Letter" | "A3" | "Legal"
    landscape=False,        # default False
    print_background=True,  # default True
    margin=PdfMargin(
        top="20mm",
        bottom="20mm",
        left="15mm",
        right="15mm",
    ),
))

print(result.url)          # public URL pointing to the PDF
print(result.pages)        # number of pages generated
print(result.tokens_used)  # pages × 2
print(result.request_id)
```

#### Multi-page PDF

Pass a list of `PdfPage` objects — each entry is an independent page with its own HTML and settings.

```python
result = client.generate_pdf([
    PdfPage(
        html="<h1 style='padding:40px'>Page 1 — Cover</h1>",
        page_size_source="css",
        format="A4",
    ),
    PdfPage(
        html="<h1 style='padding:40px'>Page 2 — Content</h1>",
        page_size_source="css",
        format="A4",
        landscape=True,
    ),
    PdfPage(
        html="<h1 style='padding:40px'>Page 3 — Appendix</h1>",
        format="A4",
        margin=PdfMargin(top="10mm", bottom="10mm", left="10mm", right="10mm"),
    ),
])

print(f"Generated {result.pages} pages, used {result.tokens_used} tokens")
```

Up to 100 pages per request.

---

### `upload_temp(file, filename=None)` → `UploadTempResult`

Uploads a file to temporary storage so you can embed it by URL inside your HTML before calling `generate_image` or `generate_pdf`. **Free — no tokens consumed.** Files are auto-deleted after **24 hours**.

```python
# Upload from a file path (str or pathlib.Path)
upload = client.upload_temp("./logo.png")

print(upload.url)         # public URL, valid for 24 h
print(upload.key)         # storage key
print(upload.size)        # file size in bytes
print(upload.expires_at)  # ISO 8601 expiry timestamp

# Now embed the URL in your HTML
result = client.generate_image(GenerateImageOptions(
    html=f'<img src="{upload.url}" style="width:200px" />',
    viewport_width=400,
))
```

```python
# Upload from an open file object
with open("./chart.svg", "rb") as f:
    upload = client.upload_temp(f, filename="chart.svg")
```

**Accepted types:** `image/png`, `image/jpeg`, `image/webp`, `image/gif`, `image/svg+xml`. Max size: **10 MB**.

---

### `get_balance()` → `BalanceResult`

Returns the current token balance.

```python
balance = client.get_balance()
print(f"You have {balance.tokens} tokens remaining.")
```

---

### `create_origin()` → `CreateOriginResult`

Provisions a new public subdomain (`<id>.agent-gen.com`) for hosting files. Use this to obtain a stable origin URL for third-party integrations that require a specific allowed origin (e.g. Tesla virtual key setup).

```python
result = client.create_origin()
print(result.id)      # "abc123xyz"
print(result.origin)  # "https://abc123xyz.agent-gen.com"
```

---

### `upload_origin_public_key(origin_id, pem)` → `UploadOriginPublicKeyResult`

Uploads an EC public key (PEM format) to an origin subdomain. The key is stored at the standard Tesla virtual key path `/.well-known/appspecific/com.tesla.3p.public-key.pem`.

```python
result = client.create_origin()

with open("public-key.pem") as f:
    pem = f.read()

key_result = client.upload_origin_public_key(result.id, pem)
print(key_result.url)
# https://abc123xyz.agent-gen.com/.well-known/appspecific/com.tesla.3p.public-key.pem
```

---

## Async usage

Every method above works identically with `AsyncAgentGenClient`. Just `await` each call:

```python
import asyncio
from agentgen import AsyncAgentGenClient, GenerateImageOptions, PdfPage

async def main():
    client = AsyncAgentGenClient(api_key="agk_...")

    # Run both requests concurrently
    image_coro = client.generate_image(GenerateImageOptions(
        html="<h1>OG Image</h1>",
        viewport_width=1200,
    ))
    pdf_coro = client.generate_pdf(PdfPage(
        html="<h1>Report</h1>",
        format="A4",
    ))

    image, pdf = await asyncio.gather(image_coro, pdf_coro)
    print(image.url, pdf.url)

asyncio.run(main())
```

---

## Error handling

```python
from agentgen import AgentGenClient, AgentGenError, InsufficientTokensError, GenerateImageOptions

client = AgentGenClient(api_key="agk_...")

try:
    result = client.generate_image(GenerateImageOptions(html="<h1>Hello</h1>"))
    print(result.url)

except InsufficientTokensError as e:
    print(f"Not enough tokens. Have {e.balance}, need {e.required}.")
    print(f"Buy more at: {e.buy_more_url}")

except AgentGenError as e:
    print(f"API error {e.status}: {e}")
    if e.detail:
        print(f"Detail: {e.detail}")
```

### Exception classes

#### `AgentGenError(Exception)`

Raised for all API errors (4xx / 5xx) except HTTP 402.

| Attribute | Type | Description |
|-----------|------|-------------|
| `args[0]` / `str(e)` | `str` | Human-readable error message |
| `status` | `int` | HTTP status code |
| `detail` | `str \| None` | Optional additional detail from the API |
| `details` | `dict \| None` | Validation field errors (present on HTTP 422) |

#### `InsufficientTokensError(AgentGenError)`

Raised when the account has too few tokens (HTTP 402).

| Attribute | Type | Description |
|-----------|------|-------------|
| `balance` | `int` | Current token balance |
| `required` | `int` | Tokens needed for this request |
| `buy_more_url` | `str` | Direct link to purchase more tokens |

---

## Type reference

```python
from agentgen import (
    # Clients
    AgentGenClient,
    AsyncAgentGenClient,

    # Request types
    GenerateImageOptions,
    PdfPage,
    PdfMargin,

    # Response types
    GenerateImageResult,
    GeneratePdfResult,
    UploadTempResult,
    BalanceResult,
    CreateOriginResult,
    UploadOriginPublicKeyResult,

    # Literal type aliases
    ImageFormat,   # Literal["png", "jpeg", "webp"]
    PdfFormat,     # Literal["A4", "Letter", "A3", "Legal"]

    # Exceptions
    AgentGenError,
    InsufficientTokensError,
)
```

### `GenerateImageOptions` fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `html` | `str` | **required** | HTML to render (max 500 KB) |
| `viewport_width` | `int \| None` | 1200 | Viewport width in px used for layout before capture (1–5000) |
| `viewport_height` | `int \| None` | 800 | Viewport height in px used for layout before capture (1–5000) |
| `selector` | `str \| None` | — | Optional CSS selector to capture instead of the full rendered document |
| `format` | `ImageFormat \| None` | `"png"` | Output format |
| `device_scale_factor` | `float \| None` | 2.0 | Device pixel ratio (1–3) |

### `PdfPage` fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `html` | `str` | **required** | HTML to render (max 500 KB) |
| `page_size_source` | `"css" \| "format" \| None` | `"css"` | Prefer CSS `@page` size or fallback format |
| `format` | `PdfFormat \| None` | `"A4"` | Fallback paper size |
| `landscape` | `bool \| None` | `False` | Landscape orientation |
| `margin` | `PdfMargin \| None` | — | Page margins |
| `print_background` | `bool \| None` | `True` | Render CSS backgrounds |

### `PdfMargin` fields

All fields are `str | None` and accept any CSS length value (e.g. `"20mm"`, `"1in"`, `"72pt"`).

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
