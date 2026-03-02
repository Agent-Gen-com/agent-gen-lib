from __future__ import annotations

import asyncio
import dataclasses
from pathlib import Path
from typing import Any, BinaryIO, Optional, Union

import httpx

from .errors import AgentGenError, InsufficientTokensError
from .types import (
    BalanceResult,
    GenerateImageOptions,
    GenerateImageResult,
    GeneratePdfResult,
    PdfPage,
    UploadTempResult,
)

DEFAULT_BASE_URL = "https://www.agent-gen.com/api"


def _to_dict(obj: Any) -> Any:
    """Recursively convert a dataclass to a dict, dropping None values."""
    if dataclasses.is_dataclass(obj) and not isinstance(obj, type):
        return {
            f.name: _to_dict(v)
            for f in dataclasses.fields(obj)  # type: ignore[arg-type]
            if (v := getattr(obj, f.name)) is not None
        }
    return obj


def _raise_for_error(response: httpx.Response) -> None:
    data: dict[str, Any] = response.json()
    if response.status_code == 402:
        raise InsufficientTokensError(
            data["error"],
            data["balance"],
            data["required"],
            data["buy_more_url"],
        )
    raise AgentGenError(
        data["error"],
        response.status_code,
        data.get("detail"),
        data.get("details"),
    )


class AgentGenClient:
    """Synchronous AgentGen API client.

    Example::

        from agentgen import AgentGenClient, GenerateImageOptions

        client = AgentGenClient(api_key="agk_...")
        result = client.generate_image(GenerateImageOptions(html="<h1>Hello</h1>"))
        print(result.url)
    """

    def __init__(self, api_key: str, base_url: str = DEFAULT_BASE_URL) -> None:
        self._base_url = base_url.rstrip("/")
        self._headers = {"X-API-Key": api_key}

    def generate_image(self, options: GenerateImageOptions) -> GenerateImageResult:
        """Render HTML to an image (PNG/JPEG/WebP). Costs **1 token**."""
        with httpx.Client() as client:
            r = client.post(
                f"{self._base_url}/v1/generate/image",
                json=_to_dict(options),
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return GenerateImageResult(**r.json())

    def generate_pdf(
        self, options: Union[PdfPage, list[PdfPage]]
    ) -> GeneratePdfResult:
        """Render HTML to a PDF. Costs **2 tokens per page**.

        Pass a single :class:`PdfPage` for a one-page document, or a list of
        :class:`PdfPage` objects for a multi-page document.
        """
        body = (
            {"pages": [_to_dict(p) for p in options]}
            if isinstance(options, list)
            else _to_dict(options)
        )
        with httpx.Client() as client:
            r = client.post(
                f"{self._base_url}/v1/generate/pdf",
                json=body,
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return GeneratePdfResult(**r.json())

    def upload_temp(
        self,
        file: Union[str, Path, BinaryIO],
        filename: Optional[str] = None,
    ) -> UploadTempResult:
        """Upload a file for use inside HTML templates.

        **Free** (no tokens). Auto-deleted after **24 hours**.

        Args:
            file: A file path (str/Path) or an open binary file object.
            filename: Optional filename hint for the upload.
        """
        if isinstance(file, (str, Path)):
            path = Path(file)
            filename = filename or path.name
            file_obj: BinaryIO = open(path, "rb")
            should_close = True
        else:
            file_obj = file
            should_close = False

        try:
            with httpx.Client() as client:
                r = client.post(
                    f"{self._base_url}/v1/upload/temp",
                    files={"file": (filename or "upload", file_obj)},
                    headers=self._headers,
                )
        finally:
            if should_close:
                file_obj.close()  # type: ignore[union-attr]

        if not r.is_success:
            _raise_for_error(r)
        return UploadTempResult(**r.json())

    def get_balance(self) -> BalanceResult:
        """Return the current token balance for the API key owner."""
        with httpx.Client() as client:
            r = client.get(
                f"{self._base_url}/v1/balance",
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return BalanceResult(**r.json())


class AsyncAgentGenClient:
    """Async AgentGen API client (use with ``async``/``await``).

    Example::

        from agentgen import AsyncAgentGenClient, GenerateImageOptions

        async def main():
            client = AsyncAgentGenClient(api_key="agk_...")
            result = await client.generate_image(GenerateImageOptions(html="<h1>Hello</h1>"))
            print(result.url)
    """

    def __init__(self, api_key: str, base_url: str = DEFAULT_BASE_URL) -> None:
        self._base_url = base_url.rstrip("/")
        self._headers = {"X-API-Key": api_key}

    async def generate_image(
        self, options: GenerateImageOptions
    ) -> GenerateImageResult:
        """Render HTML to an image (PNG/JPEG/WebP). Costs **1 token**."""
        async with httpx.AsyncClient() as client:
            r = await client.post(
                f"{self._base_url}/v1/generate/image",
                json=_to_dict(options),
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return GenerateImageResult(**r.json())

    async def generate_pdf(
        self, options: Union[PdfPage, list[PdfPage]]
    ) -> GeneratePdfResult:
        """Render HTML to a PDF. Costs **2 tokens per page**.

        Pass a single :class:`PdfPage` or a list for multi-page output.
        """
        body = (
            {"pages": [_to_dict(p) for p in options]}
            if isinstance(options, list)
            else _to_dict(options)
        )
        async with httpx.AsyncClient() as client:
            r = await client.post(
                f"{self._base_url}/v1/generate/pdf",
                json=body,
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return GeneratePdfResult(**r.json())

    async def upload_temp(
        self,
        file: Union[str, Path, bytes],
        filename: Optional[str] = None,
    ) -> UploadTempResult:
        """Upload a file for use inside HTML templates.

        **Free** (no tokens). Auto-deleted after **24 hours**.

        Args:
            file: A file path (str/Path) or raw bytes.
            filename: Optional filename hint for the upload.
        """
        if isinstance(file, (str, Path)):
            path = Path(file)
            filename = filename or path.name
            loop = asyncio.get_event_loop()
            content: bytes = await loop.run_in_executor(None, path.read_bytes)
        else:
            content = file

        async with httpx.AsyncClient() as client:
            r = await client.post(
                f"{self._base_url}/v1/upload/temp",
                files={"file": (filename or "upload", content)},
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return UploadTempResult(**r.json())

    async def get_balance(self) -> BalanceResult:
        """Return the current token balance for the API key owner."""
        async with httpx.AsyncClient() as client:
            r = await client.get(
                f"{self._base_url}/v1/balance",
                headers=self._headers,
            )
        if not r.is_success:
            _raise_for_error(r)
        return BalanceResult(**r.json())
