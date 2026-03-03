"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AgentGen = void 0;
const n8n_workflow_1 = require("n8n-workflow");
const BASE_URL = 'https://www.agent-gen.com/api';
class AgentGen {
    constructor() {
        this.description = {
            displayName: 'AgentGen',
            name: 'agentGen',
            icon: 'file:agentgen.svg',
            group: ['transform'],
            version: 1,
            subtitle: '={{$parameter["operation"]}}',
            description: 'Generate PDFs and images from HTML using the AgentGen API',
            defaults: {
                name: 'AgentGen',
            },
            inputs: ['main'],
            outputs: ['main'],
            credentials: [
                {
                    name: 'agentGenApi',
                    required: true,
                },
            ],
            properties: [
                // ─── Operation selector ─────────────────────────────────────────────────
                {
                    displayName: 'Operation',
                    name: 'operation',
                    type: 'options',
                    noDataExpression: true,
                    options: [
                        {
                            name: 'Generate Image',
                            value: 'generateImage',
                            description: 'Render HTML to an image (PNG / JPEG / WebP). Costs 1 token.',
                            action: 'Generate image from HTML',
                        },
                        {
                            name: 'Generate PDF',
                            value: 'generatePdf',
                            description: 'Render HTML to a PDF (single or multi-page). Costs 2 tokens per page.',
                            action: 'Generate PDF from HTML',
                        },
                        {
                            name: 'Get Balance',
                            value: 'getBalance',
                            description: 'Get the current token balance for your API key',
                            action: 'Get token balance',
                        },
                        {
                            name: 'Upload Temp File',
                            value: 'uploadTempFile',
                            description: 'Upload a file for use inside HTML templates. Free — auto-deleted after 24 hours.',
                            action: 'Upload a temporary file',
                        },
                    ],
                    default: 'generateImage',
                },
                // ─── Generate Image ──────────────────────────────────────────────────────
                {
                    displayName: 'HTML',
                    name: 'html',
                    type: 'string',
                    typeOptions: { rows: 8 },
                    displayOptions: {
                        show: { operation: ['generateImage'] },
                    },
                    default: '<h1 style="font-family:sans-serif;text-align:center">Hello, AgentGen!</h1>',
                    required: true,
                    description: 'HTML content to render (max 500 KB)',
                },
                {
                    displayName: 'Image Options',
                    name: 'imageOptions',
                    type: 'collection',
                    placeholder: 'Add Option',
                    displayOptions: {
                        show: { operation: ['generateImage'] },
                    },
                    default: {},
                    options: [
                        {
                            displayName: 'Device Scale Factor',
                            name: 'device_scale_factor',
                            type: 'number',
                            typeOptions: { minValue: 1, maxValue: 3, numberPrecision: 1 },
                            default: 2,
                            description: 'Device pixel ratio (1–3). Higher produces sharper images.',
                        },
                        {
                            displayName: 'Format',
                            name: 'format',
                            type: 'options',
                            options: [
                                { name: 'PNG', value: 'png' },
                                { name: 'JPEG', value: 'jpeg' },
                                { name: 'WebP', value: 'webp' },
                            ],
                            default: 'png',
                            description: 'Output image format',
                        },
                        {
                            displayName: 'Height',
                            name: 'height',
                            type: 'number',
                            typeOptions: { minValue: 1, maxValue: 5000 },
                            default: 630,
                            description: 'Viewport height in pixels (1–5000)',
                        },
                        {
                            displayName: 'Width',
                            name: 'width',
                            type: 'number',
                            typeOptions: { minValue: 1, maxValue: 5000 },
                            default: 1200,
                            description: 'Viewport width in pixels (1–5000)',
                        },
                    ],
                },
                // ─── Generate PDF ────────────────────────────────────────────────────────
                {
                    displayName: 'PDF Mode',
                    name: 'pdfMode',
                    type: 'options',
                    noDataExpression: true,
                    displayOptions: {
                        show: { operation: ['generatePdf'] },
                    },
                    options: [
                        {
                            name: 'Single Page',
                            value: 'singlePage',
                            description: 'One HTML input → one-page PDF',
                        },
                        {
                            name: 'Multi-Page',
                            value: 'multiPage',
                            description: 'JSON array of page objects → multi-page PDF',
                        },
                    ],
                    default: 'singlePage',
                },
                {
                    displayName: 'HTML',
                    name: 'pdfHtml',
                    type: 'string',
                    typeOptions: { rows: 8 },
                    displayOptions: {
                        show: { operation: ['generatePdf'], pdfMode: ['singlePage'] },
                    },
                    default: '<h1 style="font-family:sans-serif">Hello, AgentGen!</h1>',
                    required: true,
                    description: 'HTML content to render (max 500 KB)',
                },
                {
                    displayName: 'PDF Options',
                    name: 'pdfOptions',
                    type: 'collection',
                    placeholder: 'Add Option',
                    displayOptions: {
                        show: { operation: ['generatePdf'], pdfMode: ['singlePage'] },
                    },
                    default: {},
                    options: [
                        {
                            displayName: 'Format',
                            name: 'format',
                            type: 'options',
                            options: [
                                { name: 'A4', value: 'A4' },
                                { name: 'A3', value: 'A3' },
                                { name: 'Legal', value: 'Legal' },
                                { name: 'Letter', value: 'Letter' },
                            ],
                            default: 'A4',
                            description: 'Paper size',
                        },
                        {
                            displayName: 'Landscape',
                            name: 'landscape',
                            type: 'boolean',
                            default: false,
                            description: 'Whether to render in landscape orientation',
                        },
                        {
                            displayName: 'Margin Bottom',
                            name: 'margin_bottom',
                            type: 'string',
                            default: '',
                            placeholder: '20mm',
                            description: 'Bottom page margin as a CSS length (e.g. "20mm", "1in")',
                        },
                        {
                            displayName: 'Margin Left',
                            name: 'margin_left',
                            type: 'string',
                            default: '',
                            placeholder: '20mm',
                            description: 'Left page margin as a CSS length (e.g. "20mm", "1in")',
                        },
                        {
                            displayName: 'Margin Right',
                            name: 'margin_right',
                            type: 'string',
                            default: '',
                            placeholder: '20mm',
                            description: 'Right page margin as a CSS length (e.g. "20mm", "1in")',
                        },
                        {
                            displayName: 'Margin Top',
                            name: 'margin_top',
                            type: 'string',
                            default: '',
                            placeholder: '20mm',
                            description: 'Top page margin as a CSS length (e.g. "20mm", "1in")',
                        },
                        {
                            displayName: 'Print Background',
                            name: 'print_background',
                            type: 'boolean',
                            default: true,
                            description: 'Whether to render CSS backgrounds',
                        },
                    ],
                },
                {
                    displayName: 'Pages (JSON)',
                    name: 'pdfPages',
                    type: 'json',
                    displayOptions: {
                        show: { operation: ['generatePdf'], pdfMode: ['multiPage'] },
                    },
                    default: '[\n  { "html": "<h1>Page 1</h1>", "format": "A4" },\n  { "html": "<h1>Page 2</h1>", "format": "A4" }\n]',
                    required: true,
                    description: 'JSON array of page objects. Each object accepts: html (required), format, width, height, landscape, print_background, margin ({ top, bottom, left, right }).',
                },
                // ─── Upload Temp File ────────────────────────────────────────────────────
                {
                    displayName: 'Input Binary Field',
                    name: 'binaryPropertyName',
                    type: 'string',
                    displayOptions: {
                        show: { operation: ['uploadTempFile'] },
                    },
                    default: 'data',
                    required: true,
                    description: 'Name of the binary property on the incoming item that contains the file to upload',
                    hint: 'This is the binary field name from a previous node (e.g. "data" from an HTTP Request or Read Binary File node)',
                },
                {
                    displayName: 'Filename',
                    name: 'filename',
                    type: 'string',
                    displayOptions: {
                        show: { operation: ['uploadTempFile'] },
                    },
                    default: '',
                    placeholder: 'image.png',
                    description: 'Override the filename sent to the API. Defaults to the filename from the binary data.',
                },
            ],
        };
    }
    async execute() {
        const items = this.getInputData();
        const returnData = [];
        for (let i = 0; i < items.length; i++) {
            const operation = this.getNodeParameter('operation', i);
            try {
                let responseData;
                // ── Generate Image ────────────────────────────────────────────────────
                if (operation === 'generateImage') {
                    const html = this.getNodeParameter('html', i);
                    const imageOptions = this.getNodeParameter('imageOptions', i);
                    const body = { html };
                    if (imageOptions.width !== undefined)
                        body.width = imageOptions.width;
                    if (imageOptions.height !== undefined)
                        body.height = imageOptions.height;
                    if (imageOptions.format)
                        body.format = imageOptions.format;
                    if (imageOptions.device_scale_factor !== undefined)
                        body.device_scale_factor = imageOptions.device_scale_factor;
                    const options = {
                        method: 'POST',
                        url: `${BASE_URL}/v1/generate/image`,
                        body,
                        json: true,
                    };
                    responseData = (await this.helpers.httpRequestWithAuthentication.call(this, 'agentGenApi', options));
                }
                // ── Generate PDF ──────────────────────────────────────────────────────
                else if (operation === 'generatePdf') {
                    const pdfMode = this.getNodeParameter('pdfMode', i);
                    let body;
                    if (pdfMode === 'singlePage') {
                        const html = this.getNodeParameter('pdfHtml', i);
                        const pdfOptions = this.getNodeParameter('pdfOptions', i);
                        body = { html };
                        if (pdfOptions.format)
                            body.format = pdfOptions.format;
                        if (pdfOptions.landscape !== undefined)
                            body.landscape = pdfOptions.landscape;
                        if (pdfOptions.print_background !== undefined)
                            body.print_background = pdfOptions.print_background;
                        // Build margin object only if at least one margin is set
                        const margin = {};
                        if (pdfOptions.margin_top)
                            margin.top = pdfOptions.margin_top;
                        if (pdfOptions.margin_bottom)
                            margin.bottom = pdfOptions.margin_bottom;
                        if (pdfOptions.margin_left)
                            margin.left = pdfOptions.margin_left;
                        if (pdfOptions.margin_right)
                            margin.right = pdfOptions.margin_right;
                        if (Object.keys(margin).length > 0)
                            body.margin = margin;
                    }
                    else {
                        // Multi-page
                        const pagesJson = this.getNodeParameter('pdfPages', i);
                        let pages;
                        try {
                            pages = (typeof pagesJson === 'string'
                                ? JSON.parse(pagesJson)
                                : pagesJson);
                        }
                        catch {
                            throw new n8n_workflow_1.NodeOperationError(this.getNode(), 'Pages must be a valid JSON array. Each element must have at least an "html" property.', { itemIndex: i });
                        }
                        if (!Array.isArray(pages) || pages.length === 0) {
                            throw new n8n_workflow_1.NodeOperationError(this.getNode(), 'Pages must be a non-empty JSON array.', { itemIndex: i });
                        }
                        body = { pages };
                    }
                    const options = {
                        method: 'POST',
                        url: `${BASE_URL}/v1/generate/pdf`,
                        body,
                        json: true,
                    };
                    responseData = (await this.helpers.httpRequestWithAuthentication.call(this, 'agentGenApi', options));
                }
                // ── Upload Temp File ──────────────────────────────────────────────────
                else if (operation === 'uploadTempFile') {
                    const binaryPropertyName = this.getNodeParameter('binaryPropertyName', i);
                    const filenameOverride = this.getNodeParameter('filename', i);
                    const binaryData = this.helpers.assertBinaryData(i, binaryPropertyName);
                    const binaryDataBuffer = await this.helpers.getBinaryDataBuffer(i, binaryPropertyName);
                    const filename = filenameOverride || binaryData.fileName || 'upload';
                    const options = {
                        method: 'POST',
                        url: `${BASE_URL}/v1/upload/temp`,
                        body: {
                            file: {
                                value: binaryDataBuffer,
                                options: {
                                    filename,
                                    contentType: binaryData.mimeType,
                                },
                            },
                        },
                        json: true,
                    };
                    responseData = (await this.helpers.httpRequestWithAuthentication.call(this, 'agentGenApi', options));
                }
                // ── Get Balance ───────────────────────────────────────────────────────
                else if (operation === 'getBalance') {
                    const options = {
                        method: 'GET',
                        url: `${BASE_URL}/v1/balance`,
                        json: true,
                    };
                    responseData = (await this.helpers.httpRequestWithAuthentication.call(this, 'agentGenApi', options));
                }
                else {
                    throw new n8n_workflow_1.NodeOperationError(this.getNode(), `Unknown operation: ${operation}`, {
                        itemIndex: i,
                    });
                }
                returnData.push({
                    json: responseData,
                    pairedItem: { item: i },
                });
            }
            catch (error) {
                if (this.continueOnFail()) {
                    returnData.push({
                        json: { error: error.message },
                        pairedItem: { item: i },
                    });
                    continue;
                }
                throw error;
            }
        }
        return [returnData];
    }
}
exports.AgentGen = AgentGen;
//# sourceMappingURL=AgentGen.node.js.map