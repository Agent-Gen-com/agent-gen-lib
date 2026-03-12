# agentgen · Rust SDK + CLI

Async Rust client and command-line tool for the [AgentGen API](https://www.agent-gen.com) — HTML → PDF and HTML → Image generation.

This directory is a Cargo workspace with two crates:

| Crate | Type | Description |
|-------|------|-------------|
| [`agentgen`](./agentgen/) | Library | Async Rust client — use in your own Rust projects |
| [`agentgen-cli`](./agentgen-cli/) | Binary | `agentgen` CLI — use from any shell or CI pipeline |

---

## Library — `agentgen`

### Adding to your project

```toml
# Cargo.toml
[dependencies]
agentgen = { path = "./agentgen" }   # local
# — or once published to crates.io —
agentgen = "0.1"

tokio = { version = "1", features = ["full"] }
```

### Quick start

```rust
use agentgen::{AgentGenClient, types::{GenerateImageRequest, ImageFormat, PdfPage, PdfFormat, GeneratePdfRequest}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AgentGenClient::new(std::env::var("AGENTGEN_API_KEY")?);

    // Render HTML to a PNG image
    let image = client
        .generate_image(
            GenerateImageRequest::new("<h1 style='font-family:sans-serif'>Hello!</h1>")
                .viewport_width(1200)
                .format(ImageFormat::Png),
        )
        .await?;

    println!("Image URL: {}", image.url);

    // Render HTML to a PDF
    let pdf = client
        .generate_pdf(GeneratePdfRequest::SinglePage(
            PdfPage::new("<h1>Invoice #42</h1><p>Amount due: $99.00</p>")
                .format(PdfFormat::A4)
                .print_background(true),
        ))
        .await?;

    println!("PDF URL: {}", pdf.url);

    // Check balance
    let balance = client.get_balance().await?;
    println!("Remaining tokens: {}", balance.tokens);

    Ok(())
}
```

---

### Creating the client

```rust
// Production (default base URL: https://www.agent-gen.com/api)
let client = AgentGenClient::new("agk_...");

// Custom base URL — useful for local development or testing
let client = AgentGenClient::with_base_url("agk_...", "http://localhost:3000/api");
```

---

### `generate_image` — Costs 1 token

Build a `GenerateImageRequest` with the provided builder methods and await the call.

```rust
use agentgen::types::{GenerateImageRequest, ImageFormat};

let result = client
    .generate_image(
        GenerateImageRequest::new("<div style='background:#6366f1;color:#fff;padding:40px'>Hello</div>")
            .viewport_width(1200)      // default: 1200
            .viewport_height(800)      // default: 800
            .selector("#card")         // optional CSS selector to capture
            .format(ImageFormat::Png)  // Png | Jpeg | Webp, default Png
            .device_scale_factor(2.0), // 1.0–3.0, default 2.0
    )
    .await?;

println!("URL:        {}", result.url);
println!("Size:       {}×{}", result.width, result.height);
println!("Format:     {:?}", result.format);
println!("Tokens:     {}", result.tokens_used);
println!("Request ID: {}", result.request_id);
```

`GenerateImageRequest::new(html)` is the only required call; all builder methods are optional.

---

### `generate_pdf` — Costs 2 tokens per page

Pass a `GeneratePdfRequest` — either `SinglePage(PdfPage)` for one page, or `MultiPage { pages: Vec<PdfPage> }` for multiple pages.

#### Single-page PDF

```rust
use agentgen::types::{GeneratePdfRequest, PdfPage, PdfFormat, PdfMargin, PdfPageSizeSource};

let result = client
    .generate_pdf(GeneratePdfRequest::SinglePage(
        PdfPage::new(r#"
            <html>
              <body style="font-family:sans-serif;padding:40px">
                <h1>Invoice #42</h1>
                <p>Amount due: $99.00</p>
              </body>
            </html>
        "#)
        .page_size_source(PdfPageSizeSource::Css)
        .format(PdfFormat::A4)        // A4 | Letter | A3 | Legal
        .landscape(false)             // default false
        .print_background(true)       // default true
        .margin(PdfMargin {
            top: Some("20mm".into()),
            bottom: Some("20mm".into()),
            left: Some("15mm".into()),
            right: Some("15mm".into()),
        }),
    ))
    .await?;

println!("PDF URL:   {}", result.url);
println!("Pages:     {}", result.pages);
println!("Tokens:    {}", result.tokens_used);
```

#### Shorthand for uniform margins

```rust
use agentgen::types::PdfMargin;

// Sets top/bottom/left/right to the same value
let margin = PdfMargin::all("20mm");
```

#### Multi-page PDF

```rust
let result = client
    .generate_pdf(GeneratePdfRequest::MultiPage {
        pages: vec![
            PdfPage::new("<h1 style='padding:40px'>Page 1 — Cover</h1>")
                .page_size_source(PdfPageSizeSource::Css)
                .format(PdfFormat::A4),
            PdfPage::new("<h1 style='padding:40px'>Page 2 — Content</h1>")
                .page_size_source(PdfPageSizeSource::Css)
                .format(PdfFormat::A4)
                .landscape(true),
            PdfPage::new("<h1 style='padding:40px'>Page 3 — Appendix</h1>")
                .page_size_source(PdfPageSizeSource::Css)
                .format(PdfFormat::A4)
                .margin(PdfMargin::all("10mm")),
        ],
    })
    .await?;

println!("Generated {} pages, used {} tokens", result.pages, result.tokens_used);
```

Up to 100 pages per request.

---

### `upload_temp` — Free

Upload a local file so you can reference it by URL inside your HTML. Auto-deleted after **24 hours**.

```rust
use std::path::Path;

let upload = client.upload_temp(Path::new("./logo.png")).await?;

println!("URL:        {}", upload.url);
println!("Key:        {}", upload.key);
println!("Size:       {} bytes", upload.size);
println!("Expires at: {}", upload.expires_at);

// Embed the URL in HTML and then generate
let image = client
    .generate_image(
        GenerateImageRequest::new(format!(
            r#"<img src="{}" style="width:200px" />"#,
            upload.url
        )),
    )
    .await?;
```

---

### `get_balance` — Free

```rust
let balance = client.get_balance().await?;
println!("Remaining tokens: {}", balance.tokens);
```

---

### Error handling

All methods return `Result<T, AgentGenError>`. The error type is defined in `agentgen::AgentGenError`:

```rust
use agentgen::AgentGenError;

match client.generate_image(req).await {
    Ok(result) => println!("{}", result.url),
    Err(AgentGenError::InsufficientTokens { balance, required, buy_more_url }) => {
        eprintln!("Not enough tokens. Have {balance}, need {required}.");
        eprintln!("Buy more at: {buy_more_url}");
    }
    Err(AgentGenError::Api { status, message, detail }) => {
        eprintln!("API error {status}: {message}");
        if let Some(d) = detail {
            eprintln!("Detail: {d}");
        }
    }
    Err(AgentGenError::Http(e)) => eprintln!("Network error: {e}"),
    Err(AgentGenError::Io(e)) => eprintln!("I/O error: {e}"),
}
```

#### `AgentGenError` variants

| Variant | When | Fields |
|---------|------|--------|
| `Api { status, message, detail }` | Non-2xx API response (except 402) | HTTP status, error message, optional detail |
| `InsufficientTokens { balance, required, buy_more_url }` | HTTP 402 | Current balance, required tokens, buy URL |
| `Http(reqwest::Error)` | Network / transport failure | Wrapped reqwest error |
| `Io(std::io::Error)` | File read failure (upload_temp) | Wrapped I/O error |

---

### Type reference

```rust
use agentgen::types::{
    // Image
    GenerateImageRequest,
    GenerateImageResponse,
    ImageFormat,        // Png | Jpeg | Webp

    // PDF
    GeneratePdfRequest, // SinglePage(PdfPage) | MultiPage { pages }
    GeneratePdfResponse,
    PdfPage,
    PdfMargin,
    PdfFormat,          // A4 | Letter | A3 | Legal
    PdfPageSizeSource,  // Css | Format

    // Upload
    UploadTempResponse,

    // Balance
    BalanceResponse,
};
```

---

## CLI — `agentgen`

The `agentgen` binary lets you use the API directly from a terminal, shell script, or CI pipeline without writing any code.

### Building

```bash
# From the rust/ workspace root
cargo build --release -p agentgen-cli

# The binary is at:
./target/release/agentgen
```

Or install it system-wide:

```bash
cargo install --path agentgen-cli
```

### Authentication

Provide your API key via the `AGENTGEN_API_KEY` environment variable (recommended) or the `--api-key` flag:

```bash
export AGENTGEN_API_KEY="agk_..."

# or pass it inline
agentgen --api-key agk_... balance
```

---

### `agentgen balance`

Check your current token balance.

```bash
$ agentgen balance
Token balance: 42
```

---

### `agentgen image`

Render HTML to an image (PNG / JPEG / WebP). **Costs 1 token.**

```
agentgen image (--html <string> | --file <path>)
               [--viewport-width <px>]
               [--viewport-height <px>]
               [--selector <css>]
               [--format png|jpeg|webp]
               [--scale <1-3>]
               [--output <path>]
```

**Inline HTML:**

```bash
agentgen image --html '<h1 style="font-family:sans-serif">Hello!</h1>' \
               --viewport-width 1200
```

**From a file:**

```bash
agentgen image --file ./template.html --viewport-width 1200 --format png
```

**Download the result locally:**

```bash
agentgen image --file ./og-template.html \
               --viewport-width 1200 \
               --selector '#card' \
               --output og-image.png
```

**Example output:**

```
✓ Image generated
  URL:           https://…/output.png
  Dimensions:    1200 × 630 px
  Format:        PNG
  Tokens used:   1
  Request ID:    req_abc123
✓ Saved → og-image.png
```

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--html <string>` | — | Inline HTML string to render |
| `--file <path>` | — | Read HTML from a file (mutually exclusive with `--html`) |
| `--viewport-width <px>` | 1200 | Viewport width in pixels used for layout before capture |
| `--viewport-height <px>` | 800 | Viewport height in pixels used for layout before capture |
| `--selector <css>` | — | Capture a specific element instead of the full rendered document |
| `--format` | `png` | Output format: `png`, `jpeg`, or `webp` |
| `--scale <n>` | 2 | Device pixel ratio (1–3) |
| `--output / -o <path>` | — | Download the generated image to this path |

---

### `agentgen pdf`

Render HTML to a PDF. **Costs 2 tokens per page.**

```
agentgen pdf (--html <string> | --file <path> | --pages <file1> <file2> …)
             [--page-size-source css|format]
             [--format A4|Letter|A3|Legal]
             [--landscape]
             [--print-background]
             [--margin-top <value>] [--margin-bottom <value>]
             [--margin-left <value>] [--margin-right <value>]
             [--output <path>]
```

**Single-page — inline HTML:**

```bash
agentgen pdf --html '<h1>Invoice #42</h1>' \
             --page-size-source css \
             --format A4 \
             --margin-top 20mm --margin-bottom 20mm \
             --output invoice.pdf
```

**Single-page — from a file:**

```bash
agentgen pdf --file ./invoice.html \
             --page-size-source css \
             --format A4 \
             --print-background \
             --output invoice.pdf
```

**Multi-page — one HTML file per page:**

```bash
agentgen pdf --pages cover.html content.html appendix.html \
             --page-size-source css \
             --format A4 \
             --output report.pdf
```

Each file in `--pages` becomes one independent page. All pages share the same `--format`, `--landscape`, and margin settings.

**Example output:**

```
✓ PDF generated
  URL:           https://…/output.pdf
  Pages:         3 page(s)
  Tokens used:   6 token(s)
  Request ID:    req_def456
✓ Saved → report.pdf
```

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--html <string>` | — | Inline HTML for a single-page PDF |
| `--file <path>` | — | Read HTML from a file (single-page) |
| `--pages <files…>` | — | Multiple HTML files — one per page |
| `--page-size-source` | `css` | Prefer CSS `@page` size or fallback format |
| `--format` | `A4` | Fallback paper size: `A4`, `Letter`, `A3`, `Legal` |
| `--landscape` | off | Landscape orientation |
| `--print-background` | off | Render CSS backgrounds |
| `--margin-top <value>` | — | Top margin (CSS length, e.g. `20mm`) |
| `--margin-bottom <value>` | — | Bottom margin |
| `--margin-left <value>` | — | Left margin |
| `--margin-right <value>` | — | Right margin |
| `--output / -o <path>` | — | Download the generated PDF to this path |

---

### `agentgen upload`

Upload a local file to temporary storage. **Free — no tokens consumed.** Auto-deleted after **24 hours**.

```bash
agentgen upload ./logo.png
```

**Example output:**

```
✓ File uploaded
  URL:           https://…/logo.png
  Key:           tmp/…/logo.png
  Size:          14832 bytes
  Expires at:    2026-03-03T10:00:00.000Z
```

Use the printed URL inside your HTML before calling `image` or `pdf`.

---

### Global flags

| Flag | Description |
|------|-------------|
| `--api-key <key>` | API key (or set `AGENTGEN_API_KEY` env var) |
| `--version` | Print version and exit |
| `--help` | Print help and exit |

---

## Token pricing

| Operation | Cost |
|-----------|------|
| Generate image | 1 token |
| Generate PDF (per page) | 2 tokens |
| Upload temp file | Free |
| Check balance | Free |

Purchase tokens at [agent-gen.com](https://www.agent-gen.com).
