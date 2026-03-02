export type ImageFormat = 'png' | 'jpeg' | 'webp';
export type PdfFormat = 'A4' | 'Letter' | 'A3' | 'Legal';

export interface GenerateImageOptions {
  /** HTML content to render (max 500 KB). */
  html: string;
  /** Viewport width in px (1–5000, default 1200). */
  width?: number;
  /** Viewport height in px (1–5000, default 630). */
  height?: number;
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
  /** Paper format (default "A4"). */
  format?: PdfFormat;
  /** Custom page width, e.g. "8.5in" (overrides format). */
  width?: string;
  /** Custom page height, e.g. "11in" (overrides format). */
  height?: string;
  /** Landscape orientation (default false). */
  landscape?: boolean;
  /** Page margins. */
  margin?: PdfMargin;
  /** Print CSS backgrounds (default true). */
  print_background?: boolean;
}

/** Single-page or multi-page PDF input. */
export type GeneratePdfOptions = PdfPage | { pages: PdfPage[] };

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
