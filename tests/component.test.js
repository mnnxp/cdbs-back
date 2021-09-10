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

// data for standard
const uuidStandardParent = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const classifierStandard = "GOST-2012-Test";
const nameStandard = "GOST 2012 Test standard";
const descriptionStandard = "Test GOST standard";
const specifiedTolerance = "C";
const technicalCommittee = "GOST";
const publicationAt = "2021-07-31T00:00:00";
const idTypeAccess3 = 3;
const idTypeAccess1 = 1;
const idStandardStatus = 1;
const idRegion = 5;
var uuidStandardFirst = "";
var uuidStandardSecond = "";

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
const uuidImageFile = "3706d1a1-80ae-4367-be39-af7091373811";
const idRegionCompany = 5;
const idCompanyType = 2;
const uuidCompanyBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";
var uuidCompanyNoSupplier = "";
var uuidCompanySupplier = "";

// data for represent
const idRegionRepresentation = 15;
const idRepresentationType = 1;
const nameRepresentationFirst = "test first additional office";
const nameRepresentationSecond = "test second additional office";
const addressRepresentation = "Fake str, Fantom";
const phoneRepresentation = "+743874487556";
const uuidFake = "2cd385e1-8f7e-4908-8235-dfe42938b888";
const uuidRepresentArray = [];
var uuidCompanyFirst = "";
var uuidRepresentFirst = "";
var uuidRepresentDelete = "";

// data for component
const uuidComponentParent = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const nameComponent = "M Series Geared Motor";
const nameComponent2 = "X Custom Geared Motor";
const descriptionComponent = "graphqlcomment for component";
const idTypeAccessComponent = 3;
const idTypeAccessComponentPrivate = 1;
const idComponentType = 2;
const idActualStatusComponent = 1;
const isStandardComponent = true;
const isStandardComponent0 = false;
const subscribersCount = 1;
const componentFullDataQuery = ` \
uuid \
uuidComponentParent \
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
idTypeAccess \
componentType { \
  idComponentType \
  idLang \
  componentType \
} \
actualStatus { \
  idActualStatus \
  idLang \
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
  uuidComponent \
  param { \
    idParam \
    idLang \
    paramname \
  } \
  value \
} \
files { \
  uuid \
  uuidFileParent \
  uuidUser \
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
    idSpec \
    idLang \
    spec \
  } \
  uuidComponent \
} \
componentKeywords { \
  id \
  keyword \
} \
componentModifications {  \
  uuid \
  uuidComponent \
  uuidModificationParent \
  modificationName \
  description \
  filesetsForProgram { \
    id \
    uuidModification \
      program { \
        id \
        name \
      } \
  } \
  actualStatus { \
    idActualStatus \
    idLang \
    name \
  } \
  updatedAt \
  modificationParams { \
    uuidModification \
    param { \
      idParam \
      idLang \
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
  uuidComponent \
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
idTypeAccess \
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
  uuidComponent \
  supplier { \
    uuid \
    isSupplier \
    shortname \
  } \
  description \
} \
`;
var uuidComponentNoStandard = "";
var uuidComponentStandard = "";

// data for component modification
const uuidModificationParent = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const modificationName = "testmodificationcomponent";
const modificationName2 = "test modification component 2";
const descriptionModification = "commentcomponent";
const idActualStatusModification = 1;
var uuidComponentModificationFirst = "";
var uuidComponentModificationSecond = "";

// data for param
const paramnameIndexFail = 100;
const paramnameIndex = 2;
const paramnameIndex2 = 9;
const paramname = "Selector";
const paramValueTest = "testparametr";
const paramValueTest2 = "testparametr2";
var idParamTest = "";

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

async function cleanupCompanyRepresentDb() {
  return global.knex.raw('DELETE FROM company_represent_ref WHERE name in (?,?)', [
    nameRepresentationFirst,
    nameRepresentationSecond,
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
    cleanupCompanyRepresentDb();
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
    cleanupCompanyRepresentDb();
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
            uuidImageFile: "${uuidImageFile}",
            idRegion: ${idRegionCompany},
            idCompanyType: ${idCompanyType}
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
    uuidCompanySupplier = registerCompany.uuid;
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
            uuidImageFile: "${uuidImageFile}",
            idRegion: ${idRegionCompany},
            idCompanyType: ${idCompanyType}
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
    uuidCompanyNoSupplier = registerCompany.uuid;
    done();
  });

  it('/graphql:M registerComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent( data: {
                uuidComponentParent: "${uuidComponentParent}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                idTypeAccess: ${idTypeAccessComponent},
                idComponentType: ${idComponentType},
                idActualStatus: ${idActualStatusComponent},
                isStandard: ${isStandardComponent}
            }) {
                uuid
                name
                description
                idActualStatus
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
                uuidComponentParent: "${uuidComponentParent}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                idTypeAccess: ${idTypeAccessComponent},
                idComponentType: ${idComponentType},
                idActualStatus: ${idActualStatusComponent},
                isStandard: ${isStandardComponent}
            }) {
                uuid
                name
                description
                idActualStatus
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
      "description", "idActualStatus", "isStandard", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isStandard).toBe(isStandardComponent);
    expect(registerComponent.idActualStatus).toBe(idActualStatusComponent);
    uuidComponentStandard = registerComponent.uuid;
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
                uuidComponentParent: "${uuidComponentParent}",
                name: "${nameComponent2}",
                description: "${descriptionComponent}",
                idTypeAccess: ${idTypeAccessComponentPrivate},
                idComponentType: ${idComponentType},
                idActualStatus: ${idActualStatusComponent},
                isStandard: ${isStandardComponent0}
            }) {
                uuid
                name
                description
                idActualStatus
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
      "description", "idActualStatus", "isStandard", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent2);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isStandard).toBe(isStandardComponent0);
    expect(registerComponent.idActualStatus).toBe(idActualStatusComponent);
    uuidComponentNoStandard = registerComponent.uuid;
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
                idTypeAccess: ${idTypeAccessComponentPrivate},
                idComponentType: ${idComponentType},
                idActualStatus: ${idActualStatusComponent},
                isStandard: ${isStandardComponent0}
            }) {
                uuid
                name
                description
                idActualStatus
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
      "description", "idActualStatus", "isStandard", "name", "updatedAt", "uuid"
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(nameComponent);
    expect(registerComponent.description).toBe(descriptionComponent);
    expect(registerComponent.isStandard).toBe(isStandardComponent0);
    expect(registerComponent.idActualStatus).toBe(idActualStatusComponent);
    uuidComponentNoStandard = registerComponent.uuid;
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
                uuidComponentParent: "${uuidComponentParent}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                idTypeAccess: ${idTypeAccessComponent},
                idComponentType: ${idComponentType},
                idActualStatus: ${idActualStatusComponent},
                isStandard: ${isStandardComponent}
            }) {
                uuid
                name
                description
                idActualStatus
                isStandard
                updatedAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Not found this component of you.");
    expect(body.errors[0].path[0]).toBe('registerComponent');
  });

  it('/graphql:Q Get full data Component - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query componentQuery{
          component(uuidComponent: "${uuidComponentParent}") {
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
          components(componentsUuids: "${uuidComponentStandard}") {
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
  //         components(componentsUuids: "${uuidComponentStandard}") {
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

  it('/graphql:Q List components - OK with uuidComponent', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          components(componentsUuids: "${uuidComponentStandard}") {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', response1.body.data.components);
    expect(response1.body.data.components[0].uuid).toBe(uuidComponentStandard);
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
            "${uuidComponentParent}",
            "${uuidComponentStandard}",
            "${uuidComponentNoStandard}",
          ]) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', body.data.components);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(uuidComponentParent);
    expect(body.data.components[0].ownerUser.username).toBeNonEmptyString();
    expect(body.data.components[1].uuid).toBe(uuidComponentStandard);
    expect(body.data.components[1].ownerUser.username).toBe(username);
    expect(body.data.components[2].uuid).toBe(uuidComponentNoStandard);
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
          components(componentsUuids: "${uuidComponentNoStandard}") {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter components=%o', body.data.components);
    expect(body.data.components).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Component - OK with uuidComponent', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(uuidComponent: "${uuidComponentParent}") {
              ${componentFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body.data.component);
    expect(body.data.component.uuid).toBe(uuidComponentParent);
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
    expect(body.data.component.componentModifications[0].uuidComponent).toBe(uuidComponentParent);
    expect(body.data.component.componentModifications[0].actualStatus.name).toBeNonEmptyString();
    expect(body.data.component.componentModifications[0].filesetsForProgram[0].program.name).toBeNonEmptyString();
    expect(body.data.component.componentModifications[0].modificationParams).toBeNonEmptyArray();
    expect(body.data.component.componentSuppliers[0].uuidComponent).toBe(uuidComponentParent);
    expect(body.data.component.componentSuppliers[0].supplier.shortname).toBeNonEmptyString();
    done();
  });

  // Test param component
  it('/graphql:M registerParamComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamComponent( data: {
                uuidComponent: "${uuidComponentStandard}",
                idParam: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                uuidComponent
                idParam
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
                uuidComponent: "${uuidComponentStandard}",
                idParam: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                uuidComponent
                idParam
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
      "uuidComponent", "idParam", "value"
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
                uuidComponent: "${uuidComponentStandard}",
                idParam: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                uuidComponent
                idParam
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
                uuidComponent: "${uuidComponentStandard}",
                idParam: ${paramnameIndex},
                value: "${paramValueTest}"
            }) {
                uuidComponent
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Not found this component of you.");
    expect(body.errors[0].path[0]).toBe('registerParamComponent');
  });

  // Test component modification
  it('/graphql:M registerComponentModification - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation ComponentModificationQuery {
          registerComponentModification( data: {
            modificationName: "${modificationName}",
            uuidComponent: "${uuidComponentNoStandard}",
            uuidModificationParent: "${uuidModificationParent}",
            description: "${descriptionModification}",
            idActualStatus: ${idActualStatusModification}
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
            uuidComponent: "${uuidComponentStandard}",
            uuidModificationParent: "${uuidModificationParent}",
            description: "${descriptionModification}",
            idActualStatus: ${idActualStatusModification}
          }) {
            uuid
            uuidComponent
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
      ["uuid", "uuidComponent", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.uuidComponent).toBe(uuidComponentStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    uuidComponentModificationFirst = registerComponentModification.uuid;
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
            uuidComponent: "${uuidComponentNoStandard}",
            uuidModificationParent: "${uuidModificationParent}",
            description: "${descriptionModification}",
            idActualStatus: ${idActualStatusModification}
          }) {
            uuid
            uuidComponent
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
      ["uuid", "uuidComponent", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.uuidComponent).toBe(uuidComponentNoStandard);
    expect(registerComponentModification.modificationName).toBe(modificationName);
    expect(registerComponentModification.description).toBe(descriptionModification);
    uuidComponentModificationSecond = registerComponentModification.uuid;
    done();
  });

  it('/graphql:M registerComponentModification - OK with parent uuidComponentModificationFirst', async (done) => {
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
            uuidComponent: "${uuidComponentStandard}",
            uuidModificationParent: "${uuidComponentModificationFirst}",
            description: "${descriptionModification}",
            idActualStatus: ${idActualStatusModification}
          }) {
            uuid
            uuidComponent
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
      ["uuid", "uuidComponent", "modificationName", "description"]
    );
    expect(registerComponentModification.uuid).not.toBe(uuidComponentModificationFirst);
    expect(registerComponentModification.uuidComponent).toBe(uuidComponentStandard);
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
            uuidComponent: "${uuidComponentNoStandard}",
            uuidModificationParent: "${uuidModificationParent}",
            description: "${descriptionModification}",
            idActualStatus: ${idActualStatusModification}
          }) {
            uuid
            uuidComponent
            modificationName
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Not found this component of you.");
    expect(body.errors[0].path[0]).toBe('registerComponentModification');
    done();
  });

  // Test param component modification
  it('/graphql:M registerParamModification - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamModification( data: {
                uuidModification: "${uuidComponentModificationFirst}",
                idParam: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                uuidModification
                idParam
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
                uuidModification: "${uuidComponentModificationFirst}",
                idParam: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                uuidModification
                idParam
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
      "uuidModification", "idParam", "value"
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
                uuidModification: "${uuidComponentModificationFirst}",
                idParam: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                uuidModification
                idParam
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
                uuidModification: "${uuidComponentModificationFirst}",
                idParam: ${paramnameIndex2},
                value: "${paramValueTest}"
            }) {
                uuidModification
                idParam
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
