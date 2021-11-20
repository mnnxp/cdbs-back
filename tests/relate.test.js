const debug = require('debug')('cdbs-back:component.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

// data for user
const username = "baromi";
const username2 = "simaco";
const password = "password";

const uuidFail = "aba22d59-4f6c-24a4-9a37-2d38f0e577a8";
const userUuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const userUuid2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";

var authorizationTokenFirst = "";
var authorizationTokenSecond = "";

// data for component
const parentComponentUuid = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";

// data for component modification
const parentModificationUuid = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";

// data for param
const paramnameIndexFail = 100;
const paramnameIndex = 2;
const paramname = "Selector";
const paramNameTest = "testparametr";
const paramNameTest2 = "testparametr2";
var paramIdTest = 1000000;
var paramIdTest2 = 1000000;

// language
const langId1 = 1;
const langId2 = 2;

const specId5 = 5;
const specPath5 = "ROOT / MECHANICS (DESIGN, MACHINERY) / MECHANICAL COMPONENTS / Mountings / Screws and bolts";
const specPathSplit5 = "ROOT # MECHANICS (DESIGN, MACHINERY) # MECHANICAL COMPONENTS # Mountings # Screws and bolts";

async function cleanupParamDb() {
  return global.knex.raw('DELETE FROM param_ref WHERE id in (?,?)', [
    paramIdTest,
    paramIdTest2,
  ]);
}

async function cleanupParamTranslateDb() {
  return global.knex.raw('DELETE FROM param_translate_list WHERE paramname in (?,?)', [
    paramNameTest,
    paramNameTest2,
  ]);
}

async function cleanupTokenDb() {
  return global.knex.raw('DELETE FROM user_token_ref');
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?)', [
    username,
    username2,
  ]);
}

describe('param', () => {
  beforeAll(() => {
    cleanupParamDb();
    cleanupParamTranslateDb();
    cleanupTokenDb();
    return cleanupUserDb();
  });
  afterAll(() => {
    cleanupParamDb();
    cleanupParamTranslateDb();
    cleanupTokenDb();
    return cleanupUserDb();
  });

  const agent = request.agent(url);

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "testemail@mail.ru",
                firstname: "test_firstname",
                lastname: "test_lastname",
                secondname: "test_secondname",
                username: "${username}",
                password: "${password}",
                phone: "test_phone",
                description: "test_description",
                address: "test_address",
                position: "test_position",
                timeZone: "Europe/Moscow",
                regionId: 1,
                programId: 1,
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
    expect(registerUser.username).toBe(username);
    done();
  });

  it('/login - OK', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/login body=%o', body);
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenFirst = body.bearer;
        done();
      });
  });

  it('/graphql:M register second - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "testemail@mail.ru",
                firstname: "test_firstname",
                lastname: "test_lastname",
                secondname: "test_secondname",
                username: "${username2}",
                password: "${password}",
                phone: "test_phone",
                description: "test_description",
                address: "test_address",
                position: "test_position",
                timeZone: "Europe/Moscow",
                regionId: 1,
                programId: 5,
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(5);
    expect(registerUser.username).toBe(username2);
    done();
  });

  it('/login second - OK', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username2,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/login body=%o', body);
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenSecond = body.bearer;
        done();
      });
  });

  it('/graphql:M registerParam - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParam( data: {
                langId: ${langId1},
                paramname: "${paramNameTest}"
            }) {
              paramId
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('registerParam');
    done();
  });

  it('/graphql:M registerParam - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerParam( data: {
                langId: ${langId1},
                paramname: "${paramNameTest}",
            }) {
              paramId
              langId
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerParam },
    } = body;
    expect(registerParam).toContainAllKeys([
      "paramId", "langId", "paramname"
    ]);
    expect(registerParam.paramId).not.toBeNull();
    expect(registerParam.langId).toBe(langId1);
    expect(registerParam.paramname).toBe(paramNameTest);
    paramIdTest = registerParam.paramId;   // <-- save data for test "already param"
    done();
  });

  it('/graphql:M registerParam - param name is already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerParam( data: {
                langId: ${langId1},
                paramname: "${paramNameTest}"
            }) {
              paramId
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toInclude(
      "This param name is already there. Id: " + paramIdTest
    );
    done();
  });

  it('/graphql:M registerParam - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerParam( data: {
                langId: ${langId1},
                paramname: "${paramNameTest2}",
            }) {
              paramId
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerParam },
    } = body;
    expect(registerParam).toContainAllKeys([
      "paramId", "paramname"
    ]);
    expect(registerParam.paramId).not.toBeNull();
    expect(registerParam.paramname).toBe(paramNameTest2);
    paramIdTest2 = registerParam.paramId;
    done();
  });

  it('/graphql:Q List param - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListParams {
            params {
                paramId
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.params).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q List param - OK with paramId', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListUserParams {
            params (paramId: ${paramnameIndex}) {
                paramId
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.params).toBeNonEmptyArray();
    expect(response1.body.data.params[0].paramId).toBe(paramnameIndex);
    expect(response1.body.data.params[0].paramname).toBe(paramname);
    done();
  });

  it('/graphql:Q List param - OK with array paramId', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListUserParams {
            params (paramId: [1, ${paramnameIndex}]) {
                paramId
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.params).toBeNonEmptyArray();
    expect(response1.body.data.params[1].paramId).toBe(paramnameIndex);
    expect(response1.body.data.params[1].paramname).toBe(paramname);
    done();
  });

  it('/graphql:Q List param - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query ListUserParams {
            params (paramId: [1, ${paramnameIndex}]) {
                paramId
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('params');
    done();
  });

  // Testing get full path specification
  it('/graphql:Q Spec path - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            specPath (specId: 0)
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('specPath');
    done();
  });

  it('/graphql:Q Spec path - BadRequest id zero', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specPath (specId: 0)
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Spec not found'
    );
    expect(body.errors[0].path[0]).toBe('specPath');
    done();
  });

  it('/graphql:Q Spec path - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specPath (specId: ${specId5})
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { specPath }
    } = body;
    expect(specPath).toBe(specPath5);
    done();
  });

  it('/graphql:Q Spec path - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specPath (
              specId: ${specId5}
              splitChar: "#"
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specPath }
    } = body;
    expect(specPath).toBe(specPathSplit5);
    done();
  });

  // Testing get company types
  it('/graphql:Q Company types - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            companyTypes {
              name
              langId
              companyTypeId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('companyTypes');
    done();
  });

  it('/graphql:Q Company types - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyTypes {
              name
              langId
              companyTypeId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { companyTypes }
    } = body;
    expect(companyTypes).toBeNonEmptyArray();
    expect(companyTypes[0].langId).toBe(1);
    done();
  });

  it('/graphql:Q Company types - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .set(
        'Accept-Language',
        `ru`
      )
      .send({
        query: `query {
            companyTypes {
              name
              langId
              companyTypeId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { companyTypes }
    } = body;
    expect(companyTypes).toBeNonEmptyArray();
    expect(companyTypes[0].langId).toBe(2);
    done();
  });
});
