"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AgentGenApi = void 0;
class AgentGenApi {
    constructor() {
        this.name = 'agentGenApi';
        this.displayName = 'AgentGen API';
        this.documentationUrl = 'https://www.agent-gen.com';
        this.properties = [
            {
                displayName: 'API Key',
                name: 'apiKey',
                type: 'string',
                typeOptions: { password: true },
                default: '',
                required: true,
                placeholder: 'ag_...',
                description: 'Your AgentGen API key. Get one at agent-gen.com.',
            },
        ];
        /**
         * Inject the API key as an X-API-Key header on every request
         * that uses this credential type.
         */
        this.authenticate = {
            type: 'generic',
            properties: {
                headers: {
                    'X-API-Key': '={{$credentials.apiKey}}',
                },
            },
        };
        /**
         * Validate the credential by hitting the balance endpoint.
         * n8n will call this when the user clicks "Test credential".
         */
        this.test = {
            request: {
                baseURL: 'https://www.agent-gen.com/api',
                url: '/v1/balance',
                method: 'GET',
            },
        };
    }
}
exports.AgentGenApi = AgentGenApi;
//# sourceMappingURL=AgentGenApi.credentials.js.map