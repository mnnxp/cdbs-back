const debug = require('debug')('cdbs-back:relate.test.js');
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

const specLevels3 = [247, 286, 437, 465, 480, 379, 400, 4];
const specId5 = 5;
const specPath5Level5 = "ROOT/MECHANICS (CONSTRUCTION, MECHANICAL ENGINEERING)/MECHANICAL COMPONENTS/Fixings/Screws and bolts";
const specPath5 = "MECHANICAL COMPONENTS/Fixings/Screws and bolts";
const specPathSplit5 = "ROOT#MECHANICS (CONSTRUCTION, MECHANICAL ENGINEERING)#MECHANICAL COMPONENTS#Fixings#Screws and bolts";
var specName4 = "";
var specName5 = "";
var specPath10 = "";

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
    // cleanupTokenDb();
    return cleanupUserDb();
  });
  afterAll(() => {
    cleanupParamDb();
    cleanupParamTranslateDb();
    // cleanupTokenDb();
    return cleanupUserDb();
  });

  const agent = request.agent(url);

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser(args: {
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
            }){
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
            registerUser(args: {
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
            }){
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
            registerParam(args: {
                langId: ${langId1},
                paramname: "${paramNameTest}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            registerParam(args: {
                langId: ${langId1},
                paramname: "${paramNameTest}",
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerParam },
    } = body;
    paramIdTest = registerParam;   // <-- save data for test "already param"
    expect(registerParam).not.toBeNull();
    done();
  });

  it('/graphql:M registerParam - OK param name is already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerParam(args: {
                langId: ${langId1},
                paramname: "${paramNameTest}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerParam },
    } = body;
    expect(registerParam).toBe(paramIdTest);
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
            registerParam(args: {
                langId: ${langId1},
                paramname: "${paramNameTest2}",
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerParam },
    } = body;
    expect(registerParam).not.toBeNull();
    paramIdTest2 = registerParam;
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
            params(args:{
              paramIds: ${paramnameIndex}
            }){
                paramId
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    // expect(response1.body).toBe(0);
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
            params(args:{
              paramIds: [1, ${paramnameIndex}]
            }){
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
            params(args:{
              paramIds: [1, ${paramnameIndex}]
            }){
                paramId
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('params');
    done();
  });

  // Testing get full path specification
  it('/graphql:Q Specs paths - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            specsPaths (args:{
              specIds: 0
            }){
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('specsPaths');
    done();
  });

  it('/graphql:Q Specs paths - BadRequest id zero', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specsPaths (args:{
              specIds: 0
            }){
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Spec not found'
    );
    expect(body.errors[0].path[0]).toBe('specsPaths');
    done();
  });

  it('/graphql:Q Specs paths - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specsPaths (args:{
              specIds: ${specId5}
            }){
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { specsPaths }
    } = body;
    expect(specsPaths[0].path).toBe(specPath5);
    done();
  });

  it('/graphql:Q Specs paths - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specsPaths (args:{
              specIds: ${specId5}
              splitChar: "#"
              depthLevel: 50
            }){
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specsPaths }
    } = body;
    expect(specsPaths[0].path).toBe(specPathSplit5);
    done();
  });

  it('/graphql:Q Specs paths - OK without param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specsPaths {
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specsPaths }
    } = body;
    specPath10 = specsPaths[10].path;
    expect(specsPaths[1].path).toBeNonEmptyString();
    expect(specsPaths.length).toBe(30);
    done();
  });

  it('/graphql:Q Specs paths - OK with set depthLevel', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specsPaths (args:{
              specIds: ${specId5}
              depthLevel: 5
            }){
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specsPaths }
    } = body;
    expect(specsPaths[0].specId).toBe(specId5);
    expect(specsPaths[0].path).toBe(specPath5Level5);
    expect(specsPaths.length).toBe(1);
    done();
  });

  it('/graphql:Q Specs paths - OK with offset and limit', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specsPaths (args:{
                offset: 9
                limit: 10
            }){
              specId
              langId
              path
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specsPaths }
    } = body;
    expect(specsPaths[1].path).toBe(specPath10);
    expect(specsPaths.length).toBe(10);
    done();
  });

  // Testing get specification
  it('/graphql:Q Specs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            specs (args:{
              specIds: 0
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('specs');
    done();
  });

  it('/graphql:Q Specs - BadRequest id zero', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs (args:{
              specIds: 0
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { specs }
    } = body;
    expect(specs).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Specs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs (args:{
              specIds: ${specId5}
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { specs }
    } = body;
    specName5 = specs[0].spec;
    expect(specs[0].specId).toBe(specId5);
    done();
  });

  it('/graphql:Q Specs - OK by level', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs (args:{
              specsLevels: 4
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specs }
    } = body;
    expect(specs).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q Specs - OK filter all', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs (args:{
              specIds: [${specLevels3}]
              specsLevels: 4
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specs }
    } = body;
    expect(specs).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Specs - OK by level with filter', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs (args:{
              specIds: [${specLevels3}]
              specsLevels: 3
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specs }
    } = body;
    specName4 = specs[4].spec;
    expect(specs).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q Specs - OK without param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs {
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specs }
    } = body;
    expect(specs[1].spec).toBeNonEmptyString();
    expect(specs.length).toBe(100);
    done();
  });

  it('/graphql:Q Specs - OK with offset and limit', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            specs (args:{
              specIds: [${specLevels3}]
              specsLevels: 3
              offset: 3
              limit: 2
            }){
              specId
              spec
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { specs }
    } = body;
    expect(specs[1].spec).toBe(specName4);
    expect(specs.length).toBe(2);
    done();
  });

  // Testing search specification
  it('/graphql:Q searchSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            searchSpecs (args:{
              text: "bolt"
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('searchSpecs');
    done();
  });

  it('/graphql:Q searchSpecs - Ok empty str', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            searchSpecs (args:{
              text: ""
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { searchSpecs }
    } = body;
    expect(searchSpecs).toBeEmptyArray();
    done();
  });

  it('/graphql:Q searchSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            searchSpecs (args:{
              text: "${specName5}"
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { searchSpecs }
    } = body;
    expect(searchSpecs[0].specId).toBe(specId5);
    done();
  });

  it('/graphql:Q searchSpecs paths - OK with custom split', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            searchSpecs (args:{
              text: "${specName5}"
              splitChar: "#"
              depthLevel: 50
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { searchSpecs }
    } = body;
    expect(searchSpecs[0].path).toBe(specPathSplit5);
    done();
  });

  it('/graphql:Q searchSpecs - OK ru lang', async (done) => {
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
            searchSpecs (args:{
              text: "болт"
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { searchSpecs }
    } = body;
    expect(searchSpecs[1].path).toBeNonEmptyString();
    expect(searchSpecs.length).toBe(5);
    done();
  });

  it('/graphql:Q searchSpecs - OK with depthLevel 1', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            searchSpecs (args:{
              text: "${specName4}"
              depthLevel: 1
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { searchSpecs }
    } = body;
    expect(searchSpecs[0].path).toBe(specName4);
    done();
  });

  it('/graphql:Q searchSpecs - OK with offset and limit', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            searchSpecs (args:{
              text: "bolt"
              depthLevel: 1
              offset: 1
              limit: 2
            }){
              specId
              path
              langId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { searchSpecs }
    } = body;
    expect(searchSpecs.length).toBe(1);
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
      'BadRequest: Token not found'
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

  // Testing get company represent types
  it('/graphql:Q Company represent types - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            companyRepresentTypes {
              representationType
              langId
              representationTypeId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('companyRepresentTypes');
    done();
  });

  it('/graphql:Q Company represent types - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyRepresentTypes {
              representationType
              langId
              representationTypeId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { companyRepresentTypes }
    } = body;
    expect(companyRepresentTypes).toBeNonEmptyArray();
    expect(companyRepresentTypes[0].langId).toBe(1);
    done();
  });

  it('/graphql:Q Company represent types - OK', async (done) => {
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
            companyRepresentTypes {
              representationType
              langId
              representationTypeId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { companyRepresentTypes }
    } = body;
    expect(companyRepresentTypes).toBeNonEmptyArray();
    expect(companyRepresentTypes[0].langId).toBe(2);
    done();
  });
});
