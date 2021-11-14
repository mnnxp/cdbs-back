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
const typeAccessId2 = 2;
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
const componentNamePut = "componentNamePutUpdate";
const descriptionNamePut = "descriptionNamePutUpdate";
const componentTypeIdPut = 2;
const actualStatusIdPut = 1;

const parentComponentUuid = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const nameComponent = "M Series Geared Motor";
const nameComponent2 = "X Custom Geared Motor";
const descriptionComponent = "graphqlcomment for component";
const typeAccessIdComponent = 3;
const typeAccessIdComponentPrivate = 1;
const componentTypeId = 2;
const actualStatusIdComponent = 1;
const isBaseComponent = true;
const isBaseComponent0 = false;
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
    downloadUrl \
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
isBase \
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
  download { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
  ownerUser { \
    uuid \
    username \
    imageFile { \
      uuid \
      filename \
      filesize \
      downloadUrl \
    } \
  } \
  contentType \
  program { \
    id \
    name \
  } \
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
    uuid \
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
componentStandards { \
 	uuid \
  classifier \
  name \
  description \
  specifiedTolerance \
  publicationAt \
  ownerCompany { \
    uuid \
    shortname \
    inn \
    description \
    imageFile { \
      uuid \
      filename \
      filesize \
      downloadUrl \
    } \
    region { \
      regionId \
      langId \
      region \
    } \
    companyType { \
      companyTypeId \
      langId \
      name \
      shortname \
    } \
    isSupplier \
    isFollowed \
    updatedAt \
  } \
  standardStatus { \
    standardStatusId \
    langId \
    name \
  } \
  updatedAt \
  isFollowed \
} \
`;

const componentsListQuery = ` \
uuid \
name \
description \
ownerUser { \
  username \
  imageFile { \
    uuid \
    filename \
    filesize \
    downloadUrl \
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
isBase \
updatedAt \
licenses { \
  keyword \
} \
files { \
  uuid \
  parentFileUuid \
  download { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
  ownerUser { \
    uuid \
    username \
    imageFile { \
      uuid \
      filename \
      filesize \
      downloadUrl \
    } \
  } \
  contentType \
  program { \
    id \
    name \
  } \
  createdAt \
  updatedAt \
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

const fileDataQuery = `
file { \
  uuid \
  parentFileUuid \
  download { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
  ownerUser { \
    uuid \
    username \
    imageFile { \
      uuid \
      filename \
      filesize \
      downloadUrl \
    } \
  } \
  contentType \
  program { \
    id \
    name \
  } \
  createdAt \
  updatedAt \
} \
`;

var componentUuidNoStandard = "";
var componentUuidStandard = "";
var fileUuid1 = "";
var fileUuid2 = "";
var fileUuid3 = "";
var fileUuid4 = "";
var fileUuid5 = "";

var firstAccess = 1;
var secondAccess = 2;

var langId = 1;
var nameRole = "test role";
var newRoleId = 0;

var nameForUpdate = "new name";
var descriptionForUpdate = "new description";
var typeAccessIdForUpdate = 2;
var componentTypeIdForUpdate = 2;
var actualStatusIdForUpdate = 2;

// data for component modification
const parentModificationUuid = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const baseFilesetUuid = "5de37b5d-75af-4323-b5b4-2cf1e849baa2";
const modificationName = "testmodificationcomponent";
const modificationName2 = "test modification component 2";
const descriptionModification = "commentcomponent";
const actualStatusIdModification = 1;
var componentModificationUuidFirst = "";
var componentModificationUuidSecond = "";
var filesetForProgramUuid = "";
var fileOfFilesetUuid = "";

var nameModificationForUpdate = "new name modification";
var descriptionModificationForUpdate = "new description modification";
var actualStatusModificationIdForUpdate = 2;

// data for param
const paramnameIndexFail = 100;
const paramnameIndex = 2;
const paramnameIndex2 = 9;
const paramname = "Selector";
const paramValueTest = "testparametr";
const paramValueTest2 = "testparametr2";
const paramIdsTest = [1,3,5,7,11];
var paramIdTest = "";

async function cleanupComponentParamDb() {
  return global.knex.raw('DELETE FROM param_to_component WHERE value in (?,?)', [
    paramValueTest,
    paramValueTest2,
  ]);
}

async function cleanupModificationParamDb() {
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
    // cleanupComponentParamDb();
    // cleanupModificationParamDb();
    // cleanupComponentModificationDb();
    cleanupComponentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
    cleanupTokenDb();
    return cleanupUserDb();
  });
  afterAll(() => {
    // cleanupComponentParamDb();
    // cleanupModificationParamDb();
    // cleanupComponentModificationDb();
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
    authorizationUserFirst = registerUser.uuid;
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
    authorizationUserSecond = registerUser.uuid;
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
                isBase: ${isBaseComponent}
            }) {
                uuid
                name
                description
                actualStatusId
                isBase
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
                isBase: ${isBaseComponent}
            }) {
                uuid
                name
                description
                actualStatusId
                isBase
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    componentUuidStandard = registerComponent.uuid;
    expect(registerComponent).toContainAllKeys([
      "description", "actualStatusId", "isBase", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isBase).toBe(isBaseComponent);
    expect(registerComponent.actualStatusId).toBe(actualStatusIdComponent);
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
                name: "${nameComponent2}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponentPrivate},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent0}
            }) {
                uuid
                name
                description
                actualStatusId
                isBase
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
      "description", "actualStatusId", "isBase", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent2);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isBase).toBe(isBaseComponent0);
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
                isBase: ${isBaseComponent0}
            }) {
                uuid
                name
                description
                actualStatusId
                isBase
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
      "description", "actualStatusId", "isBase", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isBase).toBe(isBaseComponent0);
    expect(registerComponent.actualStatusId).toBe(actualStatusIdComponent);
    // componentUuidNoStandard = registerComponent.uuid;
    done();
  });

  // Testing self components seatch
  it('/graphql:Q Components - BadRequest not correct params', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components(arguments: {
            componentsUuids: "${authorizationUserSecond}"
            companyUuid: "${authorizationUserSecond}"
            userUuid: "${authorizationUserSecond}"
          }) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed match arguments'
    );
    expect(body.errors[0].path[0]).toBe('components');
    done();
  });

  it('/graphql:Q Components - OK get self components', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components(arguments: {userUuid: "${authorizationUserSecond}"}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components[0].uuid).toBe(componentUuidNoStandard);
    expect(components[0].name).toBe(nameComponent2);
    expect(components.length).toBe(2);
    done();
  });

  // Testing get user components
  it('/graphql:Q Components - Ok no access (private component)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          components(arguments: {userUuid: "${authorizationUserSecond}"}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Components - OK get user components', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components(arguments: {userUuid: "${authorizationUserFirst}"}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components[0].name).toBe(nameComponent);
    expect(components[0].isFollowed).toBe(false);
    expect(components.length).toBe(1);
    done();
  });

  // Testing favorite components search
  it('/graphql:M ComponentFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            addComponentFav(componentUuid: "${componentUuidStandard}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentFav body=%o', body);
    expect(body.data.addComponentFav).toBe(true);
    done();
  });

  it('/graphql:Q Fav list components - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          components(arguments: {favorite: true}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('components');
    done();
  });

  it('/graphql:Q Fav list components - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components(arguments: {favorite: true}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components[0].name).toBe(nameComponent);
    expect(components[0].isFollowed).toBe(true);
    done();
  });

  // Testing get components from favorite list user
  it('/graphql:Q List components - Ok favorite list by user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {
            userUuid:  "${authorizationUserSecond}"
            favorite:  true
          }) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(componentUuidStandard);
    expect(body.data.components[0].ownerUser.username).toBe(username);
    expect(body.data.components[0].isFollowed).toBe(false);
    expect(body.data.components.length).toBe(1);
    done();
  });

  it('/graphql:M ComponentFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            deleteComponentFav(componentUuid: "${componentUuidStandard}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFav body=%o', body);
    expect(body.data.deleteComponentFav).toBe(true);
    done();
  });

  it('/graphql:Q Fav list components - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components(arguments: {favorite: true}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components).toBeEmptyArray();
    done();
  });

  // Testing update component data
  it('/graphql:M putComponentUpdate - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidNoStandard}"
                name: "${componentNamePut}"
                description: "${descriptionNamePut}"
                componentTypeId: ${componentTypeIdPut}
                actualStatusId: ${actualStatusIdPut}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidNoStandard}"
                name: "${componentNamePut}"
                description: "${descriptionNamePut}"
                componentTypeId: ${componentTypeIdPut}
                actualStatusId: ${actualStatusIdPut}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M putComponentUpdate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidNoStandard}"
                name: "${componentNamePut}"
                description: "${descriptionNamePut}"
                componentTypeId: ${componentTypeIdPut}
                actualStatusId: ${actualStatusIdPut}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentUpdate },
    } = body;
    expect(putComponentUpdate).toBe(4);
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest data has already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidNoStandard}"
                name: "${componentNamePut}"
                description: "${descriptionNamePut}"
                componentTypeId: ${componentTypeIdPut}
                actualStatusId: ${actualStatusIdPut}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The data has already'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M putComponentUpdate - OK return data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentUpdate },
    } = body;
    expect(putComponentUpdate).toBe(4);
    done();
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

  it('/graphql:M addComponentKeywords - OK with duplicate', async (done) => {
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
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addComponentKeywords');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
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
      "BadRequest: Access denied"
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

  it('/graphql:M addComponentLicense - OK with duplicate', async (done) => {
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
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addComponentLicense');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add licenses', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
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
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
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

  it('/graphql:M addComponentSpecs - OK with duplicate', async (done) => {
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
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addComponentSpecs');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
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
      "BadRequest: Access denied"
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
          components(arguments: {componentsUuids: "${componentUuidStandard}"}) {
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

  it('/graphql:Q List components - OK with componentUuid', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {componentsUuids: "${componentUuidStandard}"}) {
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

  it('/graphql:Q List components - OK without params', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          components {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components }
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components[0].name).toBe(nameComponent);
    done();
  });

  it('/graphql:Q List components - OK without params', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components }
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components[0].name).toBe(nameComponent);
    done();
  });

  it('/graphql:Q List components - OK for 2 uuids', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {componentsUuids: [
            "${componentUuidStandard}",
            "${componentUuidNoStandard}",
          ]}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', body.data.components);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(componentUuidStandard);
    expect(body.data.components[0].ownerUser.username).toBe(username);
    expect(body.data.components[1].uuid).toBe(componentUuidNoStandard);
    expect(body.data.components[1].ownerUser.username).toBe(username2);
    done();
  });

  it('/graphql:Q List components - Ok get private component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {componentsUuids: [
            "${componentUuidNoStandard}",
            "${parentComponentUuid}",
          ]}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components).toBeEmptyArray();
    done();
  });

  it('/graphql:Q List components - Ok get without 1 no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {componentsUuids: [
            "${componentUuidStandard}",
            "${componentUuidNoStandard}",
            "${parentComponentUuid}",
          ]}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(componentUuidStandard);
    expect(body.data.components[0].ownerUser.username).toBe(username);
    expect(body.data.components[1].uuid).toBe(componentUuidNoStandard);
    expect(body.data.components[1].ownerUser.username).toBe(username2);
    expect(body.data.components.length).toBe(2);
    done();
  });

  // Testing get components from favorite list user
  it('/graphql:Q List components - Ok not found fav by user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {
            userUuid:  "${authorizationUserSecond}"
            favorite:  true
          }) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    expect(body.data.components).toBeEmptyArray();;
    done();
  });

  // Testing get components by company uuids
  it('/graphql:Q List components - Ok by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {companyUuid: "${companyUuidSupplier}"}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components }
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components[0].name).toBe(nameComponent);
    done();
  });

  it('/graphql:Q List components - Ok no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {componentsUuids: "${componentUuidNoStandard}"}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Component - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidStandard}") {
              ${componentFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body.data.component);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component).toContainAllKeys(
       ["actualStatus", "componentKeywords", "componentModifications",
       "componentParams", "componentSpecs", "componentStandards",
       "componentSuppliers", "componentType", "description", "files", "isBase",
       "isFollowed", "licenses", "name", "ownerUser", "parentComponentUuid",
       "subscribers", "typeAccessId", "updatedAt", "uuid"]
    );
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.parentComponentUuid).toBe(parentComponentUuid);
    expect(component.ownerUser.uuid).toBeNonEmptyString();
    expect(component.ownerUser.imageFile.uuid).toBeNonEmptyString();
    expect(component.componentType.componentType).toBeNonEmptyString();
    expect(component.actualStatus.name).toBeNonEmptyString();
    expect(component.subscribers).toBe(1);
    done();
  });

  // Testing add supplier component
  it('/graphql:M addComponentSupplier - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addComponentSupplier( data: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "description for supplier component",
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addComponentSupplier');
    done();
  });

  it('/graphql:M addComponentSupplier - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addComponentSupplier( data: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "description for supplier component",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql addComponentSupplier=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addComponentSupplier },
    } = body;
    expect(addComponentSupplier).toBe(true);
    done();
  });

  it('/graphql:M addComponentSupplier - BadRequest is not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addComponentSupplier( data: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidNoSupplier}",
                description: "description for supplier component",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The company is not supplier.'
    );
    expect(body.errors[0].path[0]).toBe('addComponentSupplier');
    done();
  });

  it('/graphql:M addComponentSupplier - BadRequest not standard component', async (done) => {
    // change access to public
    await global.knex.raw('UPDATE component_ref SET type_access_id=? WHERE uuid=?', [
      typeAccessIdComponent,
      componentUuidNoStandard,
    ]);
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addComponentSupplier( data: {
                componentUuid: "${componentUuidNoStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "description for supplier component",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The component is not standard.'
    );
    expect(body.errors[0].path[0]).toBe('addComponentSupplier');
    done();
    // return access to private
    await global.knex.raw('UPDATE component_ref SET type_access_id=? WHERE uuid=?', [
      typeAccessIdComponentPrivate,
      componentUuidNoStandard,
    ]);
  });

  it('/graphql:M addComponentSupplier - BadRequest supplier already exists', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addComponentSupplier( data: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "description for supplier component",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: This supplier is already with the component'
    );
    expect(body.errors[0].path[0]).toBe('addComponentSupplier');
    done();
  });

  // Testing get components by company (has one component)
  it('/graphql:Q List components - Ok by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectComponentQuery{
          components(arguments: {companyUuid:  "${companyUuidSupplier}"}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(componentUuidStandard);
    expect(body.data.components[0].ownerUser.username).toBe(username);
    expect(body.data.components.length).toBe(1);
    done();
  });

  it('/graphql:Q Get full data Component - OK check add supplier component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidStandard}") {
              uuid \
              componentSuppliers { \
                supplier { \
                  uuid \
                  isSupplier \
                  shortname \
                } \
                description \
                componentUuid \
              } \
            } \
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.componentSuppliers[0].componentUuid).toBe(componentUuidStandard);
    expect(component.componentSuppliers[0].supplier.uuid).toBeNonEmptyString();
    expect(component.componentSuppliers[0].supplier.isSupplier).toBe(true);
    expect(component.componentSuppliers[0].supplier.shortname).toBeNonEmptyString();
    done();
  });

  // Testing delete suppliers component
  it('/graphql:M deleteSuppliersComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteSuppliersComponent( data: {
                componentUuid: "${componentUuidStandard}",
                companiesUuids: "${companyUuidNoSupplier}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteSuppliersComponent');
    done();
  });

  it('/graphql:M deleteSuppliersComponent - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteSuppliersComponent( data: {
                componentUuid: "${componentUuidStandard}",
                companiesUuids: "${companyUuidSupplier}"
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql deleteSuppliersComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteSuppliersComponent },
    } = body;
    expect(deleteSuppliersComponent).toBe(1);
    done();
  });

  it('/graphql:M deleteSuppliersComponent - Ok not found row', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteSuppliersComponent( data: {
                componentUuid: "${componentUuidStandard}",
                companiesUuids: "${companyUuidSupplier}"
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    const {
      data: { deleteSuppliersComponent },
    } = body;
    expect(deleteSuppliersComponent).toBe(0);
    done();
  });

  it('/graphql:M registerStandard - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            parentStandardUuid: "${parentStandardUuid}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            companyUuid: "${companyUuidSupplier}",
            typeAccessId: ${typeAccessId3},
            standardStatusId: ${standardStatusId},
            regionId: ${regionId}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            standardStatusId
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    standardUuidFirst = registerStandard.uuid;
    expect(registerStandard.uuid).toBeNonEmptyString();
    expect(registerStandard.name).toBe(nameStandard);
    done();
  });

  // Testing add standard component
  it('/graphql:M addStandardToComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addStandardToComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardUuid: "${parentStandardUuid}",
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addStandardToComponent');
    done();
  });

  it('/graphql:M addStandardToComponent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addStandardToComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardUuid: "${parentStandardUuid}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql addStandardToComponent=%o', body);
    const {
      data: { addStandardToComponent },
    } = body;
    expect(addStandardToComponent).toBe(true);
    done();
  });

  it('/graphql:M addStandardToComponent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addStandardToComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardUuid: "${standardUuidFirst}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql addStandardToComponent=%o', body);
    const {
      data: { addStandardToComponent },
    } = body;
    expect(addStandardToComponent).toBe(true);
    done();
  });

  it('/graphql:M addStandardToComponent - BadRequest add duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addStandardToComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardUuid: "${parentStandardUuid}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: This standard is already associated with the component'
    );
    expect(body.errors[0].path[0]).toBe('addStandardToComponent');
    done();
  });

  // true add a standard to non-basic components
  it('/graphql:M addStandardToComponent - Ok not standard component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            addStandardToComponent( data: {
                componentUuid: "${componentUuidNoStandard}",
                standardUuid: "${parentStandardUuid}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    const {
      data: { addStandardToComponent },
    } = body;
    expect(addStandardToComponent).toBe(true);
    done();
  });

  it('/graphql:M addStandardToComponent - BadRequest standard already exists', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addStandardToComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardUuid: "${parentStandardUuid}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: This standard is already associated with the component'
    );
    expect(body.errors[0].path[0]).toBe('addStandardToComponent');
    done();
  });

  it('/graphql:Q Get full data Component - OK check add standard component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidStandard}") {
              uuid
              componentStandards {
                uuid
                classifier
                name
                ownerCompany {
                  uuid
                  shortname
                  region {
                    region
                  }
                  companyType {
                    shortname
                  }
                  isSupplier
                }
                standardStatus {
                  name
                }
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.componentStandards[0].uuid).toBe(standardUuidFirst);
    expect(component.componentStandards[0].name).toBe(nameStandard);
    expect(component.componentStandards[0].ownerCompany.uuid).toBe(companyUuidSupplier);
    expect(component.componentStandards.length).toBe(1);
    done();
  });

  it('/graphql:Q Get full data Component - OK check add standard component', async (done) => {
    await global.knex.raw('UPDATE standard_ref SET type_access_id=? WHERE uuid=?', [
      3,
      parentStandardUuid,
    ]);
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidStandard}") {
              uuid
              componentStandards {
                uuid
                classifier
                name
                ownerCompany {
                  uuid
                  shortname
                  region {
                    region
                  }
                  companyType {
                    shortname
                  }
                  isSupplier
                }
                standardStatus {
                  name
                }
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.componentStandards[0].uuid).toBe(parentStandardUuid);
    expect(component.componentStandards[0].classifier).toBeNonEmptyString();
    expect(component.componentStandards[0].name).toBeNonEmptyString();
    expect(component.componentStandards[0].ownerCompany.uuid).toBeNonEmptyString();
    expect(component.componentStandards[0].ownerCompany.companyType.shortname).toBeNonEmptyString();
    expect(component.componentStandards[0].standardStatus.name).toBeNonEmptyString();
    expect(component.componentStandards[1].uuid).toBe(standardUuidFirst);
    expect(component.componentStandards[1].name).toBe(nameStandard);
    expect(component.componentStandards[1].ownerCompany.uuid).toBe(companyUuidSupplier);
    done();
    // return private type access
    await global.knex.raw('UPDATE standard_ref SET type_access_id=? WHERE uuid=?', [
      1,
      parentStandardUuid,
    ]);
  });

  // Testing get components by standard
  it('/graphql:Q Components - Ok by standard (private standard)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          components(arguments: {
            standardUuid: "${parentStandardUuid}"}
          ) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components.length).toBe(1);
    done();
  });

  it('/graphql:Q Components - OK by standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          components(arguments: {
            standardUuid: "${standardUuidFirst}"}
          ) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { components },
    } = body;
    expect(components[0].uuid).toBe(componentUuidStandard);
    expect(components[0].name).toBe(nameComponent);
    expect(components[0].isFollowed).toBe(false);
    expect(components.length).toBe(1);
    done();
  });

  // Testing delete standards component
  it('/graphql:M deleteStandardsComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteStandardsComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardsUuids: "${parentStandardUuid}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardsComponent');
    done();
  });

  it('/graphql:M deleteStandardsComponent - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteStandardsComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardsUuids: "${parentStandardUuid}"
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql deleteStandardsComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteStandardsComponent },
    } = body;
    expect(deleteStandardsComponent).toBe(1);
    done();
  });

  it('/graphql:M deleteStandardsComponent - Ok not found row', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteStandardsComponent( data: {
                componentUuid: "${componentUuidStandard}",
                standardsUuids: "${parentStandardUuid}"
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    const {
      data: { deleteStandardsComponent },
    } = body;
    expect(deleteStandardsComponent).toBe(0);
    done();
  });

  // it('/graphql:M addStandardToComponent - BadRequest no access', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenSecond}`
  //     )
  //     .send({
  //       query: `mutation  {
  //           addStandardToComponent( data: {
  //               componentUuid: "${componentUuidStandard}",
  //               standardUuid: "${parentStandardUuid}"
  //           })
  //       }`,
  //     })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql  body=%o', body);
  //   const { errors, data } = body;
  //   expect(data).toBeNull();
  //   expect(errors[0].message).toBe("BadRequest: Access denied");
  //   expect(body.errors[0].path[0]).toBe('addStandardToComponent');
  //   done();
  // });

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

  it('/graphql:Q ComponentFiles - BadRequest no access', async (done) => {
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
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentFiles');
    done();
  });

  // it('/graphql:Q ComponentFiles - OK with parentComponentUuid', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenSecond}`
  //     )
  //     .send({
  //         query: `query componentQuery{
  //           componentFiles(componentUuid: "${parentComponentUuid}") {
  //             uuid
  //             filename
  //             filesize
  //             downloadUrl
  //           }
  //         }`,
  //       })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql componentFiles=%o', body);
  //   const {
  //     data: { componentFiles },
  //   } = body;
  //   expect(componentFiles).toBeNonEmptyArray();
  //   expect(componentFiles[0].uuid).toBeNonEmptyString();
  //   expect(componentFiles[0].filename).toBeNonEmptyString();
  //   expect(componentFiles[0].filesize).toBe(0);
  //   expect(componentFiles[0].downloadUrl).toBeNonEmptyString();
  //   done();
  // });

  it('/graphql:M uploadComponentFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
              fileUuid
              filename
              uploadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadComponentFiles');
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
              fileUuid
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
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename1);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[1].filename).toBe(filename2);
    expect(uploadComponentFiles[1].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[2].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[2].filename).toBe(filename3);
    expect(uploadComponentFiles[2].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[3].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[3].filename).toBe(filename4);
    expect(uploadComponentFiles[3].uploadUrl).toBeNonEmptyString();
    expect(uploadComponentFiles[4].fileUuid).toBeNonEmptyString();
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

  // it('/graphql:Q ComponentFiles - BadRequest no access', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenFirst}`
  //     )
  //     .send({
  //         query: `query componentQuery{
  //           componentFiles(componentUuid: "${componentUuidNoStandard}") {
  //             uuid
  //             filename
  //             filesize
  //             downloadUrl
  //           }
  //         }`,
  //       })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql componentFiles=%o', body);
  //   expect(body.data).toBeNull();
  //   expect(body.errors[0].message).toBe("BadRequest: Access denied");
  //   expect(body.errors[0].path[0]).toBe('componentFiles');
  //   done();
  // });

  // Testing add and update param component
  it('/graphql:M putComponentParams - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('putComponentParams');
    done();
  });

  it('/graphql:M putComponentParams - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql putComponentParams=%o', body);
    const {
      data: { putComponentParams },
    } = body;
    expect(putComponentParams).toBe(1);
    done();
  });

  it('/graphql:M putComponentParams - BadRequest duplicate param_id and value', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Duplication of existing data detected'
    );
    expect(body.errors[0].path[0]).toBe('putComponentParams');
    done();
  });

  it('/graphql:M putComponentParams - OK update value', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest2}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql putComponentParams=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentParams },
    } = body;
    expect(putComponentParams).toBe(1);
    done();
  });

  it('/graphql:Q Get full data Component - OK check update component param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              componentParams { \
                componentUuid \
                param { \
                  paramId \
                  langId \
                  paramname \
                } \
                value \
              } \
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    expect(body.data.component.componentParams[0].componentUuid).toBe(componentUuidNoStandard);
    expect(body.data.component.componentParams[0].param.paramId).toBe(paramnameIndex);
    expect(body.data.component.componentParams[0].param.paramname).toBeNonEmptyString();
    expect(body.data.component.componentParams[0].value).toBe(paramValueTest2);
    done();
  });

  // Testing delete param component
  it('/graphql:M deleteComponentParams - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}"
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentParams');
    done();
  });

  it('/graphql:M deleteComponentParams - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}"
                paramIds: ${paramnameIndex}
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql deleteComponentParams=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteComponentParams },
    } = body;
    expect(deleteComponentParams).toBe(1);
    done();
  });

  it('/graphql:M deleteComponentParams - BadRequest not found row for delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteComponentParams( data: {
                componentUuid: "${componentUuidNoStandard}"
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Fail delete rows'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentParams');
    done();
  });

  // it('/graphql:M putComponentParams - BadRequest no access', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenSecond}`
  //     )
  //     .send({
  //       query: `mutation  {
  //           putComponentParams( data: {
  //               componentUuid: "${componentUuidNoStandard}",
  //               paramId: ${paramnameIndex}
  //               value: "${paramValueTest}"
  //           }) {
  //               componentUuid
  //               paramId
  //               value
  //           }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql  body=%o', body);
  //   const { errors, data } = body;
  //   expect(data).toBeNull();
  //   expect(errors[0].message).toBe("BadRequest: Access denied");
  //   expect(body.errors[0].path[0]).toBe('putComponentParams');
  //   done();
  // });

  // Testing component modification
  it('/graphql:M registerComponentModification - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
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
        query: `mutation {
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
    componentModificationUuidFirst = registerComponentModification.uuid;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "componentUuid", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.componentUuid).toBe(componentUuidStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
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
        query: `mutation {
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
    componentModificationUuidSecond = registerComponentModification.uuid;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "componentUuid", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.componentUuid).toBe(componentUuidNoStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    done();
  });

  it('/graphql:M registerComponentModification - OK for componentUuidStandard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidStandard}",
            parentModificationUuid: "${componentModificationUuidFirst}",
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
    expect(registerComponentModification.uuid).not.toBe(componentModificationUuidFirst);
    expect(registerComponentModification.componentUuid).toBe(componentUuidStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    done();
  });

  it('/graphql:M registerComponentModification - BadRequest no access (DEMO)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
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

  // Testing update component modification database
  it('/graphql:M putComponentModificationUpdate - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putComponentModificationUpdate(
              componentModificationUuid: "${componentModificationUuidSecond}"
              data: {
                modificationName: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('putComponentModificationUpdate');
    done();
  });

  it('/graphql:M putComponentModificationUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentModificationUpdate(
              componentModificationUuid: "${componentModificationUuidFirst}"
              data: {
                modificationName: "${nameModificationForUpdate}"
                description: "${descriptionModificationForUpdate}"
                actualStatusId: ${actualStatusModificationIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentModificationUpdate');
    done();
  });

  it('/graphql:M putComponentModificationUpdate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentModificationUpdate(
              componentModificationUuid: "${componentModificationUuidFirst}"
              data: {
                modificationName: "${nameModificationForUpdate}"
                description: "${descriptionModificationForUpdate}"
                actualStatusId: ${actualStatusModificationIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentModificationUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentModificationUpdate },
    } = body;
    expect(putComponentModificationUpdate).toBe(4);
    done();
  });

  it('/graphql:M putComponentModificationUpdate - BadRequest data has already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentModificationUpdate(
              componentModificationUuid: "${componentModificationUuidFirst}"
              data: {
                modificationName: "${nameModificationForUpdate}"
                description: "${descriptionModificationForUpdate}"
                actualStatusId: ${actualStatusModificationIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The data has already'
    );
    expect(body.errors[0].path[0]).toBe('putComponentModificationUpdate');
    done();
  });

  // Testing add and update param component
  it('/graphql:M putModificationParams - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('putModificationParams');
    done();
  });

  it('/graphql:M putModificationParams - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql putModificationParams=%o', body);
    const {
      data: { putModificationParams },
    } = body;
    expect(putModificationParams).toBe(1);
    done();
  });

  it('/graphql:M putModificationParams - BadRequest duplicate param_id and value', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Duplication of existing data detected'
    );
    expect(body.errors[0].path[0]).toBe('putModificationParams');
    done();
  });

  it('/graphql:M putModificationParams - OK update value', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                params: {
                  paramId: ${paramnameIndex}
                  value: "${paramValueTest2}"
                }
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql putModificationParams=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putModificationParams },
    } = body;
    expect(putModificationParams).toBe(1);
    done();
  });

  it('/graphql:Q Get full data Component - OK check update modification param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid \
              componentModifications { \
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
            } \
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].modificationParams[0].modificationUuid).toBe(componentModificationUuidSecond);
    expect(component.componentModifications[0].modificationParams[0].param.paramId).toBe(paramnameIndex);
    expect(component.componentModifications[0].modificationParams[0].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[0].modificationParams[0].value).toBe(paramValueTest2);
    done();
  });

  // Testing delete param component modification
  it('/graphql:M deleteModificationParams - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteModificationParams');
    done();
  });

  it('/graphql:M deleteModificationParams - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                paramIds: ${paramnameIndex}
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql deleteModificationParams=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteModificationParams },
    } = body;
    expect(deleteModificationParams).toBe(1);
    done();
  });

  it('/graphql:M deleteModificationParams - BadRequest not found row for delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteModificationParams( data: {
                modificationUuid: "${componentModificationUuidSecond}",
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Fail delete rows'
    );
    expect(body.errors[0].path[0]).toBe('deleteModificationParams');
    done();
  });

  // it('/graphql:M putModificationParams - BadRequest no access', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenSecond}`
  //     )
  //     .send({
  //       query: `mutation  {
  //           putModificationParams( data: {
  //               modificationUuid: "${componentModificationUuidSecond}",
  //               paramId: ${paramnameIndex},
  //               value: "${paramValueTest}"
  //           }) {
  //               componentUuid
  //               paramId
  //               value
  //           }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql  body=%o', body);
  //   const { errors, data } = body;
  //   expect(data).toBeNull();
  //   expect(errors[0].message).toBe("BadRequest: Access denied");
  //   expect(body.errors[0].path[0]).toBe('putModificationParams');
  //   done();
  // });

  // Testing component modification files
  it('/graphql:Q ModificationFiles - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFiles(modificationUuid: "${componentModificationUuidSecond}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFiles');
    done();
  });

  it('/graphql:Q ModificationFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFiles(modificationUuid: "${parentModificationUuid}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentModificationFiles');
    done();
  });

  it('/graphql:M uploadModificationFiles - OK add files 1-5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            uploadModificationFiles(data: {
              filename: [
                "${filename1}",
                "${filename2}",
                "${filename3}",
                "${filename4}",
                "${filename5}"
              ]
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              filename
              uploadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    const {
      data: { uploadModificationFiles },
    } = body;
    expect(uploadModificationFiles).toBeNonEmptyArray();
    expect(uploadModificationFiles[0].filename).toBe(filename1);
    expect(uploadModificationFiles[0].uploadUrl).toBeNonEmptyString();
    expect(uploadModificationFiles[1].filename).toBe(filename2);
    expect(uploadModificationFiles[1].uploadUrl).toBeNonEmptyString();
    expect(uploadModificationFiles[2].filename).toBe(filename3);
    expect(uploadModificationFiles[2].uploadUrl).toBeNonEmptyString();
    expect(uploadModificationFiles[3].filename).toBe(filename4);
    expect(uploadModificationFiles[3].uploadUrl).toBeNonEmptyString();
    expect(uploadModificationFiles[4].filename).toBe(filename5);
    expect(uploadModificationFiles[4].uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q ModificationFiles - OK 5 files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFiles(modificationUuid: "${componentModificationUuidSecond}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFiles=%o', body);
    const {
      data: { componentModificationFiles },
    } = body;
    expect(componentModificationFiles).toBeNonEmptyArray();
    expect(componentModificationFiles[0].uuid).toBeNonEmptyString();
    fileUuid1 = componentModificationFiles[0].uuid;
    expect(componentModificationFiles[0].filename).toBe(filename1);
    expect(componentModificationFiles[0].filename).toBeNonEmptyString();
    expect(componentModificationFiles[0].filesize).toBe(0);
    expect(componentModificationFiles[0].downloadUrl).toBeNonEmptyString();
    fileUuid2 = componentModificationFiles[1].uuid;
    expect(componentModificationFiles[1].filename).toBe(filename2);
    fileUuid3 = componentModificationFiles[2].uuid;
    expect(componentModificationFiles[2].filename).toBe(filename3);
    fileUuid4 = componentModificationFiles[3].uuid;
    expect(componentModificationFiles[3].filename).toBe(filename4);
    fileUuid5 = componentModificationFiles[4].uuid;
    expect(componentModificationFiles[4].uuid).toBeNonEmptyString();
    expect(componentModificationFiles[4].filename).toBe(filename5);
    expect(componentModificationFiles[4].filesize).toBe(0);
    expect(componentModificationFiles[4].downloadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:M deleteModificationFile - OK delete file 1', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFile(data: {
              fileUuid: "${fileUuid1}"
              modificationUuid: "${componentModificationUuidSecond}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFile=%o', body);
    const {
      data: { deleteModificationFile },
    } = body;
    expect(deleteModificationFile).toBe(true);
    done();
  });

  it('/graphql:M deleteModificationFile - OK delete file 2', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFile(data: {
              fileUuid: "${fileUuid2}"
              modificationUuid: "${componentModificationUuidSecond}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFile=%o', body);
    const {
      data: { deleteModificationFile },
    } = body;
    expect(deleteModificationFile).toBe(true);
    done();
  });

  it('/graphql:M deleteModificationFile - OK delete file 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFile(data: {
              fileUuid: "${fileUuid3}"
              modificationUuid: "${componentModificationUuidSecond}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFile=%o', body);
    const {
      data: { deleteModificationFile },
    } = body;
    expect(deleteModificationFile).toBe(true);
    done();
  });

  it('/graphql:M deleteModificationFile - OK delete file 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFile(data: {
              fileUuid: "${fileUuid4}"
              modificationUuid: "${componentModificationUuidSecond}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFile=%o', body);
    const {
      data: { deleteModificationFile },
    } = body;
    expect(deleteModificationFile).toBe(true);
    done();
  });

  it('/graphql:M deleteModificationFile - OK delete file 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFile(data: {
              fileUuid: "${fileUuid5}"
              modificationUuid: "${componentModificationUuidSecond}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFile=%o', body);
    const {
      data: { deleteModificationFile },
    } = body;
    expect(deleteModificationFile).toBe(true);
    done();
  });

  it('/graphql:Q ModificationFiles - BadRequest not found files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFiles(modificationUuid: "${componentModificationUuidSecond}") {
              uuid
              filename
              filesize
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Not found files'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFiles');
    done();
  });

  // it('/graphql:Q ModificationFiles - BadRequest no access', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenFirst}`
  //     )
  //     .send({
  //         query: `query {
  //           componentModificationFiles(modificationUuid: "${componentModificationUuidSecond}") {
  //             uuid
  //             filename
  //             filesize
  //             downloadUrl
  //           }
  //         }`,
  //       })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql componentModificationFiles=%o', body);
  //   expect(body.data).toBeNull();
  //   expect(body.errors[0].message).toBe("BadRequest: Access denied");
  //   expect(body.errors[0].path[0]).toBe('componentModificationFiles');
  //   done();
  // });

  // Testing component modification fileset
  it('/graphql:Q componentModificationFilesets - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFilesets(
              modificationUuid: "${componentModificationUuidSecond}"
            ) {
              modificationUuid
              uuid
              program {
                id
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesets=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFilesets');
    done();
  });

  it('/graphql:Q componentModificationFilesets - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesets(
              modificationUuid: "${parentModificationUuid}"
            ) {
              modificationUuid
              uuid
              program {
                id
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentModificationFilesets');
    done();
  });

  it('/graphql:M registerModificationFileset - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `mutation {
            registerModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 7
            }){
              modificationUuid
              uuid
              programId
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('registerModificationFileset');
    done();
  });

  it('/graphql:M registerModificationFileset - Ok add fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            registerModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 7
            }){
              modificationUuid
              uuid
              programId
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    const {
      data: { registerModificationFileset },
    } = body;
    filesetForProgramUuid = registerModificationFileset.uuid;
    expect(registerModificationFileset.modificationUuid).toBe(componentModificationUuidSecond);
    expect(registerModificationFileset.uuid).toBeNonEmptyString();
    expect(registerModificationFileset.programId).toBe(7);
    done();
  });

  it('/graphql:M registerModificationFileset - BadRequest duplicate fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            registerModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 7
            }){
              modificationUuid
              uuid
              programId
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      `BadRequest: The modification has a set of files for this program: ${filesetForProgramUuid}`
    );
    expect(body.errors[0].path[0]).toBe('registerModificationFileset');
    done();
  });

  it('/graphql:M registerModificationFileset - Ok add second fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            registerModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 5
            }){
              modificationUuid
              uuid
              programId
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    const {
      data: { registerModificationFileset },
    } = body;
    filesetForProgramUuid = registerModificationFileset.uuid;
    expect(registerModificationFileset.modificationUuid).toBe(componentModificationUuidSecond);
    expect(registerModificationFileset.uuid).toBeNonEmptyString();
    expect(registerModificationFileset.programId).toBe(5);
    done();
  });

  it('/graphql:Q componentModificationFilesets - OK 2 filesets', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesets(
              modificationUuid: "${componentModificationUuidSecond}"
            ) {
              modificationUuid
              uuid
              program {
                id
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesets=%o', body);
    const {
      data: { componentModificationFilesets },
    } = body;
    expect(componentModificationFilesets[0].modificationUuid).toBe(componentModificationUuidSecond);
    expect(componentModificationFilesets[0].uuid).toBeNonEmptyString();
    expect(componentModificationFilesets[0].program.id).toBe(7);
    expect(componentModificationFilesets[0].program.name).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentModificationFilesets - Ok select 2 filesets', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesets(
              modificationUuid: "${componentModificationUuidSecond}"
              programId: [5,7]
            ) {
              modificationUuid
              uuid
              program {
                id
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesets=%o', body);
    const {
      data: { componentModificationFilesets },
    } = body;
    expect(componentModificationFilesets[0].modificationUuid).toBe(componentModificationUuidSecond);
    expect(componentModificationFilesets[0].uuid).toBeNonEmptyString();
    // expect(componentModificationFilesets[0].program.id).toBe(5);
    expect(componentModificationFilesets[0].program.name).toBeNonEmptyString();
    expect(componentModificationFilesets[1].uuid).toBeNonEmptyString();
    // expect(componentModificationFilesets[1].program.id).toBe(7);
    expect(componentModificationFilesets[1].program.name).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentModificationFilesets - Ok found 1', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesets(
              modificationUuid: "${componentModificationUuidSecond}"
              programId: [1,2,5,8]
            ) {
              modificationUuid
              uuid
              program {
                id
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesets=%o', body);
    const {
      data: { componentModificationFilesets },
    } = body;
    componentModificationFilesetsSecond = componentModificationFilesets[0].uuid;
    expect(componentModificationFilesets[0].modificationUuid).toBe(componentModificationUuidSecond);
    expect(componentModificationFilesets[0].uuid).toBeNonEmptyString();
    expect(componentModificationFilesets[0].program.id).toBe(5);
    expect(componentModificationFilesets[0].program.name).toBeNonEmptyString();
    expect(componentModificationFilesets.length).toBe(1);
    done();
  });

  it('/graphql:Q componentModificationFilesets - Ok not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesets(
              modificationUuid: "${componentModificationUuidSecond}"
              programId: [1,2,3]
            ) {
              modificationUuid
              uuid
              program {
                id
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesets=%o', body);
    const {
      data: { componentModificationFilesets },
    } = body;
    expect(componentModificationFilesets).toBeEmptyArray();
    done();
  });

  // Testing component modification file of fileset
  it('/graphql:Q componentModificationFilesOfFileset - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${componentModificationUuidSecond}"
              fileUuids: []
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFilesOfFileset');
    done();
  });

  it('/graphql:Q componentModificationFilesOfFileset - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${baseFilesetUuid}"
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentModificationFilesOfFileset');
    done();
  });

  it('/graphql:Q componentModificationFilesOfFileset - OK empty array', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${componentModificationFilesetsSecond}"
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    expect(componentModificationFilesOfFileset).toBeEmptyArray();
    done();
  });

  it('/graphql:M uploadFilesToFileset - Ok add file to fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            uploadFilesToFileset(
              data: {
                filesetUuid: "${filesetForProgramUuid}"
                filename: [
                  "${filename1}",
                  "${filename2}",
                  "${filename3}",
                  "${filename4}"
                ]
              }
            ){
              fileUuid
              filename
              uploadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    const {
      data: { uploadFilesToFileset },
    } = body;
    fileUuid1 = uploadFilesToFileset[0].fileUuid;
    fileUuid2 = uploadFilesToFileset[1].fileUuid;
    fileUuid3 = uploadFilesToFileset[2].fileUuid;
    fileUuid4 = uploadFilesToFileset[3].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename1);
    expect(uploadFilesToFileset[0].uploadUrl).toBeNonEmptyString();
    expect(uploadFilesToFileset[1].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[1].filename).toBe(filename2);
    expect(uploadFilesToFileset[1].uploadUrl).toBeNonEmptyString();
    expect(uploadFilesToFileset[2].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[2].filename).toBe(filename3);
    expect(uploadFilesToFileset[2].uploadUrl).toBeNonEmptyString();
    expect(uploadFilesToFileset[3].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[3].filename).toBe(filename4);
    expect(uploadFilesToFileset[3].uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentModificationFilesOfFileset - OK ', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${filesetForProgramUuid}"
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    fileOfFilesetUuid = componentModificationFilesOfFileset[0].file.uuid;
    expect(componentModificationFilesOfFileset[0].filesetUuid).toBe(filesetForProgramUuid);
    expect(componentModificationFilesOfFileset[0].file.uuid).toBeNonEmptyString();
    expect(componentModificationFilesOfFileset[0].file.download.filename).toBeNonEmptyString();
    expect(componentModificationFilesOfFileset[0].file.download.downloadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentModificationFilesOfFileset - OK filter fileUuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: ["${fileOfFilesetUuid}"]
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    fileOfFilesetUuid = componentModificationFilesOfFileset[0].file.uuid;
    expect(componentModificationFilesOfFileset[0].filesetUuid).toBe(filesetForProgramUuid);
    expect(componentModificationFilesOfFileset[0].file.uuid).toBeNonEmptyString();
    expect(componentModificationFilesOfFileset[0].file.download.filename).toBeNonEmptyString();
    expect(componentModificationFilesOfFileset[0].file.download.downloadUrl).toBeNonEmptyString();
    done();
  });

  // Testing component modification file of fileset for delete
  it('/graphql:M deleteFilesFromFileset - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `mutation {
            deleteFilesFromFileset(
              data: {
                filesetUuid: "${filesetForProgramUuid}"
                fileUuids: [
                  "${fileUuid1}",
                  "${fileUuid2}",
                  "${fileUuid3}",
                  "${fileUuid4}"
                ]
              }
            )
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteFilesFromFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteFilesFromFileset');
    done();
  });

  it('/graphql:M deleteFilesFromFileset - Ok delete 2 files to fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteFilesFromFileset(
              data: {
                filesetUuid: "${filesetForProgramUuid}"
                fileUuids: [
                  "${fileUuid1}",
                  "${fileUuid4}"
                ]
              }
            )
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteFilesFromFileset=%o', body);
    const {
      data: { deleteFilesFromFileset },
    } = body;
    expect(deleteFilesFromFileset).toBe(true);
    done();
  });

  it('/graphql:M deleteFilesFromFileset - BadRequest delete non-existent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteFilesFromFileset(
              data: {
                filesetUuid: "${filesetForProgramUuid}"
                fileUuids: [
                  "${fileUuid1}",
                  "${fileUuid4}"
                ]
              }
            )
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteFilesFromFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Error with delete files of fileset data'
    );
    expect(body.errors[0].path[0]).toBe('deleteFilesFromFileset');
    done();
  });

  it('/graphql:Q componentModificationFilesOfFileset - OK with parentModificationUuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${filesetForProgramUuid}"
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    expect(componentModificationFilesOfFileset[0].filesetUuid).toBe(filesetForProgramUuid);
    expect(componentModificationFilesOfFileset.length).toBe(2);
    done();
  });

  // Testing component modification fileset for delete
  it('/graphql:M deleteModificationFileset - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `mutation {
            deleteModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              filesetUuid: "${filesetForProgramUuid}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteModificationFileset');
    done();
  });

  it('/graphql:M deleteModificationFileset - Ok delete fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              filesetUuid: "${filesetForProgramUuid}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFileset=%o', body);
    const {
      data: { deleteModificationFileset },
    } = body;
    expect(deleteModificationFileset).toBe(true);
    done();
  });

  it('/graphql:M deleteModificationFileset - BadRequest delete non-existent fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFileset(data: {
              modificationUuid: "${componentModificationUuidSecond}"
              filesetUuid: "${filesetForProgramUuid}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Not found fileset data'
    );
    expect(body.errors[0].path[0]).toBe('deleteModificationFileset');
    done();
  });

  it('/graphql:Q componentModificationFilesOfFileset - BadRequest not found fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesOfFileset(
              filesetUuid: "${filesetForProgramUuid}"
            ) {
              filesetUuid
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Not found fileset data'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFilesOfFileset');
    done();
  });

  // Testing component data  update
  it('/graphql:M putComponentUpdate - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidNoStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M putComponentUpdate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentUpdate },
    } = body;
    expect(putComponentUpdate).toBe(5);
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest data has already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The data has already'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  // add access for authorizationTokenSecond
  it('/graphql:M setUserAccessComponent - OK add low access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                userUuid: "${authorizationUserSecond}"
                typeAccessId: ${secondAccess}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setUserAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setUserAccessComponent },
    } = body;
    expect(setUserAccessComponent).toBe(true);
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest need higher access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M setUserAccessComponent - OK add access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                userUuid: "${authorizationUserSecond}"
                typeAccessId: ${firstAccess}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setUserAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setUserAccessComponent },
    } = body;
    expect(setUserAccessComponent).toBe(true);
    done();
  });

  it('/graphql:Q getUsersListAccessComponent - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            getUsersListAccessComponent(
              componentUuid: "${componentUuidStandard}"
            ) {
              componentUuid
              userUuid
              typeAccess {
                typeAccessId
                langId
                name
              }
              isEnabled
              createdAt
              updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getUsersListAccessComponent=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('getUsersListAccessComponent');
    done();
  });

  it('/graphql:Q getUsersListAccessComponent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            getUsersListAccessComponent(
              componentUuid: "${componentUuidStandard}"
            ) {
              componentUuid
              userUuid
              typeAccess {
                typeAccessId
                langId
                name
              }
              isEnabled
              createdAt
              updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getUsersListAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { getUsersListAccessComponent },
    } = body;
    expect(getUsersListAccessComponent[0].componentUuid).toBe(componentUuidStandard);
    expect(getUsersListAccessComponent[0].userUuid).toBe(authorizationUserSecond);
    expect(getUsersListAccessComponent[0].typeAccess.typeAccessId).toBe(firstAccess);
    done();
  });

  it('/graphql:M putComponentUpdate - OK with access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                name: "rand"
                description: "rand rand rand"
                componentTypeId: 1
                actualStatusId: 2
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentUpdate },
    } = body;
    expect(putComponentUpdate).toBe(4);
    done();
  });

  // disable access for authorizationTokenSecond
  it('/graphql:M deleteUserAccessComponent - OK delete access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                userUuid: "${authorizationUserSecond}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteUserAccessComponent },
    } = body;
    expect(deleteUserAccessComponent).toBe(true);
    done();
  });

  it('/graphql:M deleteUserAccessComponent - BadRequest not found access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                userUuid: "${authorizationUserSecond}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessComponent=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access not found for user'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserAccessComponent');
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  // add access for company
  it('/graphql:M setCompanyAccessComponent - OK add low access company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                companyUuid: "${companyUuidNoSupplier}"
                typeAccessId: ${secondAccess}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setCompanyAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyAccessComponent },
    } = body;
    expect(setCompanyAccessComponent).toBe(true);
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest need higher access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  it('/graphql:M setCompanyAccessComponent - OK add access company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                companyUuid: "${companyUuidNoSupplier}"
                typeAccessId: ${firstAccess}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setCompanyAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyAccessComponent },
    } = body;
    expect(setCompanyAccessComponent).toBe(true);
    done();
  });

  it('/graphql:M registerCompanyRole - OK return already id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerCompanyRole( data: {
              companyUuid: "${companyUuidNoSupplier}"
              langId: ${langId}
              name: "${nameRole}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompanyRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerCompanyRole },
    } = body;
    newRoleId = registerCompanyRole;
    done();
  });

  it('/graphql:M addAccessRole - OK add access role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addAccessRole( data: {
              roleId: ${newRoleId}
              typesAccessIds: 3
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addAccessRole },
    } = body;
    expect(addAccessRole).toBe(true);
    done();
  });

  it('/graphql:M addCompanyMember - OK add company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${newRoleId}
              }
            ) {
              companyUuid
              userUuid
              roleId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addCompanyMember },
    } = body;
    expect(addCompanyMember.companyUuid).toBe(companyUuidNoSupplier);
    expect(addCompanyMember.userUuid).toBe(authorizationUserSecond);
    expect(addCompanyMember.roleId).toBe(newRoleId);
    done();
  });

  it('/graphql:Q getCompaniesListAccessComponent - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            getCompaniesListAccessComponent(
              componentUuid: "${componentUuidStandard}"
            ) {
              componentUuid
              companyUuid
              typeAccess {
                typeAccessId
                langId
                name
              }
              isEnabled
              createdAt
              updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getCompaniesListAccessComponent=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('getCompaniesListAccessComponent');
    done();
  });

  it('/graphql:M addAccessRole - OK add access role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addAccessRole( data: {
              roleId: ${newRoleId}
              typesAccessIds: 1
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addAccessRole },
    } = body;
    expect(addAccessRole).toBe(true);
    done();
  });

  it('/graphql:Q getCompaniesListAccessComponent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            getCompaniesListAccessComponent(
              componentUuid: "${componentUuidStandard}"
            ) {
              componentUuid
              companyUuid
              typeAccess {
                typeAccessId
                langId
                name
              }
              isEnabled
              createdAt
              updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getCompaniesListAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { getCompaniesListAccessComponent },
    } = body;
    expect(getCompaniesListAccessComponent[0].componentUuid).toBe(componentUuidStandard);
    expect(getCompaniesListAccessComponent[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(getCompaniesListAccessComponent[0].typeAccess.typeAccessId).toBe(firstAccess);
    done();
  });

  it('/graphql:M putComponentUpdate - OK with access from company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putComponentUpdate },
    } = body;
    expect(putComponentUpdate).toBe(6);
    done();
  });

  // disable access for authorizationTokenSecond
  it('/graphql:M deleteCompanyAccessComponent - OK delete access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                companyUuid: "${companyUuidNoSupplier}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyAccessComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyAccessComponent },
    } = body;
    expect(deleteCompanyAccessComponent).toBe(true);
    done();
  });

  it('/graphql:M deleteCompanyAccessComponent - BadRequest not found access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyAccessComponent(
              data: {
                componentUuid: "${componentUuidStandard}"
                companyUuid: "${companyUuidNoSupplier}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyAccessComponent=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access not found for company'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyAccessComponent');
    done();
  });

  it('/graphql:M putComponentUpdate - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putComponentUpdate(
              componentUuid: "${componentUuidStandard}"
              data: {
                parentComponentUuid: "${componentUuidStandard}"
                name: "${nameForUpdate}"
                description: "${descriptionForUpdate}"
                componentTypeId: ${componentTypeIdForUpdate}
                actualStatusId: ${actualStatusIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putComponentUpdate');
    done();
  });

  // Testing delete component modification
  it('/graphql:M deleteComponentModification - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteComponentModification(data: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            }) {
              uuid
              componentUuid
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
    expect(body.errors[0].path[0]).toBe('deleteComponentModification');
    done();
  });

  it('/graphql:M deleteComponentModification - BadRequest not owner user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteComponentModification(data: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            }) {
              uuid
              componentUuid
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
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentModification');
    done();
  });

  it('/graphql:M deleteComponentModification - OK standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteComponentModification(data: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            }) {
              uuid
              componentUuid
              modificationName
              description
              updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentModification=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteComponentModification },
    } = body;
    expect(deleteComponentModification).toContainAllKeys([
      "componentUuid", "description", "modificationName", "updatedAt", "uuid"
    ]);
    expect(deleteComponentModification.uuid).toBe(componentModificationUuidFirst);
    expect(deleteComponentModification.modificationName).toBe(nameModificationForUpdate);
    expect(deleteComponentModification.description).toBe(descriptionModificationForUpdate);
    done();
  });

  it('/graphql:M deleteComponentModification - BadRequest not found component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteComponentModification(data: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            }) {
              uuid
              componentUuid
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
      'BadRequest: Failed delete component modification'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponentModification');
    done();
  });

  // Testing change component access
  it('/graphql:M changeComponentAccess - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            changeComponentAccess( data: {
              componentUuid: "${componentUuidNoStandard}"
              newTypeAccessUuid: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeComponentAccess=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeComponentAccess');
    done();
  });

  it('/graphql:M changeComponentAccess - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            changeComponentAccess( data: {
              componentUuid: "${componentUuidNoStandard}"
              newTypeAccessUuid: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeComponentAccess=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeComponentAccess },
    } = body;
    expect(changeComponentAccess).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Component - OK check change access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              ownerUser {
                uuid
              }
              typeAccessId
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.ownerUser.uuid).toBe(authorizationUserSecond);
    expect(component.typeAccessId).toBe(typeAccessId2);
    done();
  });

  // Testing transfer component ownership
  it('/graphql:M transferComponentOwnership - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            transferComponentOwnership( data: {
              componentUuid: "${componentUuidNoStandard}"
              newOwnerUserUuid: "${authorizationUserFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql transferComponentOwnership=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('transferComponentOwnership');
    done();
  });

  it('/graphql:M transferComponentOwnership - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            transferComponentOwnership( data: {
              componentUuid: "${componentUuidNoStandard}"
              newOwnerUserUuid: "${authorizationUserFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql transferComponentOwnership=%o', body);
    // expect(body).toBe(0);
    const {
      data: { transferComponentOwnership },
    } = body;
    expect(transferComponentOwnership).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Component - OK check change owner', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              ownerUser {
                uuid
              }
              typeAccessId
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.ownerUser.uuid).toBe(authorizationUserFirst);
    expect(component.typeAccessId).toBe(typeAccessId2);
    done();
  });

  // Testing delete component
  it('/graphql:M deleteComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteComponent( componentUuid: "${componentUuidStandard}") {
                uuid
                name
                description
                actualStatusId
                isBase
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
    expect(body.errors[0].path[0]).toBe('deleteComponent');
    done();
  });

  it('/graphql:M deleteComponent - BadRequest not owner user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteComponent( componentUuid: "${componentUuidStandard}") {
                uuid
                name
                description
                actualStatusId
                isBase
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed delete component'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponent');
    done();
  });

  it('/graphql:M deleteComponent - OK standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteComponent( componentUuid: "${componentUuidStandard}") {
                uuid
                name
                description
                actualStatusId
                isBase
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteComponent },
    } = body;
    expect(deleteComponent).toContainAllKeys([
      "description", "actualStatusId", "isBase", "name", "updatedAt", "uuid"
    ]);
    expect(deleteComponent.uuid).toBeNonEmptyString();
    expect(deleteComponent.name).toBe(nameComponent);
    expect(deleteComponent.description).toBe(descriptionComponent);
    expect(deleteComponent.isBase).toBe(isBaseComponent);
    expect(deleteComponent.actualStatusId).toBe(actualStatusIdComponent);
    done();
  });

  it('/graphql:M deleteComponent - BadRequest not found component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteComponent( componentUuid: "${componentUuidStandard}") {
                uuid
                name
                description
                actualStatusId
                isBase
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed delete component'
    );
    expect(body.errors[0].path[0]).toBe('deleteComponent');
    done();
  });
});
