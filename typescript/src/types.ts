export type ImageFormat = 'png' | 'jpeg' | 'webp';
export type PdfFormat = 'A4' | 'Letter' | 'A3' | 'Legal';

export interface GenerateImageOptions {
  /** HTML content to render (max 500 KB). */
  html: string;
  /** Viewport width in px used for layout before capture (1–5000, default 1200). */
  viewport_width?: number;
  /** Viewport height in px used for layout before capture (1–5000, default 800). */
  viewport_height?: number;
  /** Optional CSS selector to capture instead of the full rendered document. */
  selector?: string;
  /** Output image format (default "png"). */
  format?: ImageFormat;
  /** Device pixel ratio (1–3, default 2). */
  device_scale_factor?: number;
}

export interface GenerateImageResult {
  url: string;
  width: number;
  height: number;
  format: ImageFormat;
  tokens_used: 1;
  request_id: string;
}

export interface PdfMargin {
  top?: string;
  bottom?: string;
  left?: string;
  right?: string;
}

export interface PdfPage {
  /** HTML content to render (max 500 KB). */
  html: string;
  /** Whether to prefer CSS @page size or fallback format (default "css"). */
  page_size_source?: 'css' | 'format';
  /** Fallback paper format when CSS does not define one (default "A4"). */
  format?: PdfFormat;
  /** Landscape orientation (default false). */
  landscape?: boolean;
  /** Page margins. */
  margin?: PdfMargin;
  /** Print CSS backgrounds (default true). */
  print_background?: boolean;
}

export interface GeneratePdfSinglePageOptions extends PdfPage {
  /** Run a post-processing pass to shrink the final PDF (default true). */
  optimize?: boolean;
}

export interface GeneratePdfMultiPageOptions {
  pages: PdfPage[];
  /** Run a post-processing pass to shrink the final PDF (default true). */
  optimize?: boolean;
}

/** Single-page or multi-page PDF input. */
export type GeneratePdfOptions = GeneratePdfSinglePageOptions | GeneratePdfMultiPageOptions;

export interface GeneratePdfResult {
  url: string;
  /** Number of pages in the generated PDF. */
  pages: number;
  /** Tokens consumed (2 per page). */
  tokens_used: number;
  request_id: string;
}

export interface UploadTempResult {
  url: string;
  key: string;
  size: number;
  /** Always 86400 (24 hours). */
  expires_in: 86400;
  expires_at: string;
}

export interface BalanceResult {
  tokens: number;
}

export interface CreateOriginResult {
  /** Unique origin ID, used in the subdomain. */
  id: string;
  /** Full origin URL, e.g. `https://abc123xyz.agent-gen.com`. */
  origin: string;
}

export interface UploadOriginPublicKeyResult {
  /** Public URL where the PEM key is now accessible. */
  url: string;
}
