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

// data for standard
const parentStandardUuid = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const classifierStandard = "GOST-2012-Test";
const nameStandard = "GOST 2012 Test standard";
const descriptionStandard = "Test GOST standard";
const specifiedTolerance = "C";
const technicalCommittee = "GOST";
const publicationAt = "2021-07-31T00:00:00";
const typeAccessId3 = 3;
const typeAccessId1 = 1;
const standardStatusId = 1;
const regionId = 5;
var standardUuidFirst = "";
var standardUuidSecond = "";

// data for company
const orgname = "orgname supplier of the test";
const orgname2 = "orgnametest not supplier of the test";
const shortname = "shortnametest";
const inn = "5555555";
const phoneCompany = "7777777777";
const email = "testcompany@testemail.ru";
const description = "test company";
const addressCompany = "China";
const siteUrl = "example.test";
const timeZone = "Europe/Moscow";
const imageFileUuid = "3706d1a1-80ae-4367-be39-af7091373811";
const regionIdCompany = 5;
const companyTypeId = 2;
const companyUuidBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";
var companyUuidNoSupplier = "";
var companyUuidSupplier = "";

// data for component
const parentComponentUuid = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const nameComponent = "M Series Geared Motor";
const nameComponent2 = "X Custom Geared Motor";
const descriptionComponent = "graphqlcomment for component";
const typeAccessIdComponent = 3;
const typeAccessIdComponentPrivate = 1;
const componentTypeId = 2;
const actualStatusIdComponent = 1;
const isStandardComponent = true;
const isStandardComponent0 = false;
const subscribersCount = 1;
const keywordIdsOk = [1,3,5];
const keywordIdsDup = [1,2,3,4,5];
const specIdsOk = [10,30,55];
const specIdsDup = [10,22,30,44,55];
const idErr = 0;
const licenseIdOk = 1;
const licenseIdErr = 2;
const filename1 = "file-test-name 1.pdf";
const filename2 = "file-test-name 2.pdf";
const filename3 = "file-test-name 3.pdf";
const filename4 = "file-test-name 4.pdf";
const filename5 = "file-test-name 5.pdf";
const componentFullDataQuery = ` \
uuid \
parentComponentUuid \
name \
description \
ownerUser { \
  uuid \
  username
  imageFile {
    uuid \
    filename \
    filesize \
    pathFile \
  } \
} \
typeAccessId \
componentType { \
  componentTypeId \
  langId \
  componentType \
} \
actualStatus { \
  actualStatusId \
  langId \
  name \
} \
isStandard \
subscribers \
isFollowed \
updatedAt \
licenses { \
  id \
  name \
  publicationAt \
} \
componentParams { \
  componentUuid \
  param { \
    paramId \
    langId \
    paramname \
  } \
  value \
} \
files { \
  uuid \
  parentFileUuid \
  userUuid \
  filename \
  contentType \
  idExt \
  filesize \
  pathFile \
  createdAt \
  updatedAt \
} \
componentSpecs { \
  spec { \
    specId \
    langId \
    spec \
  } \
  componentUuid \
} \
componentKeywords { \
  id \
  keyword \
} \
componentModifications {  \
  uuid \
  componentUuid \
  parentModificationUuid \
  modificationName \
  description \
  filesetsForProgram { \
    id \
    modificationUuid \
      program { \
        id \
        name \
      } \
  } \
  actualStatus { \
    actualStatusId \
    langId \
    name \
  } \
  updatedAt \
  modificationParams { \
    modificationUuid \
    param { \
      paramId \
      langId \
      paramname \
    } \
    value \
  } \
} \
componentSuppliers { \
  supplier { \
    uuid \
    isSupplier \
    shortname \
  } \
  componentUuid \
} \
`;

const componentsListQuery = ` \
uuid \
name \
description \
ownerUser { \
  username \
  imageFile { \
    pathFile \
  } \
} \
typeAccessId \
componentType { \
  componentType \
} \
actualStatus { \
  name \
} \
isFollowed \
isStandard \
updatedAt \
licenses { \
  keyword \
} \
files { \
  uuid \
  filename \
  pathFile \
} \
componentSuppliers { \
  componentUuid \
  supplier { \
    uuid \
    isSupplier \
    shortname \
  } \
  description \
} \
`;
var componentUuidNoStandard = "";
var componentUuidStandard = "";
var fileUuid1 = "";
var fileUuid2 = "";
var fileUuid3 = "";
var fileUuid4 = "";
var fileUuid5 = "";

// data for component modification
const parentModificationUuid = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const modificationName = "testmodificationcomponent";
const modificationName2 = "test modification component 2";
const descriptionModification = "commentcomponent";
const actualStatusIdModification = 1;
var componentUuidModificationFirst = "";
var componentUuidModificationSecond = "";

// data for param
const paramnameIndexFail = 100;
const paramnameIndex = 2;
const paramnameIndex2 = 9;
const paramname = "Selector";
const paramValueTest = "testparametr";
const paramValueTest2 = "testparametr2";
var paramIdTest = "";

async function cleanupParamComponentDb() {
  return global.knex.raw('DELETE FROM param_to_component WHERE value in (?,?)', [
    paramValueTest,
    paramValueTest2,
  ]);
}

async function cleanupParamModificationDb() {
  return global.knex.raw('DELETE FROM param_to_modification WHERE value in (?,?)', [
    paramValueTest,
    paramValueTest2,
  ]);
}

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname IN (?,?)', [
    orgname,
    orgname2,
  ]);
}

async function cleanupStandardDb() {
  return global.knex.raw('DELETE FROM standard_ref WHERE name in (?)', [
    nameStandard,
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

async function cleanupComponentDb() {
  return global.knex.raw('DELETE FROM component_ref WHERE name in (?,?)', [
    nameComponent,
    nameComponent2,
  ]);
}

async function cleanupComponentModificationDb() {
  return global.knex.raw(
    'DELETE FROM component_modification_list WHERE modification_name in (?,?)',
    [
    modificationName,
    modificationName2,
  ]);
}

describe('component', () => {
  beforeAll(() => {
    cleanupParamComponentDb();
    cleanupParamModificationDb();
    cleanupComponentModificationDb();
    cleanupComponentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
    cleanupTokenDb();
    return cleanupUserDb();
  });
  afterAll(() => {
    cleanupParamComponentDb();
    cleanupParamModificationDb();
    cleanupComponentModificationDb();
    cleanupComponentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
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

  it('/graphql:M registerCompany - OK Supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation newCompany {
         registerCompany( data: {
            orgname: "${orgname}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: "${timeZone}",
            regionId: ${regionIdCompany},
            companyTypeId: ${companyTypeId}
          }) {
            uuid
            shortname
            isSupplier
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany.uuid).toBeNonEmptyString();
    expect(registerCompany.shortname).toBe(shortname);
    expect(registerCompany.isSupplier).toBe(false);
    companyUuidSupplier = registerCompany.uuid;
    done();
    // change supplier status on 1
    await global.knex.raw('UPDATE company_ref SET is_supplier=? WHERE orgname=?', [
      't',
      orgname,
    ]);
  });

  it('/graphql:M registerCompany - OK NoSupplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation newCompany {
         registerCompany( data: {
            orgname: "${orgname2}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: "${timeZone}",
            regionId: ${regionIdCompany},
            companyTypeId: ${companyTypeId}
          }) {
            uuid
            shortname
            isSupplier
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany.uuid).toBeNonEmptyString();
    expect(registerCompany.shortname).toBe(shortname);
    expect(registerCompany.isSupplier).toBe(false);
    companyUuidNoSupplier = registerCompany.uuid;
    done();
  });

  it('/graphql:M registerComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent( data: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isStandard: ${isStandardComponent}
            }) {
                uuid
                name
                description
                actualStatusId
                isStandard
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('registerComponent');
    done();
  });

  it('/graphql:M registerComponent - OK standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerComponent( data: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isStandard: ${isStandardComponent}
            }) {
                uuid
                name
                description
                actualStatusId
                isStandard
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    expect(registerComponent).toContainAllKeys([
      "description", "actualStatusId", "isStandard", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isStandard).toBe(isStandardComponent);
    expect(registerComponent.actualStatusId).toBe(actualStatusIdComponent);
    componentUuidStandard = registerComponent.uuid;
    done();
  });

  it('/graphql:M registerComponent - OK not standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerComponent( data: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent2}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponentPrivate},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isStandard: ${isStandardComponent0}
            }) {
                uuid
                name
                description
                actualStatusId
                isStandard
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    expect(registerComponent).toContainAllKeys([
      "description", "actualStatusId", "isStandard", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent2);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isStandard).toBe(isStandardComponent0);
    expect(registerComponent.actualStatusId).toBe(actualStatusIdComponent);
    componentUuidNoStandard = registerComponent.uuid;
    done();
  });

  it('/graphql:M registerComponent - OK not set component parent', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerComponent( data: {
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponentPrivate},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isStandard: ${isStandardComponent0}
            }) {
                uuid
                name
                description
                actualStatusId
                isStandard
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    expect(registerComponent).toContainAllKeys([
      "description", "actualStatusId", "isStandard", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isStandard).toBe(isStandardComponent0);
    expect(registerComponent.actualStatusId).toBe(actualStatusIdComponent);
    componentUuidNoStandard = registerComponent.uuid;
    done();
  });

  it('/graphql:M registerComponent - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerComponent( data: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isStandard: ${isStandardComponent}
            }) {
                uuid
                name
                description
                actualStatusId
                isStandard
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('registerComponent');
  });

  // Testing adding component keywords
  it('/graphql:M addComponentKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addComponentKeywords');
    done();
  });

  it('/graphql:M addComponentKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { addComponentKeywords },
    } = body;
    expect(addComponentKeywords).toBe(3);
    done();
  });

  it('/graphql:M addComponentKeywords - OK with dublicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { addComponentKeywords },
    } = body;
    expect(addComponentKeywords).toBe(2);
    done();
  });

  it('/graphql:M addComponentKeywords - BadRequest all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: This ids [1, 3, 5] already has"
    );
    expect(body.errors[0].path[0]).toBe('addComponentKeywords');
    done();
  });

  it('/graphql:M addComponentKeywords - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found keywords"
    );
    expect(body.errors[0].path[0]).toBe('addComponentKeywords');
    done();
  });

  it('/graphql:M addComponentKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found acces of the component"
    );
    expect(body.errors[0].path[0]).toBe('addComponentKeywords');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              ${componentFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body.data.component);
    expect(body.data.component.uuid).toBe(componentUuidNoStandard);
    expect(body.data.component.componentKeywords[0].id).toBe(1);
    expect(body.data.component.componentKeywords[0].keyword).toBeNonEmptyString();
    expect(body.data.component.componentKeywords[1].id).toBe(2);
    expect(body.data.component.componentKeywords[1].keyword).toBeNonEmptyString();
    expect(body.data.component.componentKeywords[2].id).toBe(3);
    expect(body.data.component.componentKeywords[2].keyword).toBeNonEmptyString();
    expect(body.data.component.componentKeywords[3].id).toBe(4);
    expect(body.data.component.componentKeywords[3].keyword).toBeNonEmptyString();
    expect(body.data.component.componentKeywords[4].id).toBe(5);
    expect(body.data.component.componentKeywords[4].keyword).toBeNonEmptyString();
    done();
  });

  // Testing delete component keywords
  it('/graphql:M deleteComponentKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentKeywords');
    done();
  });

  it('/graphql:M deleteComponentKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentKeywords=%o', body);
    const {
      data: { deleteComponentKeywords },
    } = body;
    expect(deleteComponentKeywords).toBe(3);
    done();
  });

  it('/graphql:M deleteComponentKeywords - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found keywords"
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentKeywords');
    done();
  });

  it('/graphql:M deleteComponentKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteComponentKeywords(data: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found acces of the component"
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentKeywords');
    done();
  });

  // Testing adding component license
  it('/graphql:M addComponentLicense - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addComponentLicense');
    done();
  });

  it('/graphql:M addComponentLicense - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    const {
      data: { addComponentLicense },
    } = body;
    expect(addComponentLicense).toBe(true);
    done();
  });

  it('/graphql:M addComponentLicense - OK with dublicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: This license for the component is already"
    );
    expect(body.errors[0].path[0]).toBe('addComponentLicense');
    done();
  });

  it('/graphql:M addComponentLicense - Error incorrect id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${idErr}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "Internal Server Error"
    );
    expect(body.errors[0].path[0]).toBe('addComponentLicense');
    done();
  });

  it('/graphql:M addComponentLicense - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${idErr}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found acces of the component"
    );
    expect(body.errors[0].path[0]).toBe('addComponentLicense');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add licenses', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              ${componentFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body.data.component);
    expect(body.data.component.uuid).toBe(componentUuidNoStandard);
    expect(body.data.component.licenses).toBeNonEmptyArray();
    expect(body.data.component.licenses[0].id).toBe(licenseIdOk);
    done();
  });

  // Testing delete component license
  it('/graphql:M deleteComponentLicense - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentLicense');
    done();
  });

  it('/graphql:M deleteComponentLicense - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentLicense=%o', body);
    expect(body.data.deleteComponentLicense).toBe(1);
    done();
  });

  it('/graphql:M deleteComponentLicense - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${idErr}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentLicense=%o', body);
    expect(body.data.deleteComponentLicense).toBe(0);
    done();
  });

  it('/graphql:M deleteComponentLicense - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteComponentLicense(data: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${idErr}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found acces of the component"
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentLicense');
    done();
  });

  // Testing adding component specs
  it('/graphql:M addComponentSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addComponentSpecs');
    done();
  });

  it('/graphql:M addComponentSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { addComponentSpecs },
    } = body;
    expect(addComponentSpecs).toBe(3);
    done();
  });

  it('/graphql:M addComponentSpecs - OK with dublicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { addComponentSpecs },
    } = body;
    expect(addComponentSpecs).toBe(2);
    done();
  });

  it('/graphql:M addComponentSpecs - BadRequest all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: This ids [10, 30, 55] already has"
    );
    expect(body.errors[0].path[0]).toBe('addComponentSpecs');
    done();
  });

  it('/graphql:M addComponentSpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('addComponentSpecs');
    done();
  });

  it('/graphql:M addComponentSpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found acces of the component"
    );
    expect(body.errors[0].path[0]).toBe('addComponentSpecs');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              ${componentFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body.data.component);
    const {
      component: { componentSpecs },
    } = body.data;
    expect(componentSpecs[0].componentUuid).toBe(componentUuidNoStandard);
    expect(componentSpecs[0].spec.specId).toBe(10);
    expect(componentSpecs[0].spec.spec).toBeNonEmptyString();
    expect(componentSpecs[1].spec.specId).toBe(30);
    expect(componentSpecs[1].spec.spec).toBeNonEmptyString();
    expect(componentSpecs[2].spec.specId).toBe(55);
    expect(componentSpecs[2].spec.spec).toBeNonEmptyString();
    expect(componentSpecs[3].spec.specId).toBe(22);
    expect(componentSpecs[3].spec.spec).toBeNonEmptyString();
    expect(componentSpecs[4].componentUuid).toBe(componentUuidNoStandard);
    expect(componentSpecs[4].spec.specId).toBe(44);
    expect(componentSpecs[4].spec.spec).toBeNonEmptyString();
    done();
  });

  // Testing delete component specs
  it('/graphql:M deleteComponentSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentSpecs');
    done();
  });

  it('/graphql:M deleteComponentSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentSpecs=%o', body);
    const {
      data: { deleteComponentSpecs },
    } = body;
    expect(deleteComponentSpecs).toBe(3);
    done();
  });

  it('/graphql:M deleteComponentSpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentSpecs');
    done();
  });

  it('/graphql:M deleteComponentSpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteComponentSpecs(data: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found acces of the component"
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentSpecs');
    done();
  });

  it('/graphql:Q Get full data Component - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query componentQuery{
          component(componentUuid: "${parentComponentUuid}") {
            ${componentFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('component');
    done();
  });

  it('/graphql:Q List components - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query componentsQuery{
          components(componentsUuids: "${componentUuidStandard}") {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('components');
    done();
  });

  // it('/graphql:Q List components - OK Not found', async (done) => {
  //   const response1 = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenFirst}`
  //     )
  //     .send({
  //       query: `query componentsQuery{
  //         components(componentsUuids: "${componentUuidStandard}") {
  //           ${componentsListQuery}
  //         }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql all components=%o', response1.body.data.components);
  //   expect(response1.body.data.components).toBeEmptyArray();
  //   // expect(response1.body.data.components.pop().valueActualStatus).toBe(value_actual_status);
  //   done();
  // });

  it('/graphql:Q List components - OK with componentUuid', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(componentsUuids: "${componentUuidStandard}") {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', response1.body.data.components);
    expect(response1.body.data.components[0].uuid).toBe(componentUuidStandard);
    expect(response1.body.data.components[0].name).toBe(nameComponent);
    done();
  });

  it('/graphql:Q List components - OK for 3 uuids', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectComponentQuery{
          components(componentsUuids: [
            "${parentComponentUuid}",
            "${componentUuidStandard}",
            "${componentUuidNoStandard}",
          ]) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', body.data.components);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(parentComponentUuid);
    expect(body.data.components[0].ownerUser.username).toBeNonEmptyString();
    expect(body.data.components[1].uuid).toBe(componentUuidStandard);
    expect(body.data.components[1].ownerUser.username).toBe(username);
    expect(body.data.components[2].uuid).toBe(componentUuidNoStandard);
    expect(body.data.components[2].ownerUser.username).toBe(username2);
    done();
  });

  it('/graphql:Q List components - OK no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(componentsUuids: "${componentUuidNoStandard}") {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', body.data.components);
    expect(body.data.components).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Component - OK with componentUuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${parentComponentUuid}") {
              ${componentFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body.data.component);
    expect(body.data.component.uuid).toBe(parentComponentUuid);
    expect(body.data.component.ownerUser.uuid).toBeNonEmptyString();
    expect(body.data.component.ownerUser.imageFile.pathFile).toBeNonEmptyString();
    expect(body.data.component.componentType.componentType).toBeNonEmptyString();
    expect(body.data.component.actualStatus.name).toBeNonEmptyString();
    expect(body.data.component.licenses[0].name).toBeNonEmptyString();
    expect(body.data.component.subscribers).toBe(subscribersCount);
    expect(body.data.component.componentParams).toBeNonEmptyArray();
    expect(body.data.component.files).toBeNonEmptyArray();
    expect(body.data.component.componentSpecs).toBeNonEmptyArray();
    expect(body.data.component.componentKeywords).toBeNonEmptyArray();
    expect(body.data.component.componentModifications[0].uuid).toBeNonEmptyString();
    expect(body.data.component.componentModifications[0].componentUuid).toBe(parentComponentUuid);
    expect(body.data.component.componentModifications[0].actualStatus.name).toBeNonEmptyString();
    expect(body.data.component.componentModifications[0].filesetsForProgram[0].program.name).toBeNonEmptyString();
    expect(body.data.component.componentModifications[0].modificationParams).toBeNonEmptyArray();
    expect(body.data.component.componentSuppliers[0].componentUuid).toBe(parentComponentUuid);
    expect(body.data.component.componentSuppliers[0].supplier.shortname).toBeNonEmptyString();
    done();
  });

  // Testing component files
  it('/graphql:Q ComponentFiles - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query componentQuery{
            componentFiles(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('componentFiles');
    done();
  });

  it('/graphql:Q ComponentFiles - OK with parentComponentUuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            componentFiles(componentUuid: "${parentComponentUuid}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body.data);
    const {
      data: { componentFiles },
    } = body;
    expect(componentFiles).toBeNonEmptyArray();
    expect(componentFiles[0].uuid).toBeNonEmptyString();
    expect(componentFiles[0].filename).toBeNonEmptyString();
    expect(componentFiles[0].filesize).toBe(0);
    expect(componentFiles[0].downloadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:M uploadComponentFiles - OK add files 1-5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            uploadComponentFiles(data: {
              filename: [
                "${filename1}",
                "${filename2}",
                "${filename3}",
                "${filename4}",
                "${filename5}"
              ]
              componentUuid: "${componentUuidNoStandard}"
            }){
              filename
              uploadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    const {
      data: { uploadComponentFiles },
    } = body;
    expect(uploadComponentFiles).toBeNonEmptyArray();
    expect(uploadComponentFiles[0].filename).toBe(filename1);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[1].filename).toBe(filename2);
    expect(uploadComponentFiles[1].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[2].filename).toBe(filename3);
    expect(uploadComponentFiles[2].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[3].filename).toBe(filename4);
    expect(uploadComponentFiles[3].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[4].filename).toBe(filename5);
    expect(uploadComponentFiles[4].uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q ComponentFiles - OK 5 files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            componentFiles(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    const {
      data: { componentFiles },
    } = body;
    expect(componentFiles).toBeNonEmptyArray();
    expect(componentFiles[0].uuid).toBeNonEmptyString();
    fileUuid1 = componentFiles[0].uuid;
    expect(componentFiles[0].filename).toBe(filename1);
    expect(componentFiles[0].filename).toBeNonEmptyString();
    expect(componentFiles[0].filesize).toBe(0);
    expect(componentFiles[0].downloadUrl).toBeNonEmptyString();
    fileUuid2 = componentFiles[1].uuid;
    expect(componentFiles[1].filename).toBe(filename2);
    fileUuid3 = componentFiles[2].uuid;
    expect(componentFiles[2].filename).toBe(filename3);
    fileUuid4 = componentFiles[3].uuid;
    expect(componentFiles[3].filename).toBe(filename4);
    fileUuid5 = componentFiles[4].uuid;
    expect(componentFiles[4].uuid).toBeNonEmptyString();
    expect(componentFiles[4].filename).toBe(filename5);
    expect(componentFiles[4].filesize).toBe(0);
    expect(componentFiles[4].downloadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:M deleteComponentFile - OK delete file 1', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteComponentFile(data: {
              fileUuid: "${fileUuid1}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    const {
      data: { deleteComponentFile },
    } = body;
    expect(deleteComponentFile).toBe(true);
    done();
  });

  it('/graphql:M deleteComponentFile - OK delete file 2', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteComponentFile(data: {
              fileUuid: "${fileUuid2}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    const {
      data: { deleteComponentFile },
    } = body;
    expect(deleteComponentFile).toBe(true);
    done();
  });

  it('/graphql:M deleteComponentFile - OK delete file 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteComponentFile(data: {
              fileUuid: "${fileUuid3}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    const {
      data: { deleteComponentFile },
    } = body;
    expect(deleteComponentFile).toBe(true);
    done();
  });

  it('/graphql:M deleteComponentFile - OK delete file 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteComponentFile(data: {
              fileUuid: "${fileUuid4}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    const {
      data: { deleteComponentFile },
    } = body;
    expect(deleteComponentFile).toBe(true);
    done();
  });

  it('/graphql:M deleteComponentFile - OK delete file 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteComponentFile(data: {
              fileUuid: "${fileUuid5}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    const {
      data: { deleteComponentFile },
    } = body;
    expect(deleteComponentFile).toBe(true);
    done();
  });

  it('/graphql:Q ComponentFiles - BadRequest not found files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            componentFiles(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Not found files'
    );
    expect(body.errors[0].path[0]).toBe('componentFiles');
    done();
  });

  it('/graphql:Q ComponentFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            componentFiles(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentFiles');
    done();
  });

  // Testing param component
  it('/graphql:M registerParamComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamComponent( data: {
                componentUuid: "${componentUuidStandard}",
                paramId: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                componentUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('registerParamComponent');
    done();
  });

  it('/graphql:M registerParamComponent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerParamComponent( data: {
                componentUuid: "${componentUuidStandard}",
                paramId: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                componentUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerParamComponent=%o', body);
    const {
      data: { registerParamComponent },
    } = body;
    expect(registerParamComponent).toContainAllKeys([
      "componentUuid", "paramId", "value"
    ]);
    expect(registerParamComponent.id).not.toBeNull();
    expect(registerParamComponent.value).toBe(paramValueTest);
    done();
  });

  it('/graphql:M registerParamComponent - BadRequest duplicate param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerParamComponent( data: {
                componentUuid: "${componentUuidStandard}",
                paramId: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                componentUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      "BadRequest: This param name is already with the component."
    );
    expect(body.errors[0].path[0]).toBe('registerParamComponent');
    done();
  });

  it('/graphql:M registerParamComponent - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerParamComponent( data: {
                componentUuid: "${componentUuidStandard}",
                paramId: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                componentUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('registerParamComponent');
  });

  // Testing component modification
  it('/graphql:M registerComponentModification - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation ComponentModificationQuery {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidNoStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          }) {
            uuid
            modificationName
            description
            updatedAt
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('registerComponentModification');
    done();
  });

  it('/graphql:M registerComponentModification - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation ComponentModificationQuery {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          }) {
            uuid
            componentUuid
            modificationName
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "componentUuid", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.componentUuid).toBe(componentUuidStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    componentUuidModificationFirst = registerComponentModification.uuid;
    done();
  });

  it('/graphql:M registerComponentModification - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation ComponentModificationQuery {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidNoStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          }) {
            uuid
            componentUuid
            modificationName
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "componentUuid", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.componentUuid).toBe(componentUuidNoStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    componentUuidModificationSecond = registerComponentModification.uuid;
    done();
  });

  it('/graphql:M registerComponentModification - OK with parent componentUuidModificationFirst', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation ComponentModificationQuery {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidStandard}",
            parentModificationUuid: "${componentUuidModificationFirst}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          }) {
            uuid
            componentUuid
            modificationName
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "componentUuid", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBe(componentUuidModificationFirst);
    expect(registerComponentModification.componentUuid).toBe(componentUuidStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    done();
  });

  it('/graphql:M registerComponentModification - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation ComponentModificationQuery {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidNoStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          }) {
            uuid
            componentUuid
            modificationName
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('registerComponentModification');
    done();
  });

  // Testing param component modification
  it('/graphql:M registerParamModification - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamModification( data: {
                modificationUuid: "${componentUuidModificationFirst}",
                paramId: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                modificationUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('registerParamModification');
    done();
  });

  it('/graphql:M registerParamModification - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerParamModification( data: {
                modificationUuid: "${componentUuidModificationFirst}",
                paramId: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                modificationUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    const {
      data: { registerParamModification },
    } = body;
    expect(registerParamModification).toContainAllKeys([
      "modificationUuid", "paramId", "value"
    ]);
    expect(registerParamModification.value).toBe(paramValueTest);
    done();
  });

  it('/graphql:M registerParamModification - BadRequest duplicate param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerParamModification( data: {
                modificationUuid: "${componentUuidModificationFirst}",
                paramId: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                modificationUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body =%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      "BadRequest: This param name is already with the modification."
    );
    expect(body.errors[0].path[0]).toBe('registerParamModification');
    done();
  });

  it('/graphql:M registerParamModification - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerParamModification( data: {
                modificationUuid: "${componentUuidModificationFirst}",
                paramId: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                modificationUuid
                paramId
                value
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Not found this modification of you.");
    expect(body.errors[0].path[0]).toBe('registerParamModification');
  });
});
