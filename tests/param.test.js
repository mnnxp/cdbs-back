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
const uuidUser = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuidUser2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";

var authorizationTokenFirst = "";
var authorizationTokenSecond = "";

// data for component
const uuidComponentParent = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";

// data for component modification
const uuidModificationParent = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";

// data for param
const paramnameIndexFail = 100;
const paramnameIndex = 2;
const paramname = "Selector";
const paramNameTest = "testparametr";
const paramNameTest2 = "testparametr2";
var idParamTest = "";

async function cleanupParamDb() {
  return global.knex.raw('DELETE FROM param_ref WHERE paramname in (?,?)', [
    paramNameTest,
    paramNameTest2,
  ]);
}

async function cleanupTokenDb() {
  return global.knex.raw('DELETE FROM user_tokens_ref');
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
    cleanupTokenDb();
    return cleanupUserDb();
  });
  afterAll(() => {
    cleanupParamDb();
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
                uuidImageFile: "bc1c2151-86d0-4656-9c9d-d016dd584297",
                idRegion: 1,
                idProgram: 1,
            }) {
                uuid
                idProgram
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'idProgram', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.idProgram).toBe(1);
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
                uuidImageFile: "bc1c2151-86d0-4656-9c9d-d016dd584297",
                idRegion: 1,
                idProgram: 5,
            }) {
                uuid
                idProgram
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'idProgram', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.idProgram).toBe(5);
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
                paramname: "${paramNameTest}"
            }) {
              id
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
                paramname: "${paramNameTest}",
            }) {
              id
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
      "id", "paramname"
    ]);
    expect(registerParam.id).not.toBeNull();
    expect(registerParam.paramname).toBe(paramNameTest);
    idParamTest = registerParam.id;   // <-- save data for test "already param"
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
                paramname: "${paramNameTest}"
            }) {
              id
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toInclude(
      "This param name is already there. Id: " + idParamTest
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
                paramname: "${paramNameTest2}",
            }) {
              id
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
      "id", "paramname"
    ]);
    expect(registerParam.id).not.toBeNull();
    expect(registerParam.paramname).toBe(paramNameTest2);
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
        query: `query ListParam {
            param {
                id
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.param).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q List param - OK with idParam', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListUserParam {
            param (idParam: ${paramnameIndex}) {
                id
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.param).toBeNonEmptyArray();
    expect(response1.body.data.param[0].id).toBe(paramnameIndex);
    expect(response1.body.data.param[0].paramname).toBe(paramname);
    done();
  });

  it('/graphql:Q List param - OK with array idParam', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListUserParam {
            param (idParam: [1, ${paramnameIndex}]) {
                id
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.param).toBeNonEmptyArray();
    expect(response1.body.data.param[1].id).toBe(paramnameIndex);
    expect(response1.body.data.param[1].paramname).toBe(paramname);
    done();
  });

  it('/graphql:Q List param - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query ListUserParam {
            param (idParam: [1, ${paramnameIndex}]) {
                id
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
    expect(body.errors[0].path[0]).toBe('param');
    done();
  });
});
