"""AgentGen Python SDK — HTML to PDF and Image.

Quick start::

    from agentgen import AgentGenClient, GenerateImageOptions

    client = AgentGenClient(api_key="agk_...")

    # Generate an image
    img = client.generate_image(GenerateImageOptions(html="<h1>Hello</h1>"))
    print(img.url)

    # Generate a PDF
    from agentgen import PdfPage
    pdf = client.generate_pdf(PdfPage(html="<h1>Invoice</h1>", format="A4"))
    print(pdf.url)
"""

from .client import AgentGenClient, AsyncAgentGenClient
from .errors import AgentGenError, InsufficientTokensError
from .types import (
    BalanceResult,
    CreateOriginResult,
    GenerateImageOptions,
    GenerateImageResult,
    GeneratePdfOptions,
    GeneratePdfResult,
    ImageFormat,
    PdfFormat,
    PdfMargin,
    PdfPage,
    UploadOriginPublicKeyResult,
    UploadTempResult,
)

__all__ = [
    # Clients
    "AgentGenClient",
    "AsyncAgentGenClient",
    # Request types
    "GenerateImageOptions",
    "PdfMargin",
    "PdfPage",
    "GeneratePdfOptions",
    # Response types
    "GenerateImageResult",
    "GeneratePdfResult",
    "UploadTempResult",
    "BalanceResult",
    "CreateOriginResult",
    "UploadOriginPublicKeyResult",
    # Literals
    "ImageFormat",
    "PdfFormat",
    # Errors
    "AgentGenError",
    "InsufficientTokensError",
]
