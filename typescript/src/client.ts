import type {
  BalanceResult,
  GenerateImageOptions,
  GenerateImageResult,
  GeneratePdfOptions,
  GeneratePdfResult,
  UploadTempResult,
} from './types.js';
import { AgentGenError, InsufficientTokensError } from './errors.js';

const DEFAULT_BASE_URL = 'https://www.agent-gen.com/api';

export interface AgentGenClientOptions {
  /** Your AgentGen API key (get one at https://www.agent-gen.com). */
  apiKey: string;
  /** Override the API base URL (optional, defaults to https://www.agent-gen.com/api). */
  baseUrl?: string;
}

export class AgentGenClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;

  constructor(options: AgentGenClientOptions) {
    this.apiKey = options.apiKey;
    this.baseUrl = (options.baseUrl ?? DEFAULT_BASE_URL).replace(/\/$/, '');
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
  ): Promise<T> {
    const isFormData = body instanceof FormData;
    const headers: Record<string, string> = { 'X-API-Key': this.apiKey };
    if (!isFormData) headers['Content-Type'] = 'application/json';

    const res = await fetch(`${this.baseUrl}${path}`, {
      method,
      headers,
      body: isFormData
        ? body
        : body !== undefined
        ? JSON.stringify(body)
        : undefined,
    });

    const data = (await res.json()) as Record<string, unknown>;

    if (!res.ok) {
      if (res.status === 402) {
        throw new InsufficientTokensError(
          data['error'] as string,
          data['balance'] as number,
          data['required'] as number,
          data['buy_more_url'] as string,
        );
      }
      throw new AgentGenError(
        data['error'] as string,
        res.status,
        data['detail'] as string | undefined,
        data['details'] as Record<string, unknown> | undefined,
      );
    }

    return data as T;
  }

  /**
   * Render HTML to an image (PNG / JPEG / WebP).
   * Costs **1 token**.
   */
  generateImage(options: GenerateImageOptions): Promise<GenerateImageResult> {
    return this.request('POST', '/v1/generate/image', options);
  }

  /**
   * Render HTML to a PDF — single page or multi-page.
   * Costs **2 tokens per page**.
   *
   * @example Single page
   * ```ts
   * await client.generatePdf({ html: '<h1>Invoice</h1>', format: 'A4' });
   * ```
   * @example Multi-page
   * ```ts
   * await client.generatePdf({ pages: [{ html: '<h1>Page 1</h1>' }, { html: '<h1>Page 2</h1>' }] });
   * ```
   */
  generatePdf(options: GeneratePdfOptions): Promise<GeneratePdfResult> {
    return this.request('POST', '/v1/generate/pdf', options);
  }

  /**
   * Upload a file to temporary storage for use inside HTML templates.
   * **Free** (no tokens consumed). Auto-deleted after **24 hours**.
   *
   * @param file A `Blob`, `File`, or `BufferSource` to upload.
   * @param filename Optional filename hint (used for Content-Disposition).
   */
  async uploadTemp(
    file: Blob | File | BufferSource,
    filename?: string,
  ): Promise<UploadTempResult> {
    const blob = file instanceof Blob ? file : new Blob([file as ArrayBuffer]);
    const form = new FormData();
    form.append('file', blob, filename ?? 'upload');
    return this.request('POST', '/v1/upload/temp', form);
  }

  /**
   * Get the current token balance for the API key owner.
   */
  getBalance(): Promise<BalanceResult> {
    return this.request('GET', '/v1/balance');
  }
}
