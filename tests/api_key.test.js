const debug = require('debug')('cdbs-back:api_key.test.js');
const request = require('supertest');
const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

const keyOwner = "key_owner";
const username = "apikey_user_test";
const password = "StrPassword#123";
const addUsername = "additional_user_test";
const addPassword = "TestPass123!";
const tempUsername = "temp_user_delete";

let authToken = "";
let addAuthToken = "";
let userUuid = "";
let addUserUuid = "";
let createdKeyId = 0;
let createdRawKey = "";

const apiKeysQuery = `
  query {
    apiKeys {
      id
      name
      isActive
      createdAt
    }
  }
`;

const apiKeyQuery = `
  query GetApiKey($id: Int!) {
    apiKey(keyId: $id) {
      id
      name
      isActive
      createdAt
      expiresAt
      lastUsedAt
    }
  }
`;

const createApiKeyMutation = `
  mutation CreateApiKey($name: String!, $expiresAt: DateTime) {
    createApiKey(name: $name, expiresAt: $expiresAt)
  }
`;

const updateApiKeyMutation = `
  mutation UpdateApiKey($keyId: Int!, $name: String, $isActive: Boolean) {
    updateApiKey(keyId: $keyId, args: { name: $name, isActive: $isActive })
  }
`;

const deleteApiKeyMutation = `
  mutation DeleteApiKey($keyId: Int!) {
    deleteApiKey(keyId: $keyId)
  }
`;

const rotateApiKeyMutation = `
  mutation RotateApiKey($keyId: Int!) {
    rotateApiKey(keyId: $keyId)
  }
`;

const updateApiKeyExpiryMutation = `
  mutation UpdateApiKeyExpiry($keyId: Int!, $expiresAt: DateTime!) {
    updateApiKey(keyId: $keyId, args: { expiresAt: $expiresAt })
  }
`;

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username in (?, ?, ?, ?)', [
    username,
    keyOwner,
    addUsername,
    tempUsername
  ]);
}

async function cleanupApiKeyDb() {
  return global.knex.raw('DELETE FROM user_api_key_ref');
}

describe('API Keys', () => {
  const agent = request.agent(url);

  beforeAll(async () => {
    await cleanupApiKeyDb();
    await cleanupUserDb();
  });

  afterAll(async () => {
    await cleanupApiKeyDb();
    await cleanupUserDb();
  });

  describe('Main API Key Tests', () => {
    it('should register test user', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          mutation RegisterUser($username: String!, $password: String!) {
            registerUser(args: {
              email: "apikey@test.com"
              username: $username
              password: $password
              firstname: "API"
              lastname: "Key"
              regionId: 1
              programId: 1
            }) {
              uuid
              username
            }
          }
        `,
        variables: { username, password }
      });
      expect(body.errors).toBeUndefined();
      userUuid = body.data.registerUser.uuid;
    });

    it('should login', async () => {
      const { body } = await agent.post('/login').send({
        user: { username, password }
      });
      authToken = body.bearer;
      expect(authToken).toBeNonEmptyString();
    });

    it('should NOT create API key without auth', async () => {
      const { body } = await agent.post('/graphql').send({
        query: createApiKeyMutation,
        variables: { name: 'CI/CD' }
      });
      expect(body.errors[0].message).toBe('BadRequest: Token not found');
    });

    it('should create API key', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: createApiKeyMutation,
          variables: { name: 'CI/CD Pipeline' }
        });
      expect(body.errors).toBeUndefined();
      createdRawKey = body.data.createApiKey;
      expect(createdRawKey).toBeNonEmptyString();
      expect(createdRawKey.length).toBe(32);
    });

    it('should list API keys', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({ query: apiKeysQuery });
      expect(body.errors).toBeUndefined();
      expect(body.data.apiKeys).toBeNonEmptyArray();
      expect(body.data.apiKeys[0].name).toBe('CI/CD Pipeline');
      expect(body.data.apiKeys[0].isActive).toBe(true);
      createdKeyId = body.data.apiKeys[0].id;
    });

    it('should get specific API key by ID', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: apiKeyQuery,
          variables: { id: createdKeyId }
        });
      expect(body.errors).toBeUndefined();
      expect(body.data.apiKey.id).toBe(createdKeyId);
      expect(body.data.apiKey.name).toBe('CI/CD Pipeline');
      expect(body.data.apiKey.isActive).toBe(true);
    });

    it('should update API key name', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: updateApiKeyMutation,
          variables: { keyId: createdKeyId, name: 'Updated CI/CD' }
        });
      expect(body.errors).toBeUndefined();
      expect(body.data.updateApiKey).toBe(1);

      const { body: getBody } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: apiKeyQuery,
          variables: { id: createdKeyId }
        });
      expect(getBody.data.apiKey.name).toBe('Updated CI/CD');
    });

    it('should deactivate API key', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: updateApiKeyMutation,
          variables: { keyId: createdKeyId, isActive: false }
        });
      expect(body.errors).toBeUndefined();
      expect(body.data.updateApiKey).toBe(1);

      const { body: getBody } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: apiKeyQuery,
          variables: { id: createdKeyId }
        });
      expect(getBody.data.apiKey.isActive).toBe(false);
    });

    it('should reactivate API key', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: updateApiKeyMutation,
          variables: { keyId: createdKeyId, isActive: true }
        });
      expect(body.errors).toBeUndefined();
      expect(body.data.updateApiKey).toBe(1);

      const { body: getBody } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: apiKeyQuery,
          variables: { id: createdKeyId }
        });
      expect(getBody.data.apiKey.isActive).toBe(true);
    });

    it('should update API key with no changes but replace (returns 1)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: updateApiKeyMutation,
          variables: { keyId: createdKeyId, name: 'Updated CI/CD' }
        });
      expect(body.data.updateApiKey).toBe(1);
    });

    it('should delete API key', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: deleteApiKeyMutation,
          variables: { keyId: createdKeyId }
        });
      expect(body.errors).toBeUndefined();
      expect(body.data.deleteApiKey).toBe(true);

      const { body: getBody } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: apiKeyQuery,
          variables: { id: createdKeyId }
        });
      expect(getBody.errors[0].message).toBe('BadRequest: Data not found');
    });

    it('should NOT delete non-existent API key', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authToken}`)
        .send({
          query: deleteApiKeyMutation,
          variables: { keyId: 99999 }
        });
      expect(body.data.deleteApiKey).toBe(false);
    });

    describe('rotateApiKey', () => {
      let keyId = 0;

      beforeEach(async () => {
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Key To Rotate' }
          });
        keyId = (await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({ query: apiKeysQuery }))
          .body.data.apiKeys[0].id;
      });

      it('should rotate API key (create new, delete old)', async () => {
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: rotateApiKeyMutation,
            variables: { keyId }
          });
        expect(body.errors).toBeUndefined();
        const newRawKey = body.data.rotateApiKey;
        expect(newRawKey).toBeNonEmptyString();
        expect(newRawKey.length).toBe(32);
        expect(newRawKey).not.toBe(createdRawKey);

        const { body: getBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: apiKeyQuery,
            variables: { id: keyId }
          });
        expect(getBody.errors[0].message).toBe('BadRequest: Data not found');

        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({ query: apiKeysQuery });
        expect(listBody.data.apiKeys).toBeNonEmptyArray();
        expect(listBody.data.apiKeys[0].name).toBe('Key To Rotate');
      });

      it('should NOT rotate non-existent key', async () => {
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: rotateApiKeyMutation,
            variables: { keyId: 8888 }
          });
        expect(body.errors[0].message).toBe('BadRequest: Data not found');
      });

      it('should NOT rotate key owned by another user', async () => {
        const { body: registerBody } = await agent.post('/graphql').send({
          query: `
            mutation RegisterUser($username: String!, $password: String!) {
              registerUser(args: {
                email: "owner@test.com"
                username: $username
                password: $password
                regionId: 1
                programId: 1
              }) { uuid username }
            }
          `,
          variables: { username: keyOwner, password: 'TestPass123!' }
        });
        expect(registerBody.errors).toBeUndefined();

        const { body: loginBody } = await agent.post('/login').send({
          user: { username: keyOwner, password: 'TestPass123!' }
        });
        const ownerToken = loginBody.bearer;
        expect(ownerToken).toBeNonEmptyString();

        const { body: createBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${ownerToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Owner Key' }
          });
        expect(createBody.errors).toBeUndefined();

        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${ownerToken}`)
          .send({ query: apiKeysQuery });
        expect(listBody.data.apiKeys).toBeNonEmptyArray();
        const ownerKeyId = listBody.data.apiKeys[0].id;

        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: rotateApiKeyMutation,
            variables: { keyId: ownerKeyId }
          });
        expect(body.errors[0].message).toBe('BadRequest: Data not found');
      });
    });

    describe('API Key Authentication', () => {
      let apiKey = "";

      beforeEach(async () => {
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Auth Test Key' }
          });
        apiKey = body.data.createApiKey;
      });

      it('should authenticate with X-API-Key header', async () => {
        const { body } = await agent
          .post('/graphql')
          .set('X-API-Key', apiKey)
          .send({
            query: `query { myself { username } }`
          });
        expect(body.errors).toBeUndefined();
        expect(body.data.myself.username).toBe(username);
      });

      it('should NOT authenticate with invalid X-API-Key', async () => {
        const { body } = await agent
          .post('/graphql')
          .set('X-API-Key', 'invalid_key_123')
          .send({
            query: `query { myself { username } }`
          });
        expect(body.errors[0].message).toBe('Unauthorized');
      });

      it('should NOT authenticate with deactivated key', async () => {
        const { body: keysBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({ query: apiKeysQuery });
        const keyId = keysBody.data.apiKeys.find(k => k.name === 'Auth Test Key').id;

        await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${authToken}`)
          .send({
            query: updateApiKeyMutation,
            variables: { keyId, isActive: false }
          });

        const { body } = await agent
          .post('/graphql')
          .set('X-API-Key', apiKey)
          .send({
            query: `query { myself { username } }`
          });
        expect(body.errors[0].message).toBe('Unauthorized');
      });
    });
  });

  describe('Additional API Key Tests', () => {
    beforeAll(async () => {
      await cleanupApiKeyDb();
      await global.knex.raw('DELETE FROM user_ref WHERE username = ?', [addUsername]);
    });

    afterAll(async () => {
      await global.knex.raw('DELETE FROM user_ref WHERE username = ?', [addUsername]);
    });

    it('should register test user for additional tests', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          mutation RegisterUser($username: String!, $password: String!) {
            registerUser(args: {
              email: "additional@test.com"
              username: $username
              password: $password
              firstname: "Additional"
              lastname: "Test"
              regionId: 1
              programId: 1
            }) {
              uuid
              username
            }
          }
        `,
        variables: { username: addUsername, password: addPassword }
      });
      expect(body.errors).toBeUndefined();
      addUserUuid = body.data.registerUser.uuid;
    });

    it('should login for additional tests', async () => {
      const { body } = await agent.post('/login').send({
        user: { username: addUsername, password: addPassword }
      });
      addAuthToken = body.bearer;
      expect(addAuthToken).toBeNonEmptyString();
    });

    describe('Expiration Tests', () => {
      it('should create API key with expiration date', async () => {
        const expiresAt = new Date(Date.now() + 86400000).toISOString();
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Expiring Key', expiresAt }
          });
        expect(body.errors).toBeUndefined();

        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const key = listBody.data.apiKeys.find(k => k.name === 'Expiring Key');
        expect(key).toBeDefined();

        const { body: getBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: apiKeyQuery,
            variables: { id: key.id }
          });
        expect(getBody.data.apiKey.expiresAt).toBe(expiresAt.replace('Z', '+00:00'));
      });

      it('should update API key expiration', async () => {
        const { body: createBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Update Expiry Key' }
          });
        expect(createBody.errors).toBeUndefined();

        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys.find(k => k.name === 'Update Expiry Key').id;

        const newExpiresAt = new Date(Date.now() + 86400000).toISOString();
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: updateApiKeyExpiryMutation,
            variables: { keyId, expiresAt: newExpiresAt }
          });
        expect(body.errors).toBeUndefined();
        expect(body.data.updateApiKey).toBe(1);

        const { body: getBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: apiKeyQuery,
            variables: { id: keyId }
          });
        expect(getBody.data.apiKey.expiresAt).toBe(newExpiresAt.replace('Z', '+00:00'));
      });

      it('should NOT authenticate with expired key', async () => {
        const expiresAt = new Date(Date.now() - 3600000).toISOString();
        const { body: createBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Expired Key' }
          });
        expect(createBody.errors).toBeUndefined();
        const apiKey = createBody.data.createApiKey;

        // Get key ID
        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys.find(k => k.name === 'Expired Key').id;

        // Update expires_at to past date directly in DB
        const pastDate = new Date(Date.now() - 3600000).toISOString();
        await global.knex.raw('UPDATE user_api_key_ref SET expires_at = ? WHERE id = ?', [pastDate, keyId]);

        const { body } = await agent
          .post('/graphql')
          .set('X-API-Key', apiKey)
          .send({
            query: `query { myself { username } }`
          });
        expect(body.errors[0].message).toBe('Unauthorized');
      });
    });

    describe('Validation Tests', () => {
      it('should NOT create API key with empty name', async () => {
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: "" }
          });
        expect(body.errors[0].message).toBe('BadRequest: Data not found');
      });

      it('should NOT create API key with very long name (>100 chars)', async () => {
        const longName = 'a'.repeat(101);
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: longName }
          });
        expect(body.errors[0].message).toBe('BadRequest: Text must be less than 100 characters');
      });
    });

    describe('Authorization Tests', () => {
      it('should NOT list API keys without auth', async () => {
        const { body } = await agent
          .post('/graphql')
          .send({ query: apiKeysQuery });
        expect(body.errors[0].message).toBe('BadRequest: Token not found');
      });

      it('should NOT get API key without auth', async () => {
        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys[0]?.id || 1;

        const { body } = await agent
          .post('/graphql')
          .send({
            query: apiKeyQuery,
            variables: { id: keyId }
          });
        expect(body.errors[0].message).toBe('BadRequest: Token not found');
      });

      it('should return error for non-existent API key', async () => {
        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: apiKeyQuery,
            variables: { id: 99999 }
          });
        expect(body.errors[0].message).toBe('BadRequest: Data not found');
      });

      it('should NOT delete API key without auth', async () => {
        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys[0]?.id || 1;

        const { body } = await agent
          .post('/graphql')
          .send({
            query: deleteApiKeyMutation,
            variables: { keyId }
          });
        expect(body.errors[0].message).toBe('BadRequest: Token not found');
      });

      it('should NOT rotate API key without auth', async () => {
        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys[0]?.id || 1;

        const { body } = await agent
          .post('/graphql')
          .send({
            query: rotateApiKeyMutation,
            variables: { keyId }
          });
        expect(body.errors[0].message).toBe('BadRequest: Token not found');
      });

      it('should NOT update API key without auth', async () => {
        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys[0]?.id || 1;

        const { body } = await agent
          .post('/graphql')
          .send({
            query: updateApiKeyMutation,
            variables: { keyId, name: 'New Name' }
          });
        expect(body.errors[0].message).toBe('BadRequest: Token not found');
      });
    });

    describe('Multiple Keys Tests', () => {
      it('should create multiple API keys for same user', async () => {
        const keys = ['Key 1', 'Key 2', 'Key 3'];
        for (const name of keys) {
          const { body } = await agent
            .post('/graphql')
            .set('Authorization', `Bearer ${addAuthToken}`)
            .send({
              query: createApiKeyMutation,
              variables: { name }
            });
          expect(body.errors).toBeUndefined();
        }

        const { body } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyNames = body.data.apiKeys.map(k => k.name);
        for (const name of keys) {
          expect(keyNames).toContain(name);
        }
      });
    });

    describe('lastUsedAt Tests', () => {
      it('should update lastUsedAt when key is used', async () => {
        const { body: createBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: createApiKeyMutation,
            variables: { name: 'Last Used Test' }
          });
        expect(createBody.errors).toBeUndefined();
        const apiKey = createBody.data.createApiKey;

        const { body: listBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({ query: apiKeysQuery });
        const keyId = listBody.data.apiKeys.find(k => k.name === 'Last Used Test').id;

        await agent
          .post('/graphql')
          .set('X-API-Key', apiKey)
          .send({
            query: `query { myself { username } }`
          });

        const { body: getBody } = await agent
          .post('/graphql')
          .set('Authorization', `Bearer ${addAuthToken}`)
          .send({
            query: apiKeyQuery,
            variables: { id: keyId }
          });
        expect(getBody.data.apiKey.lastUsedAt).toBeNonEmptyString();
      });
    });
  });
});