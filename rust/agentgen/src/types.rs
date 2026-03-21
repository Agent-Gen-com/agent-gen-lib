use serde::{Deserialize, Serialize};

// ── Image ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
}

/// Request body for `POST /v1/generate/image`.
///
/// Build with [`GenerateImageRequest::new`] and chain optional setters:
/// ```
/// use agentgen::types::{GenerateImageRequest, ImageFormat};
///
/// let req = GenerateImageRequest::new("<h1>Hello</h1>")
///     .viewport_width(1200)
///     .viewport_height(800)
///     .format(ImageFormat::Png)
///     .device_scale_factor(2.0);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GenerateImageRequest {
    pub html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_scale_factor: Option<f64>,
}

impl GenerateImageRequest {
    pub fn new(html: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            viewport_width: None,
            viewport_height: None,
            selector: None,
            format: None,
            device_scale_factor: None,
        }
    }

    pub fn viewport_width(mut self, width: u32) -> Self {
        self.viewport_width = Some(width);
        self
    }

    pub fn viewport_height(mut self, height: u32) -> Self {
        self.viewport_height = Some(height);
        self
    }

    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    pub fn format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn device_scale_factor(mut self, factor: f64) -> Self {
        self.device_scale_factor = Some(factor);
        self
    }
}

/// Response from `POST /v1/generate/image`.
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateImageResponse {
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub tokens_used: u32,
    pub request_id: String,
}

// ── PDF ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PdfFormat {
    A4,
    Letter,
    A3,
    Legal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PdfPageSizeSource {
    Css,
    Format,
}

/// Page margins for a PDF page.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PdfMargin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}

impl PdfMargin {
    pub fn all(value: impl Into<String>) -> Self {
        let v = value.into();
        Self {
            top: Some(v.clone()),
            bottom: Some(v.clone()),
            left: Some(v.clone()),
            right: Some(v),
        }
    }
}

/// A single page in a PDF request.
///
/// Build with [`PdfPage::new`]:
/// ```
/// use agentgen::types::{PdfPage, PdfFormat, PdfMargin, PdfPageSizeSource};
///
/// let page = PdfPage::new("<h1>Invoice</h1>")
///     .page_size_source(PdfPageSizeSource::Css)
///     .format(PdfFormat::A4)
///     .print_background(true)
///     .margin(PdfMargin::all("20mm"));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PdfPage {
    pub html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size_source: Option<PdfPageSizeSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PdfFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<PdfMargin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_background: Option<bool>,
}

impl PdfPage {
    pub fn new(html: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            page_size_source: None,
            format: None,
            landscape: None,
            margin: None,
            print_background: None,
        }
    }

    pub fn page_size_source(mut self, page_size_source: PdfPageSizeSource) -> Self {
        self.page_size_source = Some(page_size_source);
        self
    }

    pub fn format(mut self, format: PdfFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn landscape(mut self, landscape: bool) -> Self {
        self.landscape = Some(landscape);
        self
    }

    pub fn margin(mut self, margin: PdfMargin) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn print_background(mut self, print_background: bool) -> Self {
        self.print_background = Some(print_background);
        self
    }
}

/// Request body for `POST /v1/generate/pdf` — single page or multi-page.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GeneratePdfRequest {
    SinglePage(PdfPage),
    MultiPage { pages: Vec<PdfPage> },
}

/// Response from `POST /v1/generate/pdf`.
#[derive(Debug, Clone, Deserialize)]
pub struct GeneratePdfResponse {
    pub url: String,
    pub pages: u32,
    pub tokens_used: u32,
    pub request_id: String,
}

// ── Upload ───────────────────────────────────────────────────────────────────

/// Response from `POST /v1/upload/temp`.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadTempResponse {
    pub url: String,
    pub key: String,
    pub size: u64,
    pub expires_in: u32,
    pub expires_at: String,
}

// ── Balance ──────────────────────────────────────────────────────────────────

/// Response from `GET /v1/balance`.
#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    pub tokens: i64,
}

// ── Origin ───────────────────────────────────────────────────────────────────

/// Response from `POST /v1/origin`.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOriginResponse {
    /// Unique origin ID, used in the subdomain.
    pub id: String,
    /// Full origin URL, e.g. `https://abc123xyz.agent-gen.com`.
    pub origin: String,
}

/// Response from `POST /v1/origin/{id}/public-key`.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadOriginPublicKeyResponse {
    /// Public URL where the PEM key is now accessible.
    pub url: String,
}

// ── Compress Image ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressImageFormat {
    Jpeg,
    Png,
    Webp,
    Avif,
    Tiff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressImageMode {
    Lossless,
    Balanced,
    Aggressive,
}

/// Response from `POST /v1/compress/image`.
#[derive(Debug, Clone, Deserialize)]
pub struct CompressImageResponse {
    pub url: String,
    pub format: String,
    pub mode: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub savings_percent: f64,
    pub width: u32,
    pub height: u32,
    pub tokens_used: u32,
}
