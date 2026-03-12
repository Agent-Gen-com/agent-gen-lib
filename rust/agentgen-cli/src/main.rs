use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use agentgen::{
    AgentGenClient,
    types::{
        GenerateImageRequest, GeneratePdfRequest, ImageFormat, PdfFormat, PdfMargin, PdfPage,
        PdfPageSizeSource,
    },
};
use std::fs;

// ── CLI definition ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "agentgen",
    about = "AgentGen CLI — render HTML to PDF or Image via the AgentGen API",
    version,
    arg_required_else_help = true
)]
struct Cli {
    /// AgentGen API key. Can also be set via AGENTGEN_API_KEY env var.
    #[arg(long, env = "AGENTGEN_API_KEY", global = true, hide_env_values = true)]
    api_key: String,

    /// Override the API base URL (default: https://www.agent-gen.com/api).
    #[arg(
        long,
        default_value = "https://www.agent-gen.com/api",
        global = true,
        hide = true
    )]
    base_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render HTML to an image (PNG/JPEG/WebP). Costs 1 token.
    Image(ImageArgs),
    /// Render HTML to a PDF (single or multi-page). Costs 2 tokens per page.
    Pdf(PdfArgs),
    /// Upload a file for use inside HTML templates (free, 24 h TTL).
    Upload(UploadArgs),
    /// Show the current token balance.
    Balance,
    /// Provision a new public origin subdomain (free).
    Origin,
    /// Upload an EC public key (PEM) to an origin subdomain (free).
    PublicKey(PublicKeyArgs),
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct HtmlSource {
    /// Inline HTML string to render.
    #[arg(long)]
    html: Option<String>,

    /// Read HTML from a file.
    #[arg(long)]
    file: Option<PathBuf>,
}

#[derive(Args)]
struct ImageArgs {
    #[command(flatten)]
    source: HtmlSource,

    /// Viewport width in pixels used for layout before capture (default 1200).
    #[arg(long)]
    viewport_width: Option<u32>,

    /// Viewport height in pixels used for layout before capture (default 800).
    #[arg(long)]
    viewport_height: Option<u32>,

    /// Optional CSS selector to capture instead of the full rendered document.
    #[arg(long)]
    selector: Option<String>,

    /// Output format: png | jpeg | webp (default png).
    #[arg(long, default_value = "png")]
    format: String,

    /// Device pixel ratio, 1–3 (default 2).
    #[arg(long)]
    scale: Option<f64>,

    /// Download the generated image to this local path.
    #[arg(long, short)]
    output: Option<PathBuf>,
}

#[derive(Args)]
struct PdfArgs {
    /// Inline HTML string (single-page PDF).
    #[arg(long, group = "pdf_source")]
    html: Option<String>,

    /// Read HTML from a file (single-page PDF).
    #[arg(long, group = "pdf_source")]
    file: Option<PathBuf>,

    /// Paths to multiple HTML files — one file = one page (multi-page PDF).
    #[arg(long, group = "pdf_source", num_args = 1..)]
    pages: Option<Vec<PathBuf>>,

    /// Paper format: A4 | Letter | A3 | Legal (default A4).
    #[arg(long)]
    format: Option<String>,

    /// Page sizing mode: css | format (default css).
    #[arg(long)]
    page_size_source: Option<String>,

    /// Landscape orientation.
    #[arg(long)]
    landscape: bool,

    /// Print CSS backgrounds.
    #[arg(long)]
    print_background: bool,

    /// Top margin (e.g. 20mm).
    #[arg(long)]
    margin_top: Option<String>,

    /// Bottom margin.
    #[arg(long)]
    margin_bottom: Option<String>,

    /// Left margin.
    #[arg(long)]
    margin_left: Option<String>,

    /// Right margin.
    #[arg(long)]
    margin_right: Option<String>,

    /// Download the generated PDF to this local path.
    #[arg(long, short)]
    output: Option<PathBuf>,
}

#[derive(Args)]
struct UploadArgs {
    /// Path to the file to upload.
    file: PathBuf,
}

#[derive(Args)]
struct PublicKeyArgs {
    /// Origin ID returned by `agentgen origin`.
    origin_id: String,

    /// Path to the PEM public key file.
    pem_file: PathBuf,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn read_html(html: Option<String>, file: Option<PathBuf>) -> Result<String> {
    if let Some(h) = html {
        return Ok(h);
    }
    if let Some(path) = file {
        return std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()));
    }
    bail!("provide --html <string> or --file <path>")
}

fn parse_image_format(s: &str) -> ImageFormat {
    match s.to_lowercase().as_str() {
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::Webp,
        _ => ImageFormat::Png,
    }
}

fn parse_pdf_format(s: &str) -> Option<PdfFormat> {
    match s.to_uppercase().as_str() {
        "A4" => Some(PdfFormat::A4),
        "LETTER" => Some(PdfFormat::Letter),
        "A3" => Some(PdfFormat::A3),
        "LEGAL" => Some(PdfFormat::Legal),
        _ => None,
    }
}

fn parse_page_size_source(s: &str) -> Option<PdfPageSizeSource> {
    match s.to_lowercase().as_str() {
        "css" => Some(PdfPageSizeSource::Css),
        "format" => Some(PdfPageSizeSource::Format),
        _ => None,
    }
}

fn build_page(
    html: String,
    format: Option<&str>,
    page_size_source: Option<&str>,
    landscape: bool,
    print_background: bool,
    margin_top: Option<String>,
    margin_bottom: Option<String>,
    margin_left: Option<String>,
    margin_right: Option<String>,
) -> PdfPage {
    let mut page = PdfPage::new(html);
    if let Some(source) = page_size_source.and_then(parse_page_size_source) {
        page = page.page_size_source(source);
    }
    if let Some(fmt) = format.and_then(parse_pdf_format) {
        page = page.format(fmt);
    }
    if landscape {
        page = page.landscape(true);
    }
    if print_background {
        page = page.print_background(true);
    }
    let has_margin = margin_top.is_some()
        || margin_bottom.is_some()
        || margin_left.is_some()
        || margin_right.is_some();
    if has_margin {
        page = page.margin(PdfMargin {
            top: margin_top,
            bottom: margin_bottom,
            left: margin_left,
            right: margin_right,
        });
    }
    page
}

async fn download_to(url: &str, path: &PathBuf) -> Result<()> {
    let bytes = reqwest::get(url)
        .await
        .context("failed to download file")?
        .bytes()
        .await?;
    tokio::fs::write(path, &bytes)
        .await
        .with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = AgentGenClient::with_base_url(&cli.api_key, &cli.base_url);

    if let Err(err) = run(cli.command, &client).await {
        eprintln!("{} {}", "error:".red().bold(), err);
        std::process::exit(1);
    }
}

async fn run(command: Commands, client: &AgentGenClient) -> Result<()> {
    match command {
        // ── balance ──────────────────────────────────────────────────────────
        Commands::Balance => {
            let b = client.get_balance().await?;
            println!(
                "{} {}",
                "Token balance:".bold(),
                b.tokens.to_string().cyan().bold()
            );
        }

        // ── origin ───────────────────────────────────────────────────────────
        Commands::Origin => {
            eprintln!("{}", "Provisioning origin…".dimmed());
            let r = client.create_origin().await?;
            println!("{}", "✓ Origin provisioned".green().bold());
            println!("  {:<14} {}", "ID:".dimmed(), r.id);
            println!("  {:<14} {}", "Origin URL:".dimmed(), r.origin);
        }

        // ── public-key ───────────────────────────────────────────────────────
        Commands::PublicKey(args) => {
            let pem = fs::read_to_string(&args.pem_file)
                .with_context(|| format!("failed to read {}", args.pem_file.display()))?;
            eprintln!("{}", "Uploading public key…".dimmed());
            let r = client.upload_origin_public_key(&args.origin_id, &pem).await?;
            println!("{}", "✓ Public key uploaded".green().bold());
            println!("  {:<14} {}", "URL:".dimmed(), r.url);
        }

        // ── upload ───────────────────────────────────────────────────────────
        Commands::Upload(args) => {
            eprintln!("{} {}…", "Uploading".dimmed(), args.file.display());
            let r = client.upload_temp(&args.file).await?;
            println!("{}", "✓ File uploaded".green().bold());
            println!("  {:<14} {}", "URL:".dimmed(), r.url);
            println!("  {:<14} {}", "Key:".dimmed(), r.key);
            println!("  {:<14} {} bytes", "Size:".dimmed(), r.size);
            println!("  {:<14} {}", "Expires at:".dimmed(), r.expires_at);
        }

        // ── image ────────────────────────────────────────────────────────────
        Commands::Image(args) => {
            let html = read_html(args.source.html, args.source.file)?;
            let mut req = GenerateImageRequest::new(html)
                .format(parse_image_format(&args.format));
            if let Some(w) = args.viewport_width {
                req = req.viewport_width(w);
            }
            if let Some(h) = args.viewport_height {
                req = req.viewport_height(h);
            }
            if let Some(selector) = args.selector {
                req = req.selector(selector);
            }
            if let Some(s) = args.scale {
                req = req.device_scale_factor(s);
            }

            eprintln!("{}", "Generating image…".dimmed());
            let r = client.generate_image(req).await?;

            println!("{}", "✓ Image generated".green().bold());
            println!("  {:<14} {}", "URL:".dimmed(), r.url);
            println!(
                "  {:<14} {} × {} px",
                "Dimensions:".dimmed(),
                r.width,
                r.height
            );
            println!(
                "  {:<14} {}",
                "Format:".dimmed(),
                format!("{:?}", r.format).to_uppercase()
            );
            println!("  {:<14} {} token(s)", "Tokens used:".dimmed(), r.tokens_used);
            println!("  {:<14} {}", "Request ID:".dimmed(), r.request_id);

            if let Some(output) = args.output {
                eprintln!("{} {}…", "Saving to".dimmed(), output.display());
                download_to(&r.url, &output).await?;
                println!("{} {}", "✓ Saved →".green(), output.display());
            }
        }

        // ── pdf ──────────────────────────────────────────────────────────────
        Commands::Pdf(args) => {
            let output = args.output.clone();
            let fmt = args.format.as_deref();

            let request = if let Some(page_files) = args.pages {
                // Multi-page: each file → one page
                let mut pages = Vec::new();
                for path in &page_files {
                    let html = std::fs::read_to_string(path)
                        .with_context(|| format!("failed to read {}", path.display()))?;
                    pages.push(build_page(
                        html,
                        fmt,
                        args.page_size_source.as_deref(),
                        args.landscape,
                        args.print_background,
                        args.margin_top.clone(),
                        args.margin_bottom.clone(),
                        args.margin_left.clone(),
                        args.margin_right.clone(),
                    ));
                }
                GeneratePdfRequest::MultiPage { pages }
            } else {
                let html = read_html(args.html, args.file)?;
                let page = build_page(
                    html,
                    fmt,
                    args.page_size_source.as_deref(),
                    args.landscape,
                    args.print_background,
                    args.margin_top,
                    args.margin_bottom,
                    args.margin_left,
                    args.margin_right,
                );
                GeneratePdfRequest::SinglePage(page)
            };

            eprintln!("{}", "Generating PDF…".dimmed());
            let r = client.generate_pdf(request).await?;

            println!("{}", "✓ PDF generated".green().bold());
            println!("  {:<14} {}", "URL:".dimmed(), r.url);
            println!("  {:<14} {} page(s)", "Pages:".dimmed(), r.pages);
            println!("  {:<14} {} token(s)", "Tokens used:".dimmed(), r.tokens_used);
            println!("  {:<14} {}", "Request ID:".dimmed(), r.request_id);

            if let Some(out) = output {
                eprintln!("{} {}…", "Saving to".dimmed(), out.display());
                download_to(&r.url, &out).await?;
                println!("{} {}", "✓ Saved →".green(), out.display());
            }
        }
    }

    Ok(())
}
