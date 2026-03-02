export class AgentGenError extends Error {
  readonly status: number;
  readonly detail?: string;
  readonly details?: Record<string, unknown>;

  constructor(
    message: string,
    status: number,
    detail?: string,
    details?: Record<string, unknown>,
  ) {
    super(message);
    this.name = 'AgentGenError';
    this.status = status;
    this.detail = detail;
    this.details = details;
  }
}

export class InsufficientTokensError extends AgentGenError {
  readonly balance: number;
  readonly required: number;
  readonly buyMoreUrl: string;

  constructor(
    message: string,
    balance: number,
    required: number,
    buyMoreUrl: string,
  ) {
    super(message, 402);
    this.name = 'InsufficientTokensError';
    this.balance = balance;
    this.required = required;
    this.buyMoreUrl = buyMoreUrl;
  }
}
