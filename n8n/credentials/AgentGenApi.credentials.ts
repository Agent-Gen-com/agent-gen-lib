import type {
	IAuthenticateGeneric,
	ICredentialTestRequest,
	ICredentialType,
	INodeProperties,
} from 'n8n-workflow';

export class AgentGenApi implements ICredentialType {
	name = 'agentGenApi';
	displayName = 'AgentGen API';
	documentationUrl = 'https://www.agent-gen.com';

	properties: INodeProperties[] = [
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
	authenticate: IAuthenticateGeneric = {
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
	test: ICredentialTestRequest = {
		request: {
			baseURL: 'https://www.agent-gen.com/api',
			url: '/v1/balance',
			method: 'GET',
		},
	};
}
