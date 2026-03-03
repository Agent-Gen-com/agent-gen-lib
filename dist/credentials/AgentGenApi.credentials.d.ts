import type { IAuthenticateGeneric, ICredentialTestRequest, ICredentialType, INodeProperties } from 'n8n-workflow';
export declare class AgentGenApi implements ICredentialType {
    name: string;
    displayName: string;
    documentationUrl: string;
    properties: INodeProperties[];
    /**
     * Inject the API key as an X-API-Key header on every request
     * that uses this credential type.
     */
    authenticate: IAuthenticateGeneric;
    /**
     * Validate the credential by hitting the balance endpoint.
     * n8n will call this when the user clicks "Test credential".
     */
    test: ICredentialTestRequest;
}
//# sourceMappingURL=AgentGenApi.credentials.d.ts.map