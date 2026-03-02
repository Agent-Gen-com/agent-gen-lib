from __future__ import annotations

from typing import Any, Optional


class AgentGenError(Exception):
    """Raised when the AgentGen API returns an error response."""

    def __init__(
        self,
        message: str,
        status: int,
        detail: Optional[str] = None,
        details: Optional[dict[str, Any]] = None,
    ) -> None:
        super().__init__(message)
        self.status = status
        self.detail = detail
        self.details = details

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(status={self.status}, message={str(self)!r})"


class InsufficientTokensError(AgentGenError):
    """Raised when the account has too few tokens for the requested operation."""

    def __init__(
        self,
        message: str,
        balance: int,
        required: int,
        buy_more_url: str,
    ) -> None:
        super().__init__(message, 402)
        self.balance = balance
        self.required = required
        self.buy_more_url = buy_more_url
