from __future__ import annotations

import dataclasses
from typing import Literal, Optional

ImageFormat = Literal["png", "jpeg", "webp"]
PdfFormat = Literal["A4", "Letter", "A3", "Legal"]


@dataclasses.dataclass
class GenerateImageOptions:
    """Options for HTML → image generation."""

    html: str
    """HTML content to render (max 500 KB)."""
    viewport_width: Optional[int] = None
    """Viewport width in px used for layout before capture (1–5000, default 1200)."""
    viewport_height: Optional[int] = None
    """Viewport height in px used for layout before capture (1–5000, default 800)."""
    selector: Optional[str] = None
    """Optional CSS selector to capture instead of the full rendered document."""
    format: Optional[ImageFormat] = None
    """Output format: "png" | "jpeg" | "webp" (default "png")."""
    device_scale_factor: Optional[float] = None
    """Device pixel ratio (1–3, default 2)."""


@dataclasses.dataclass
class GenerateImageResult:
    url: str
    width: int
    height: int
    format: ImageFormat
    tokens_used: int
    request_id: str


@dataclasses.dataclass
class PdfMargin:
    top: Optional[str] = None
    bottom: Optional[str] = None
    left: Optional[str] = None
    right: Optional[str] = None


@dataclasses.dataclass
class PdfPage:
    """A single PDF page."""

    html: str
    """HTML content to render (max 500 KB)."""
    page_size_source: Optional[Literal["css", "format"]] = None
    """Prefer CSS @page size or the fallback format (default "css")."""
    format: Optional[PdfFormat] = None
    """Fallback paper size: "A4" | "Letter" | "A3" | "Legal" (default "A4")."""
    landscape: Optional[bool] = None
    """Landscape orientation (default False)."""
    margin: Optional[PdfMargin] = None
    """Page margins."""
    print_background: Optional[bool] = None
    """Print CSS backgrounds (default True)."""


@dataclasses.dataclass
class GeneratePdfOptions:
    """Top-level PDF request options."""

    pages: list[PdfPage]
    """Pages to render as one PDF."""
    optimize: Optional[bool] = None
    """Shrink the final PDF with a post-processing pass (default True)."""


@dataclasses.dataclass
class GeneratePdfResult:
    url: str
    pages: int
    tokens_used: int
    request_id: str


@dataclasses.dataclass
class UploadTempResult:
    url: str
    key: str
    size: int
    expires_in: int
    expires_at: str


@dataclasses.dataclass
class BalanceResult:
    tokens: int


@dataclasses.dataclass
class CreateOriginResult:
    id: str
    """Unique origin ID, used in the subdomain."""
    origin: str
    """Full origin URL, e.g. https://abc123xyz.agent-gen.com."""


@dataclasses.dataclass
class UploadOriginPublicKeyResult:
    url: str
    """Public URL where the PEM key is now accessible."""
