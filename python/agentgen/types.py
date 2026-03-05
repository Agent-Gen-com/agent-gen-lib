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
    width: Optional[int] = None
    """Viewport width in px (1–5000, default 1200)."""
    height: Optional[int] = None
    """Viewport height in px (1–5000, default 630)."""
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
    format: Optional[PdfFormat] = None
    """Paper size: "A4" | "Letter" | "A3" | "Legal" (default "A4")."""
    width: Optional[str] = None
    """Custom page width, e.g. "8.5in" (overrides format)."""
    height: Optional[str] = None
    """Custom page height, e.g. "11in"."""
    landscape: Optional[bool] = None
    """Landscape orientation (default False)."""
    margin: Optional[PdfMargin] = None
    """Page margins."""
    print_background: Optional[bool] = None
    """Print CSS backgrounds (default True)."""


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
