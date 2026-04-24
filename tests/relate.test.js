const debug = require('debug')('cdbs-back:relate.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(30000);

// ==============================================
// ТЕСТОВЫЕ ДАННЫЕ
// ==============================================
const username = "baromi";
const username2 = "simaco";
const password = "password";

const uuidFail = "aba22d59-4f6c-24a4-9a37-2d38f0e577a8";

// language
const langId1 = 1;
const langId2 = 2;

const specLevels3 = [247, 286, 437, 465, 480, 379, 400, 4];
const specId5 = 5;
const specPath5Level5 = "ROOT/Structural Components/Fastening Elements/Threaded Fasteners/Screws and bolts";
const specPath5 = "Fastening Elements/Threaded Fasteners/Screws and bolts";
const specPathSplit5 = "ROOT#Structural Components#Fastening Elements#Threaded Fasteners#Screws and bolts";
const specPathDepth5 = 5;
let specName4 = "";
let specName5 = "";
let specPath10 = "";

const paramNameTest = "testparametr";
const paramNameTest2 = "testparametr2";
const paramNameTest3 = "testparametr3";
let paramIdTest = 1000000;
let paramIdTest2 = 1000000;

// ==============================================
// GRAPHQL ЗАПРОСЫ
// ==============================================
const paramTranslateList = `
  paramId
  langId
  paramname
`;

const specTranslateList = `
  specId
  langId
  spec
`;

const specPathQuery = `
  specId
  langId
  path
  depth
`;

// ==============================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
// ==============================================
async function cleanupParamDb() {
  return global.knex.raw('DELETE FROM param_ref WHERE id in (?,?)', [
    paramIdTest,
    paramIdTest2,
  ]);
}

async function cleanupParamTranslateDb() {
  return global.knex.raw('DELETE FROM param_translate_list WHERE paramname in (?,?,?)', [
    paramNameTest,
    paramNameTest2,
    paramNameTest3,
  ]);
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?)', [
    username,
    username2,
  ]);
}

// ==============================================
// ТЕСТЫ
// ==============================================
describe('relate', () => {
  let agent;
  let authorizationTokenFirst = "";
  let authorizationTokenSecond = "";

  beforeAll(async () => {
    await cleanupParamDb();
    await cleanupParamTranslateDb();
    await cleanupUserDb();
    agent = request.agent(url);
  });

  afterAll(async () => {
    await cleanupParamDb();
    await cleanupParamTranslateDb();
    await cleanupUserDb();
  });

  // ==============================================
  // РЕГИСТРАЦИЯ ПОЛЬЗОВАТЕЛЕЙ
  // ==============================================
  describe('User Registration', () => {
    it('should register the first user', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          mutation RegisterUser($userData: IptUserData!) {
            registerUser(args: $userData) {
              uuid
              programId
              username
            }
          }
        `,
        variables: {
          userData: {
            email: "testemail@mail.ru",
            firstname: "test_firstname",
            lastname: "test_lastname",
            secondname: "test_secondname",
            username: username,
            password: password,
            phone: "test_phone",
            description: "test_description",
            address: "test_address",
            position: "test_position",
            timeZone: "Europe/Moscow",
            regionId: 1,
            programId: 1,
          },
        },
      });

      expect(body.data.registerUser).toBeDefined();
      expect(body.data.registerUser.uuid).toBeNonEmptyString();
      expect(body.data.registerUser.programId).toBe(1);
      expect(body.data.registerUser.username).toBe(username);
    });

    it('should login the first user', async () => {
      const { body } = await agent.post('/login').send({
        user: {
          username: username,
          password: password,
        },
      });

      expect(body.bearer).toBeNonEmptyString();
      authorizationTokenFirst = body.bearer;
    });

    it('should register the second user', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          mutation RegisterUser($userData: IptUserData!) {
            registerUser(args: $userData) {
              uuid
              programId
              username
            }
          }
        `,
        variables: {
          userData: {
            email: "testemail2@mail.ru",
            firstname: "test_firstname2",
            lastname: "test_lastname2",
            secondname: "test_secondname2",
            username: username2,
            password: password,
            phone: "test_phone2",
            description: "test_description2",
            address: "test_address2",
            position: "test_position2",
            timeZone: "Europe/Moscow",
            regionId: 1,
            programId: 5,
          },
        },
      });

      expect(body.data.registerUser).toBeDefined();
      expect(body.data.registerUser.uuid).toBeNonEmptyString();
      expect(body.data.registerUser.programId).toBe(5);
      expect(body.data.registerUser.username).toBe(username2);
    });

    it('should login the second user', async () => {
      const { body } = await agent.post('/login').send({
        user: {
          username: username2,
          password: password,
        },
      });

      expect(body.bearer).toBeNonEmptyString();
      authorizationTokenSecond = body.bearer;
    });
  });

  // ==============================================
  // ПАРАМЕТРЫ (PARAM)
  // ==============================================
  describe('Param Management', () => {
    it('should NOT register a param without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          mutation RegisterParam($data: IptParamData!) {
            registerParam(args: $data) {
              ${paramTranslateList}
            }
          }
        `,
        variables: {
          data: {
            langId: langId1,
            paramname: paramNameTest,
          },
        },
      });

      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Token not found');
      expect(body.errors[0].path[0]).toBe('registerParam');
    });

    it('should register a new param', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            mutation RegisterParam($data: IptParamData!) {
              registerParam(args: $data) {
                ${paramTranslateList}
              }
            }
          `,
          variables: {
            data: {
              langId: langId1,
              paramname: paramNameTest,
            },
          },
        });

      expect(body.data.registerParam.langId).toBe(langId1);
      expect(body.data.registerParam.paramname).toBe(paramNameTest);
      paramIdTest = body.data.registerParam.paramId;
    });

    it('should return existing param ID when registering duplicate', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            mutation RegisterParam($data: IptParamData!) {
              registerParam(args: $data) {
                paramId
                langId
                paramname
              }
            }
          `,
          variables: {
            data: {
              langId: langId1,
              paramname: paramNameTest,
            },
          },
        });

      expect(body.data.registerParam.paramId).toBe(paramIdTest);
      expect(body.data.registerParam.langId).toBe(langId1);
      expect(body.data.registerParam.paramname).toBe(paramNameTest);
    });

    it('should register another param', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            mutation RegisterParam($data: IptParamData!) {
              registerParam(args: $data) {
                paramId
                langId
                paramname
              }
            }
          `,
          variables: {
            data: {
              langId: langId1,
              paramname: paramNameTest2,
            },
          },
        });

      expect(body.data.registerParam).not.toBeNull();
      paramIdTest2 = body.data.registerParam.paramId;
    });

    it('should register multiple params in bulk', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            mutation RegisterParamsBulk($data: [IptParamData!]!) {
              registerParamsBulk(args: $data) {
                paramId
                langId
                paramname
              }
            }
          `,
          variables: {
            data: [
              { langId: langId1, paramname: paramNameTest },
              { langId: langId1, paramname: paramNameTest2 },
              { langId: langId2, paramname: paramNameTest2 },
              { langId: langId1, paramname: paramNameTest3 },
            ],
          },
        });

      expect(body.data.registerParamsBulk[0].paramId).toBe(paramIdTest);
      expect(body.data.registerParamsBulk[0].paramname).toBe(paramNameTest);
      expect(body.data.registerParamsBulk[1].paramId).toBe(paramIdTest2);
      expect(body.data.registerParamsBulk[1].paramname).toBe(paramNameTest2);
      expect(body.data.registerParamsBulk[2].paramId).not.toBe(paramIdTest2);
      expect(body.data.registerParamsBulk[2].langId).toBe(langId2);
      expect(body.data.registerParamsBulk[2].paramname).toBe(paramNameTest2);
      expect(body.data.registerParamsBulk[3].paramId).not.toBeNull();
      expect(body.data.registerParamsBulk[3].paramname).toBe(paramNameTest3);
    });
  });

  // ==============================================
  // СПИСКИ ПАРАМЕТРОВ
  // ==============================================
  describe('Params List', () => {
    it('should return list of params', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetParams {
              params {
                paramId
                paramname
              }
            }
          `,
        });

      expect(body.data.params).toBeNonEmptyArray();
    });

    it('should return param by ID', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetParams($paramIds: [Int!]) {
              params(paramIds: $paramIds) {
                paramId
                paramname
              }
            }
          `,
          variables: { paramIds: 2 },
        });

      expect(body.data.params[0].paramId).toBe(2);
      expect(body.data.params[0].paramname).toBe("Selector");
    });

    it('should return params by array of IDs', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetParams($paramIds: [Int!]) {
              params(paramIds: $paramIds) {
                paramId
                paramname
              }
            }
          `,
          variables: { paramIds: [1, 2] },
        });

      expect(body.data.params[1].paramId).toBe(2);
      expect(body.data.params[1].paramname).toBe("Selector");
    });

    it('should NOT return params without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          query GetParams($paramIds: [Int!]) {
            params(paramIds: $paramIds) {
              paramId
              paramname
            }
          }
        `,
        variables: { paramIds: [1, 2] },
      });

      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Token not found');
      expect(body.errors[0].path[0]).toBe('params');
    });
  });

  // ==============================================
  // ПУТИ СПЕЦИФИКАЦИЙ (SPECS PATHS)
  // ==============================================
  describe('Specs Paths', () => {
    it('should NOT return path for invalid spec ID without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          query GetSpecsPaths($specIds: [Int!]) {
            specsPaths(args: { specIds: $specIds }) {
              ${specPathQuery}
            }
          }
        `,
        variables: { specIds: 0 },
      });

      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Spec not found');
      expect(body.errors[0].path[0]).toBe('specsPaths');
    });

    it('should NOT return path for invalid spec ID', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths($specIds: [Int!]) {
              specsPaths(args: { specIds: $specIds }) {
                ${specPathQuery}
              }
            }
          `,
          variables: { specIds: 0 },
        });

      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Spec not found');
      expect(body.errors[0].path[0]).toBe('specsPaths');
    });

    it('should return path for spec ID 5', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths($specIds: [Int!]) {
              specsPaths(args: { specIds: $specIds }) {
                ${specPathQuery}
              }
            }
          `,
          variables: { specIds: specId5 },
        });

      expect(body.data.specsPaths[0].path).toBe(specPath5);
      expect(body.data.specsPaths[0].depth).toBe(specPathDepth5);
    });

    it('should return path with custom split character', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths($specIds: [Int!], $splitChar: String, $depthLevel: Int) {
              specsPaths(args: { specIds: $specIds, splitChar: $splitChar, depthLevel: $depthLevel }) {
                ${specPathQuery}
              }
            }
          `,
          variables: {
            specIds: specId5,
            splitChar: "#",
            depthLevel: 50,
          },
        });

      expect(body.data.specsPaths[0].path).toBe(specPathSplit5);
    });

    it('should return all specs paths without params', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths {
              specsPaths {
                ${specPathQuery}
              }
            }
          `,
        });

      expect(body.data.specsPaths[1].path).toBeNonEmptyString();
      expect(body.data.specsPaths.length).toBe(100);
      specPath10 = body.data.specsPaths[10].path;
    });

    it('should return path with depth level 5', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths($specIds: [Int!], $depthLevel: Int) {
              specsPaths(args: { specIds: $specIds, depthLevel: $depthLevel }) {
                ${specPathQuery}
              }
            }
          `,
          variables: {
            specIds: specId5,
            depthLevel: 5,
          },
        });

      expect(body.data.specsPaths[0].specId).toBe(specId5);
      expect(body.data.specsPaths[0].path).toBe(specPath5Level5);
      expect(body.data.specsPaths[0].depth).toBe(specPathDepth5);
      expect(body.data.specsPaths.length).toBe(1);
    });

    it('should return paths with depth level 5 (second test)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths($specIds: [Int!], $depthLevel: Int) {
              specsPaths(args: { specIds: $specIds, depthLevel: $depthLevel }) {
                ${specPathQuery}
              }
            }
          `,
          variables: {
            specIds: specId5,
            depthLevel: 5,
          },
        });

      expect(body.data.specsPaths[0].specId).toBe(specId5);
      expect(body.data.specsPaths[0].path).toBe(specPath5Level5);
      expect(body.data.specsPaths[0].depth).toBe(specPathDepth5);
      expect(body.data.specsPaths.length).toBe(1);
    });

    it('should return paths with pagination', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecsPaths($page: Int, $perPage: Int) {
              specsPaths(paginate: { currentPage: $page, perPage: $perPage }) {
                ${specPathQuery}
              }
            }
          `,
          variables: { page: 2, perPage: 10 },
        });

      expect(body.data.specsPaths[0].path).toBe(specPath10);
      expect(body.data.specsPaths.length).toBe(10);
    });
  });

  // ==============================================
  // СПЕЦИФИКАЦИИ (SPECS)
  // ==============================================
  describe('Specs', () => {
    it('should return empty array for invalid spec ID without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          query GetSpecs($specIds: [Int!]) {
            specs(args: { specIds: $specIds }) {
              ${specTranslateList}
            }
          }
        `,
        variables: { specIds: 0 },
      });

      expect(body.data.specs).toBeEmptyArray();
    });

    it('should return empty array for invalid spec ID', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs($specIds: [Int!]) {
              specs(args: { specIds: $specIds }) {
                specId
                spec
              }
            }
          `,
          variables: { specIds: 0 },
        });

      expect(body.data.specs).toBeEmptyArray();
    });

    it('should return spec by ID', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs($specIds: [Int!]) {
              specs(args: { specIds: $specIds }) {
                specId
                spec
              }
            }
          `,
          variables: { specIds: specId5 },
        });

      specName5 = body.data.specs[0].spec;
      expect(body.data.specs[0].specId).toBe(specId5);
    });

    it('should return specs with parent hierarchy', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs($specsLevels: Int) {
              specs(args: { specsLevels: $specsLevels }) {
                specId
                spec
                parentSpec {
                  specId
                  spec
                  parentSpec {
                    specId
                    spec
                    parentSpec {
                      specId
                      spec
                      parentSpec {
                        specId
                        spec
                        parentSpec {
                          specId
                          spec
                        }
                      }
                    }
                  }
                }
              }
            }
          `,
          variables: { specsLevels: 4 },
        });

      expect(body.data.specs).toBeNonEmptyArray();
      expect(body.data.specs[1].parentSpec.parentSpec.parentSpec.specId).not.toBe(1);
      expect(body.data.specs[1].parentSpec.parentSpec.parentSpec.parentSpec.specId).toBe(1);
    });

    it('should return empty array for invalid filter combination', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs($specIds: [Int!], $specsLevels: Int) {
              specs(args: { specIds: $specIds, specsLevels: $specsLevels }) {
                specId
                spec
              }
            }
          `,
          variables: {
            specIds: specLevels3,
            specsLevels: 4,
          },
        });

      expect(body.data.specs).toBeEmptyArray();
    });

    it('should return specs by level with filter', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs($specIds: [Int!], $specsLevels: Int) {
              specs(args: { specIds: $specIds, specsLevels: $specsLevels }) {
                specId
                spec
              }
            }
          `,
          variables: {
            specIds: specLevels3,
            specsLevels: 3,
          },
        });
      specName4 = body.data.specs[3].spec;
      expect(body.data.specs).toBeNonEmptyArray();
    });

    it('should return all specs without params', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs {
              specs {
                specId
                spec
              }
            }
          `,
        });

      expect(body.data.specs[1].spec).toBeNonEmptyString();
      expect(body.data.specs.length).toBe(100);
    });

    it('should return specs without params (second test)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs {
              specs {
                specId
                spec
              }
            }
          `,
        });

      expect(body.data.specs).toBeNonEmptyArray();
      expect(body.data.specs[1].spec).toBeNonEmptyString();
      expect(body.data.specs.length).toBe(100);
    });

    it('should return specs with pagination', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetSpecs($specIds: [Int!], $specsLevels: Int, $page: Int, $perPage: Int) {
              specs(
                args: { specIds: $specIds, specsLevels: $specsLevels }
                paginate: { currentPage: $page, perPage: $perPage }
              ) {
                specId
                spec
              }
            }
          `,
          variables: {
            specIds: specLevels3,
            specsLevels: 3,
            page: 2,
            perPage: 3,
          },
        });

      expect(body.data.specs[0].spec).toBe(specName4);
      expect(body.data.specs.length).toBe(1);
    });
  });

  // ==============================================
  // ПОИСК СПЕЦИФИКАЦИЙ
  // ==============================================
  describe('Search Specs', () => {
    it('should search specs by text without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          query SearchSpecs($text: String!) {
            searchSpecs(args: { text: $text }) {
              specId
              path
              langId
            }
          }
        `,
        variables: { text: "bolt" },
      });

      expect(body.data.searchSpecs.length).toBe(5);
      expect(body.data.searchSpecs[0].specId).toBe(5);
      expect(body.data.searchSpecs[1].specId).toBe(8);
      expect(body.data.searchSpecs[1].path).toBe("Threaded Fasteners/Screws and bolts/U Bolts");
      expect(body.data.searchSpecs[2].specId).toBe(7);
      expect(body.data.searchSpecs[3].specId).toBe(9);
      expect(body.data.searchSpecs[4].specId).toBe(6);
    });

    it('should return empty array for empty search text', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query SearchSpecs($text: String!) {
              searchSpecs(args: { text: $text }) {
                specId
                path
                langId
              }
            }
          `,
          variables: { text: "" },
        });

      expect(body.data.searchSpecs).toBeEmptyArray();
    });

    it('should search specs by name', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query SearchSpecs($text: String!) {
              searchSpecs(args: { text: $text }) {
                specId
                path
                langId
              }
            }
          `,
          variables: { text: specName5 },
        });

      expect(body.data.searchSpecs[0].specId).toBe(specId5);
    });

    it('should search specs with custom split character', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query SearchSpecs($text: String!, $splitChar: String, $depthLevel: Int) {
              searchSpecs(args: { text: $text, splitChar: $splitChar, depthLevel: $depthLevel }) {
                specId
                path
                langId
              }
            }
          `,
          variables: {
            text: specName5,
            splitChar: "#",
            depthLevel: 50,
          },
        });

      expect(body.data.searchSpecs[0].path).toBe(specPathSplit5);
    });

    it('should search specs in Russian', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'ru')
        .send({
          query: `
            query SearchSpecs($text: String!) {
              searchSpecs(args: { text: $text }) {
                specId
                path
                langId
              }
            }
          `,
          variables: { text: "болт" },
        });

      expect(body.data.searchSpecs[1].path).toBeNonEmptyString();
      expect(body.data.searchSpecs.length).toBe(5);
    });

    it('should search specs with depth level 1', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query SearchSpecs($text: String!, $depthLevel: Int) {
              searchSpecs(args: { text: $text, depthLevel: $depthLevel }) {
                specId
                path
                langId
              }
            }
          `,
          variables: {
            text: specName5,
            depthLevel: 1,
          },
        });

      expect(body.data.searchSpecs[0].path).toBe(specName5);
    });

    it('should search specs with pagination', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query SearchSpecs($text: String!, $depthLevel: Int, $page: Int, $perPage: Int) {
              searchSpecs(
                args: { text: $text, depthLevel: $depthLevel }
                paginate: { currentPage: $page, perPage: $perPage }
              ) {
                specId
                path
                langId
              }
            }
          `,
          variables: {
            text: "bolt",
            depthLevel: 1,
            page: 2,
            perPage: 2,
          },
        });

      expect(body.data.searchSpecs.length).toBe(2);
    });
  });

  // ==============================================
  // ТИПЫ КОМПАНИЙ
  // ==============================================
  describe('Company Types', () => {
    it('should NOT return company types without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          query GetCompanyTypes {
            companyTypes {
              name
              langId
              companyTypeId
            }
          }
        `,
      });

      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Token not found');
      expect(body.errors[0].path[0]).toBe('companyTypes');
    });

    it('should return company types in English', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetCompanyTypes {
              companyTypes {
                name
                langId
                companyTypeId
              }
            }
          `,
        });

      expect(body.data.companyTypes).toBeNonEmptyArray();
      expect(body.data.companyTypes[0].langId).toBe(1);
    });

    it('should return company types in Russian', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'ru')
        .send({
          query: `
            query GetCompanyTypes {
              companyTypes {
                name
                langId
                companyTypeId
              }
            }
          `,
        });

      expect(body.data.companyTypes).toBeNonEmptyArray();
      expect(body.data.companyTypes[0].langId).toBe(2);
    });

    it('should return company types in Chinese (zh-Hans)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh-Hans')
        .send({
          query: `
            query GetCompanyTypes {
              companyTypes {
                name
                langId
                companyTypeId
              }
            }
          `,
        });

      expect(body.data.companyTypes).toBeNonEmptyArray();
      expect(body.data.companyTypes[0].langId).toBe(3);

      const limitedCompany = body.data.companyTypes.find(ct => ct.companyTypeId === 1);
      expect(limitedCompany).toBeDefined();
      expect(limitedCompany.name).toBe("有限责任公司");
    });

    it('should return company types in Chinese (zh-Hant)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh-Hant')
        .send({
          query: `
            query GetCompanyTypes {
              companyTypes {
                name
                langId
                companyTypeId
                shortname
              }
            }
          `,
        });

      expect(body.data.companyTypes).toBeNonEmptyArray();
      expect(body.data.companyTypes[0].langId).toBe(3);
      expect(body.data.companyTypes[0].name).toBe("上市公司");

      const companyTypeIds = body.data.companyTypes.map(ct => ct.companyTypeId);
      expect(companyTypeIds).toContain(1);
      expect(companyTypeIds).toContain(2);
      expect(companyTypeIds).toContain(3);
      expect(companyTypeIds).toContain(4);
      expect(companyTypeIds).toContain(5);
    });
  });

  // ==============================================
  // ТИПЫ ПРЕДСТАВИТЕЛЬСТВ КОМПАНИЙ
  // ==============================================
  describe('Company Representation Types', () => {
    it('should NOT return representation types without token', async () => {
      const { body } = await agent.post('/graphql').send({
        query: `
          query GetCompanyRepresentTypes {
            companyRepresentTypes {
              representationType
              langId
              representationTypeId
            }
          }
        `,
      });

      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Token not found');
      expect(body.errors[0].path[0]).toBe('companyRepresentTypes');
    });

    it('should return representation types in English', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `
            query GetCompanyRepresentTypes {
              companyRepresentTypes {
                representationType
                langId
                representationTypeId
              }
            }
          `,
        });

      expect(body.data.companyRepresentTypes).toBeNonEmptyArray();
      expect(body.data.companyRepresentTypes[0].langId).toBe(1);
    });

    it('should return representation types in Russian', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'ru')
        .send({
          query: `
            query GetCompanyRepresentTypes {
              companyRepresentTypes {
                representationType
                langId
                representationTypeId
              }
            }
          `,
        });

      expect(body.data.companyRepresentTypes).toBeNonEmptyArray();
      expect(body.data.companyRepresentTypes[0].langId).toBe(2);
    });
  });

  // ==============================================
  // РЕГИОНЫ (CHINESE LOCALIZATION)
  // ==============================================
  describe('Regions - Chinese Localization', () => {
    it('should return regions in Chinese (zh-Hant-HK)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh-Hant-HK')
        .send({
          query: `
            query GetRegions {
              regions {
                regionId
                region
                langId
              }
            }
          `,
        });

      expect(body.data.regions).toBeNonEmptyArray();
      expect(body.data.regions[0].langId).toBe(3);
      expect(body.data.regions[0].region).toBe("中东地区");
      expect(body.data.regions[0].regionId).toBe(5);

      const africa = body.data.regions.find(r => r.regionId === 1);
      expect(africa).toBeDefined();
      expect(africa.region).toBe("非洲");

      const regionIds = body.data.regions.map(r => r.regionId);
      expect(regionIds).toContain(1);
      expect(regionIds).toContain(2);
      expect(regionIds).toContain(3);
      expect(regionIds).toContain(4);
      expect(regionIds).toContain(5);
      expect(regionIds).toContain(6);
      expect(regionIds).toContain(7);
      expect(regionIds).toContain(8);
      expect(regionIds).toContain(9);
      expect(regionIds).toContain(10);
      expect(regionIds).toContain(11);
    });

    it('should return regions in Chinese (zh-Hans-CN)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh-Hans-CN')
        .send({
          query: `
            query GetRegions {
              regions {
                regionId
                region
                langId
              }
            }
          `,
        });

      expect(body.data.regions).toBeNonEmptyArray();
      expect(body.data.regions[0].langId).toBe(3);

      const china = body.data.regions.find(r => r.regionId === 12);
      if (china) {
        expect(china.region).toBeNonEmptyString();
      }
    });

    it('should return regions in alphabetical order for Chinese', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh')
        .send({
          query: `
            query GetRegions {
              regions {
                regionId
                region
              }
            }
          `,
        });

      const regionNames = body.data.regions.map(r => r.region);
      expect(regionNames[0]).toBe("中东地区");
      expect(regionNames[1]).toBe("亚太地区");
      expect(regionNames[2]).toBe("其他地区");
      expect(regionNames[3]).toBe("北美洲");
    });
  });

  // ==============================================
  // ТИПЫ ДОСТУПА (CHINESE LOCALIZATION)
  // ==============================================
  describe('Type Access - Chinese Localization', () => {
    it('should return type access in Chinese (zh-Hans)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh-Hans')
        .send({
          query: `
            query GetTypeAccess {
              typesAccess {
                typeAccessId
                name
                langId
              }
            }
          `,
        });

      expect(body.data.typesAccess).toBeNonEmptyArray();
      expect(body.data.typesAccess[2].langId).toBe(3);
      expect(body.data.typesAccess[2].name).toBe("公开");
    });

    it('should return type access in Chinese (zh-Hant-TW)', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .set('Accept-Language', 'zh-Hant-TW')
        .send({
          query: `
            query GetTypeAccess {
              typesAccess {
                typeAccessId
                name
                langId
              }
            }
          `,
        });

      expect(body.data.typesAccess).toBeNonEmptyArray();
      expect(body.data.typesAccess[2].langId).toBe(3);
      expect(body.data.typesAccess[2].name).toBe("公开");
    });
  });
});