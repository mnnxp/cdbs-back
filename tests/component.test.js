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
const nameStandard = "GOST 2012 Test standard";
const descriptionStandard = "Test GOST standard";
const publicationAt = "2021-07-31T00:00:00";
const typeAccessId3 = 3;
const typeAccessId2 = 2;
const typeAccessId1 = 1;
const standardStatusId = 1;
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

const descriptionSupplier = "description for supplier component";
const descriptionSupplierNew = "new description supplier";

// data for component
const componentNamePut = "componentNamePutUpdate";
const descriptionNamePut = "descriptionNamePutUpdate";
const componentTypeIdPut = 2;
const actualStatusIdPut = 1;
var componentActualStatusesId1 = "";
var componentActualStatusesId2 = "";
var componentTypesId1 = "";

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
const filename0 = "file-test-name 0.pdf";
const filename1 = "file-test-name 1.pdf";
const filename2 = "file-test-name 2.pdf";
const filename3 = "file-test-name 3.pdf";
const filename4 = "file-test-name 4.pdf";
const filename5 = "file-test-name 5.pdf";
const filename6 = "test-favicon-ru.png";
const filename7 = "company-favicon-org.png";
const filename8 = "test-component-file.pdf";
const filename9 = "test-no-proxy.png";
const filename10 = "certificate.pdf";

const modificationNameM1 = "1/16/20-SS-V";
const descriptionM1 = "Dest 1/ss-v";
const actualStatusIdM1 = 1;
const modificationNameM2 = "2/16/20-TT-V";
const descriptionM2 = "Dest 2/tt-v";
const actualStatusIdM2 = 2;
const modificationNameM3 = "3/16/20-D-V";
const descriptionM3 = "Dest 3/d-v";
const actualStatusIdM3 = 3;
const modificationNameM4 = "4/16/20-D-V";
const descriptionM4 = "Dest 4/d-v";
const actualStatusIdM4 = 4;
const badFilenameComponentFaviconTest = "no image file.pdf";
const goodFilenameComponentFaviconTest = "image file.png";

const componentFullDataQuery = ` \
uuid \
parentComponentUuid \
name \
description \
imageFile {
  uuid \
  hash \
sha256Hash \
  filename \
  filesize \
  downloadUrl \
} \
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
typeAccess { \
  typeAccessId \
  langId \
  name \
} \
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
  filename \
  parentFileUuid \
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
  filesize \
  program { \
    id \
    name \
  } \
  createdAt \
  updatedAt \
} \
componentSpecs { \
  specId \
  langId \
  spec \
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
  name \
  description \
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
imageFile {
  uuid \
  filename \
  filesize \
  downloadUrl \
} \
ownerUser { \
  username \
  imageFile { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
} \
typeAccess { \
  typeAccessId \
  langId \
  name \
} \
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
  filename \
  filesize \
  downloadUrl \
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
uuid \
filename \
parentFileUuid \
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
filesize \
program { \
  id \
  name \
} \
createdAt \
updatedAt \
`;

const downloadFileFields = ` \
uuid \
hash \
sha256Hash \
filename \
filesize \
downloadUrl \
`;

const uploadFileFields = ` \
fileUuid \
filename \
uploadUrl \
`;

var componentModificationFields = `
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
createdAt \
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
`;

const showFileRelatedDataFields = `
uuid \
filename \
parentFileUuid \
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
filesize \
program { \
  id \
  name \
} \
createdAt \
updatedAt \
`;

const showFileRevisionsQuery = ` \
uuid \
filename \
revision \
commitMsg \
parentFileUuid \
ownerUser { \
  username \
} \
`;

const showFilesQuery = ` \
uuid \
filename \
revision \
parentFileUuid \
createdAt \
`;

var componentUuidNoStandard = "";
var componentUuidStandard = "";
var fileUuid1 = "";
var fileUuid2 = "";
var fileUuid3 = "";
var fileUuid4 = "";
var fileUuid5 = "";

var seconRevFileFileTestUuid2 = "";
var threeRevFileFileTestUuid2 = "";
var fourthRevFileFileTestUuid2 = "";
var fifthRevFileFileTestUuid2 = "";
var sixthRevFileFileTestUuid2 = "";
var seventhRevFileFileTestUuid2 = "";

var updatedAtCheckComponent = "";
var updatedAtCheckModification = "";

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

const defaultImageUuid = "bc1c2151-86d0-4656-9c9d-d016dd584297";
const defaultImageHash = "60767c27d985cafa603b4a44ddc5fa8557e5c00d38d27f85b14ed460533e219c";
const defaultImageSha256Hash = "80227309427c1c6a1bea25a4cf3f5ebfc4a66a4d66a9460af214a78487c59abc";
// data for component modification
const parentModificationUuid = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const baseFilesetUuid = "5de37b5d-75af-4323-b5b4-2cf1e849baa2";
const modificationName = "testmodificationcomponent";
const modificationName2 = "test modification component 2";
const descriptionModification = "commentcomponent";
const veryLongDescriptionModification = Array(25100).join('я');
const actualStatusIdModification = 1;
var componentModificationUuidFirst = "";
var componentModificationUuidSecond = "";
var filesetForProgramUuid = "";
var fileOfFilesetUuid = "";

var nameModificationForUpdate = "new name modification";
var descriptionModificationForUpdate = "new description modification";
var actualStatusModificationIdForUpdate = 2;

// data for subscribers
var initialFavCompaniesCount = 0;
var initialFavComponentsCount = 0;

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

// Sets a mark in the database that the file has been uploaded and verified
async function setFileAsUploadedDb(fileUuid) {
  return global.knex.raw('UPDATE file_ref SET is_checked=true, is_hidden=false WHERE uuid=?', [
    fileUuid,
  ]);
}

async function setFlagHiddenAsOldRevDb(fileUuid) {
  return global.knex.raw('UPDATE file_ref SET is_hidden=true WHERE uuid=?', [
    fileUuid,
  ]);
}

async function setFlagDeleteAsOldRevDb(fileUuid) {
  return global.knex.raw('UPDATE file_ref SET is_hidden=true, is_delete=true WHERE uuid=?', [
    fileUuid,
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
    // cleanupTokenDb();
    return cleanupUserDb();
  });
  afterAll(() => {
    // cleanupComponentParamDb();
    // cleanupModificationParamDb();
    // cleanupComponentModificationDb();
    cleanupComponentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
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

  it('/graphql:Q selfData - OK get initial favorites count', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          selfData {
            favCompaniesCount
            favComponentsCount
            favStandardsCount
            favUsersCount
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql selfData=%o', body);
    const {
      data: { selfData },
    } = body;
    initialFavCompaniesCount = selfData.favCompaniesCount;
    initialFavComponentsCount = selfData.favComponentsCount;
    done();
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
         registerCompany(args: {
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
            companyTypeId: ${companyTypeId},
            typeAccessId: ${typeAccessId1}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany).toBeNonEmptyString();
    companyUuidSupplier = registerCompany;
    done();
    // change supplier status on 1
    await global.knex.raw('UPDATE company_ref SET is_supplier=? WHERE orgname=?', [
      't',
      orgname,
    ]);
  });

  it('/graphql:Q selfData - OK check favCompaniesCount', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          selfData {
            favCompaniesCount
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql selfData=%o', body);
    const {
      data: { selfData },
    } = body;
    expect(selfData.favCompaniesCount).toBe(initialFavCompaniesCount);
    done();
  });

  it('/graphql:M addCompanyFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          addCompanyFav(companyUuid: "${companyUuidSupplier}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyFav=%o', body);
    expect(body.data.addCompanyFav).toBe(true);
    done();
  });

  it('/graphql:Q selfData - OK check favCompaniesCount increment', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          selfData {
            favCompaniesCount
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql selfData=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.favCompaniesCount).toBe(initialFavCompaniesCount + 1);
    done();
  });

  it('/graphql:M CompanyFav - Ok delete after add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            deleteCompanyFav(companyUuid: "${companyUuidSupplier}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyFav body=%o', body);
    // expect(body).toBe(0);
    expect(body.data.deleteCompanyFav).toBe(true);
    done();
  });

  it('/graphql:Q selfData - OK check favCompaniesCount decrement', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          selfData {
            favCompaniesCount
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql selfData=%o', body);
    const {
      data: { selfData },
    } = body;
    expect(selfData.favCompaniesCount).toBe(initialFavCompaniesCount);
    done();
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
         registerCompany(args: {
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
            companyTypeId: ${companyTypeId},
            typeAccessId: ${typeAccessId1}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany).toBeNonEmptyString();
    companyUuidNoSupplier = registerCompany;
    done();
  });

  it('/graphql:M registerComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent(args: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('registerComponent');
    done();
  });

  it('/graphql:M registerComponent - BadRequest not access parent component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerComponent(args: {
                parentComponentUuid: "${parentComponentUuid}",
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
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
            registerComponent(args: {
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    componentUuidStandard = registerComponent;
    expect(registerComponent).toBeNonEmptyString();
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
            registerComponent(args: {
                name: "${nameComponent2}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponentPrivate},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent0}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    componentUuidNoStandard = registerComponent;
    expect(registerComponent).toBeNonEmptyString();
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
            registerComponent(args: {
                name: "${nameComponent}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponentPrivate},
                componentTypeId: ${componentTypeId},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent0}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    expect(registerComponent).toBeNonEmptyString();
    // componentUuidNoStandard = registerComponent.uuid;
    done();
  });

  // Testing proxied
  // Proxying for user favicon upload (.cadbase.ru)
  it('/graphql:M uploadFavicon - OK proxy for .cadbase.ru', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.ru')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `mutation {
          uploadFavicon(filename: "${filename6}") {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
      debug('/graphql uploadFavicon proxy .cadbase.ru=%o', body);
    const { uploadFavicon } = body.data;
    // Verifying that uploadUrl contains the correct proxied domain
    expect(uploadFavicon.uploadUrl).toStartWith("https://s3.cadbase.ru/");
    expect(uploadFavicon.filename).toBe(filename6);
    // Marking the file as uploaded for further tests
    await setFileAsUploadedDb(uploadFavicon.fileUuid);
    // Checking the download link via presignedUrl
    const { body: downloadBody } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.ru')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `query {
          presignedUrl(fileUuid: "${uploadFavicon.fileUuid}") {
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
      expect(downloadBody.data.presignedUrl.downloadUrl).toStartWith(
      "https://s3.cadbase.ru/"
    );
    done();
  });

  // Proxying for company favicon upload (.cadbase.org)
  it('/graphql:M uploadCompanyFavicon - OK proxy for .cadbase.org', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.org')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `mutation {
          uploadCompanyFavicon(
            companyUuid: "${companyUuidNoSupplier}"
            filename: "${filename7}"
          ) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
      debug('/graphql uploadCompanyFavicon proxy .cadbase.org=%o', body);
    const { uploadCompanyFavicon } = body.data;
    // Checking proxying of the download link
    expect(uploadCompanyFavicon.uploadUrl).toStartWith("https://s3.cadbase.org/");
    await setFileAsUploadedDb(uploadCompanyFavicon.fileUuid);
    // Verifying download via company data request
    const { body: companyBody } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.org')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `query {
          company(companyUuid: "${companyUuidNoSupplier}") {
            imageFile {
              downloadUrl
            }
          }
        }`,
      })
      .expect(HttpStatus.OK);
      expect(companyBody.data.company.imageFile.downloadUrl).toStartWith(
      "https://s3.cadbase.org/"
    );
    done();
  });

  // Proxying for component files
  it('/graphql:M uploadComponentFiles - OK proxy check', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.ru')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidStandard}"
            filenames: ["${filename8}"]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
      debug('/graphql uploadComponentFiles proxy=%o', body);
    const [fileData] = body.data.uploadComponentFiles;
    // Checking the download link
    expect(fileData.filename).toStartWith(filename8);
    expect(fileData.uploadUrl).toStartWith("https://s3.cadbase.ru/");
    await setFileAsUploadedDb(fileData.fileUuid);
    // Verifying the download link through the component
    const { body: componentBody } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.ru')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `query {
          componentFiles(args: {
            componentUuid: "${componentUuidStandard}"
            filesUuids: ["${fileData.fileUuid}"]
          }) {
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
      const [downloadFile] = componentBody.data.componentFiles;
    expect(downloadFile.downloadUrl).toStartWith("https://s3.cadbase.ru/");
    done();
  });

  // Checking absence of proxy for unknown domain
  it('/graphql:M uploadFavicon - NO proxy for unknown domain', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set('Host', 'api.test-domain.com') // Неизвестный домен
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `mutation {
          uploadFavicon(filename: "${filename9}") {
            fileUuid
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
      debug('/graphql uploadFavicon no proxy=%o', body);
    const { uploadFavicon } = body.data;
    // The link should contain the original S3 endpoint
    expect(uploadFavicon.uploadUrl).toContain(".scw.cloud");
    expect(uploadFavicon.uploadUrl).not.toContain("s3.cadbase.ru");
    expect(uploadFavicon.uploadUrl).not.toContain("s3.cadbase.org");
    await setFileAsUploadedDb(uploadFavicon.fileUuid);
    done();
  });

  // Proxying for user certificates
  it('/graphql:M uploadUserCertificate - OK proxy check', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.org')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `mutation {
          uploadUserCertificate(certData: {
            description: "Test certificate"
            filename: "${filename10}"
          }) {
            fileUuid
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql uploadUserCertificate proxy=%o', body);
    const { uploadUserCertificate } = body.data;
    expect(uploadUserCertificate.uploadUrl).toStartWith("https://s3.cadbase.org/");
    await setFileAsUploadedDb(uploadUserCertificate.fileUuid);
    // Checking via user data request
    const { body: userBody } = await agent
      .post('/graphql')
      .set('Host', 'api.cadbase.org')
      .set('Authorization', `Bearer ${authorizationTokenFirst}`)
      .send({
        query: `query {
          selfData {
            certificates {
              file {
                downloadUrl
              }
            }
          }
        }`,
      })
      .expect(HttpStatus.OK);
      const certificates = userBody.data.selfData.certificates;
    const lastCertificate = certificates[certificates.length - 1];
    expect(lastCertificate.file.downloadUrl).toStartWith("https://s3.cadbase.org/");
    done();
  });

  // Testing self components
  it('/graphql:Q Components - BadRequest not correct params', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          components(args: {
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
          components(args: {userUuid: "${authorizationUserSecond}"}) {
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

  it('/graphql:Q Get full data Component - OK check first (default) modification', async (done) => {
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
    expect(body.data.component.componentModifications[0].uuid).toBeNonEmptyString();
    expect(body.data.component.componentModifications[0].modificationName).toBe('N1');
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
          components(args: {userUuid: "${authorizationUserSecond}"}) {
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
          components(args: {userUuid: "${authorizationUserFirst}"}) {
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

  // Testing favorite components
  it('/graphql:M ComponentFav - Ok delete after auto add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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

  it('/graphql:Q selfData - OK check favComponentsCount decrement', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          selfData {
            favComponentsCount
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql selfData=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.favComponentsCount).toBe(initialFavComponentsCount);
    done();
  });

  it('/graphql:Q Get full data Component - OK check subscribers count', async (done) => {
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
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.parentComponentUuid).toBe(componentUuidStandard);
    expect(component.ownerUser.uuid).toBeNonEmptyString();
    expect(component.ownerUser.imageFile.uuid).toBeNonEmptyString();
    expect(component.componentType.componentType).toBeNonEmptyString();
    expect(component.actualStatus.name).toBeNonEmptyString();
    expect(component.subscribers).toBe(0);
    done();
  });

  it('/graphql:M ComponentFav - Ok add (user 1)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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

  it('/graphql:M ComponentFav - Ok add (user 2)', async (done) => {
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

  it('/graphql:Q selfData - OK check favComponentsCount increment', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          selfData {
            favComponentsCount
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql selfData=%o', body);
    const {
      data: { selfData },
    } = body;
    expect(selfData.favComponentsCount).toBe(initialFavComponentsCount + 1);
    done();
  });

  it('/graphql:Q Get full data Component - OK check subscribers count', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.parentComponentUuid).toBe(componentUuidStandard);
    expect(component.ownerUser.uuid).toBeNonEmptyString();
    expect(component.ownerUser.imageFile.uuid).toBeNonEmptyString();
    expect(component.componentType.componentType).toBeNonEmptyString();
    expect(component.actualStatus.name).toBeNonEmptyString();
    expect(component.subscribers).toBe(2);
    done();
  });

  it('/graphql:M ComponentFav - Ok delete after add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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


  it('/graphql:Q Fav list components - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          components(args: {favorite: true}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          components(args: {favorite: true}) {
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
    expect(components[1].uuid).toBe(componentUuidStandard);
    expect(components[1].name).toBe(nameComponent);
    expect(components[1].isFollowed).toBe(true);
    expect(components.length).toBe(4);
    done();
  });

  it('/graphql:Q Get full data Component - OK check subscribers count', async (done) => {
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
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.parentComponentUuid).toBe(componentUuidStandard);
    expect(component.ownerUser.uuid).toBeNonEmptyString();
    expect(component.ownerUser.imageFile.uuid).toBeNonEmptyString();
    expect(component.componentType.componentType).toBeNonEmptyString();
    expect(component.actualStatus.name).toBeNonEmptyString();
    expect(component.subscribers).toBe(1);
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
          components(args: {
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
    expect(body.data.components[1].uuid).toBe(componentUuidStandard);
    expect(body.data.components[1].ownerUser.username).toBe(username);
    expect(body.data.components[1].isFollowed).toBe(false);
    expect(body.data.components.length).toBe(2); // + 1 default favorite for a new user
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
          components(args: {favorite: true}) {
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
    // expect(components).toBeEmptyArray();
    // 3 favorite - 1 set by default and 2 automatic addition on creation
    expect(components.length).toBe(3);
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
              args: {
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
      'BadRequest: Token not found'
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
              args: {
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
              args: {
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
    expect(putComponentUpdate).toBe(3);
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
              args: {
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
              args: {
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
    expect(putComponentUpdate).toBe(3);
    done();
  });

  // Testing adding component keywords
  it('/graphql:M addComponentKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addComponentKeywords(args: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          addComponentKeywords(args: {
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
          addComponentKeywords(args: {
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
          addComponentKeywords(args: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentKeywords=%o', body);
    const {
      data: { addComponentKeywords },
    } = body;
    expect(addComponentKeywords).toBe(0);
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
          addComponentKeywords(args: {
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
          addComponentKeywords(args: {
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
    expect(body.data.component.imageFile.uuid).toBe(defaultImageUuid);
    expect(body.data.component.imageFile.hash).toBe(defaultImageHash);
    expect(body.data.component.imageFile.sha256Hash).toBe(defaultImageSha256Hash);
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

  // Testing get keywords for component
  it('/graphql:Q componentKeywords - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          componentKeywords(
            componentUuid: "${componentUuidNoStandard}"
          ){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentKeywords');
    done();
  });

  it('/graphql:Q componentKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          componentKeywords(
            componentUuid: "${componentUuidNoStandard}"
          ){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentKeywords');
    done();
  });

  it('/graphql:Q componentKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
          componentKeywords(
            componentUuid: "${componentUuidNoStandard}"
          ){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { componentKeywords },
    } = body;
    expect(componentKeywords[0].id).toBe(1);
    expect(componentKeywords[0].keyword).toBeNonEmptyString();
    expect(componentKeywords[1].id).toBe(2);
    expect(componentKeywords[1].keyword).toBeNonEmptyString();
    expect(componentKeywords[2].id).toBe(3);
    expect(componentKeywords[2].keyword).toBeNonEmptyString();
    expect(componentKeywords[3].id).toBe(4);
    expect(componentKeywords[3].keyword).toBeNonEmptyString();
    expect(componentKeywords[4].id).toBe(5);
    expect(componentKeywords[4].keyword).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentKeywords - OK with paginate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
          componentKeywords(
            componentUuid: "${componentUuidNoStandard}"
            paginate: {
              currentPage: 2
              perPage: 2
            }
          ){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentKeywords },
    } = body;
    expect(componentKeywords.length).toBe(2);
    expect(componentKeywords[0].id).toBe(3);
    expect(componentKeywords[0].keyword).toBeNonEmptyString();
    expect(componentKeywords[1].id).toBe(4);
    expect(componentKeywords[1].keyword).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentKeywords - OK not found keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
          componentKeywords(
            componentUuid: "${componentUuidNoStandard}"
            paginate: {
              currentPage: 2
              perPage: 500
            }
          ){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentKeywords },
    } = body;
    expect(componentKeywords).toBeEmptyArray();
    done();
  });

  // Testing delete component keywords
  it('/graphql:M deleteComponentKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteComponentKeywords(args: {
            componentUuid: "${componentUuidNoStandard}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          deleteComponentKeywords(args: {
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
          deleteComponentKeywords(args: {
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
          deleteComponentKeywords(args: {
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

  // Testing component actual status
  it('/graphql:Q componentActualStatuses - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentActualStatuses {
              actualStatusId
              langId
              name
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentActualStatuses=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('componentActualStatuses');
    done();
  });

  it('/graphql:Q componentActualStatuses - OK full list', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentActualStatuses {
              actualStatusId
              langId
              name
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentActualStatuses=%o', body);
    const {
      data: { componentActualStatuses },
    } = body;
    componentActualStatusesId1 = componentActualStatuses[1].actualStatusId;
    componentActualStatusesId2 = componentActualStatuses[2].actualStatusId;
    expect(componentActualStatuses).toBeNonEmptyArray();
    expect(componentActualStatuses[0].name).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentActualStatuses - OK filter list', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentActualStatuses(
              filter: [
                ${componentActualStatusesId1}
                ${componentActualStatusesId2}
            ]){
              actualStatusId
              langId
              name
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentActualStatuses=%o', body);
    const {
      data: { componentActualStatuses },
    } = body;
    expect(componentActualStatuses).toBeNonEmptyArray();
    expect(componentActualStatuses[0].actualStatusId).toBe(componentActualStatusesId1);
    expect(componentActualStatuses[1].actualStatusId).toBe(componentActualStatusesId2);
    expect(componentActualStatuses.length).toBe(2);
    done();
  });

  it('/graphql:Q componentActualStatuses - OK filter with bad ids', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentActualStatuses(
              filter: [
                -6
                885585
                0
            ]){
              actualStatusId
              langId
              name
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentActualStatuses=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentActualStatuses },
    } = body;
    expect(componentActualStatuses).toBeEmptyArray();
    done();
  });

  // Testing component types
  it('/graphql:Q componentTypes - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentTypes {
              componentTypeId
              langId
              componentType
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentTypes=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('componentTypes');
    done();
  });

  it('/graphql:Q componentTypes - OK full list', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentTypes {
              componentTypeId
              langId
              componentType
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentTypes=%o', body);
    const {
      data: { componentTypes },
    } = body;
    componentTypesId1 = componentTypes[1].componentTypeId;
    expect(componentTypes).toBeNonEmptyArray();
    expect(componentTypes[0].componentType).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentTypes - OK filter list', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentTypes(
              filter: [
                ${componentTypesId1}
            ]){
              componentTypeId
              langId
              componentType
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentTypes=%o', body);
    const {
      data: { componentTypes },
    } = body;
    expect(componentTypes).toBeNonEmptyArray();
    expect(componentTypes[0].componentTypeId).toBe(componentTypesId1);
    expect(componentTypes.length).toBe(1);
    done();
  });

  it('/graphql:Q componentTypes - OK filter with bad ids', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentTypes(
              filter: [
                -6
                885585
                0
            ]){
              componentTypeId
              langId
              componentType
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentTypes=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentTypes },
    } = body;
    expect(componentTypes).toBeEmptyArray();
    done();
  });

  // Testing adding component license
  it('/graphql:M addComponentLicense - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addComponentLicense(args: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          addComponentLicense(args: {
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
          addComponentLicense(args: {
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
          addComponentLicense(args: {
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
          addComponentLicense(args: {
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
    // expect(body).toBe(0);
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
          deleteComponentLicense(args: {
            componentUuid: "${componentUuidNoStandard}"
            licenseId: ${licenseIdOk}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentLicense=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          deleteComponentLicense(args: {
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
          deleteComponentLicense(args: {
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
          deleteComponentLicense(args: {
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
          addComponentSpecs(args: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          addComponentSpecs(args: {
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
          addComponentSpecs(args: {
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
          addComponentSpecs(args: {
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
          addComponentSpecs(args: {
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
          addComponentSpecs(args: {
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
    expect(componentSpecs[0].specId).toBe(10);
    expect(componentSpecs[0].spec).toBeNonEmptyString();
    expect(componentSpecs[1].specId).toBe(22);
    expect(componentSpecs[1].spec).toBeNonEmptyString();
    expect(componentSpecs[2].specId).toBe(30);
    expect(componentSpecs[2].spec).toBeNonEmptyString();
    expect(componentSpecs[3].specId).toBe(44);
    expect(componentSpecs[3].spec).toBeNonEmptyString();
    expect(componentSpecs[4].specId).toBe(55);
    expect(componentSpecs[4].spec).toBeNonEmptyString();
    done();
  });

  // Testing get specs for component
  it('/graphql:Q componentSpecs - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          componentSpecs(
            componentUuid: "${componentUuidNoStandard}"
          ){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentSpecs');
    done();
  });

  it('/graphql:Q componentSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
          componentSpecs(
            componentUuid: "${componentUuidNoStandard}"
          ){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { componentSpecs },
    } = body;
    expect(componentSpecs.length).toBe(5);
    expect(componentSpecs[0].specId).toBe(10);
    expect(componentSpecs[0].spec).toBeNonEmptyString();
    expect(componentSpecs[1].specId).toBe(22);
    expect(componentSpecs[1].spec).toBeNonEmptyString();
    expect(componentSpecs[2].specId).toBe(30);
    expect(componentSpecs[2].spec).toBeNonEmptyString();
    expect(componentSpecs[3].specId).toBe(44);
    expect(componentSpecs[3].spec).toBeNonEmptyString();
    expect(componentSpecs[4].specId).toBe(55);
    expect(componentSpecs[4].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentSpecs - OK with paginate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
          componentSpecs(
            componentUuid: "${componentUuidNoStandard}"
            paginate: {
              currentPage: 4
              perPage: 1
            }
          ){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentSpecs },
    } = body;
    expect(componentSpecs.length).toBe(1);
    expect(componentSpecs[0].specId).toBe(44);
    expect(componentSpecs[0].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentSpecs - OK not found specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
          componentSpecs(
            componentUuid: "${componentUuidNoStandard}"
            paginate: {
              currentPage: 10
              perPage: 50
            }
          ){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentSpecs },
    } = body;
    expect(componentSpecs).toBeEmptyArray();
    done();
  });

  // Testing delete component specs
  it('/graphql:M deleteComponentSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteComponentSpecs(args: {
            componentUuid: "${componentUuidNoStandard}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          deleteComponentSpecs(args: {
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
          deleteComponentSpecs(args: {
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
          deleteComponentSpecs(args: {
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

  it('/Get full data Component - BadRequest Access denied (no token)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query componentQuery{
          component(componentUuid: "${componentUuidNoStandard}") {
            ${componentFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql component=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('component');
    done();
  });

  it('/graphql:Q List components - Ok (no token)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query componentsQuery{
          components(args: {componentsUuids: "${componentUuidStandard}"}) {
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
    expect(components[0].typeAccess.typeAccessId).toBe(typeAccessId3);
    expect(components.length).toBe(1);
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
          components(args: {componentsUuids: "${componentUuidStandard}"}) {
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

  it('/graphql:Q List components - OK without params TokenFirst', async (done) => {
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
    expect(components[components.length-1].uuid).toBe(componentUuidStandard);
    expect(components[components.length-1].name).toBe(nameComponent);
    done();
  });

  it('/graphql:Q List components - OK without params TokenSecond', async (done) => {
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
    expect(components[components.length-1].uuid).toBe(componentUuidStandard);
    expect(components[components.length-1].name).toBe(nameComponent);
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
          components(args: {componentsUuids: [
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
          components(args: {componentsUuids: [
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
          components(args: {componentsUuids: [
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

  it('/graphql:Q List components - Ok get without 2 (no access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query selectComponentQuery{
          components(args: {componentsUuids: [
            "${componentUuidStandard}",
            "${componentUuidNoStandard}",
            "${parentComponentUuid}",
          ]}) {
            ${componentsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    // expect(body).toBe(0);
    debug('/graphql body=%o', body);
    expect(body.data.components).toBeNonEmptyArray();
    expect(body.data.components[0].uuid).toBe(componentUuidStandard);
    expect(body.data.components[0].typeAccess.typeAccessId).toBe(typeAccessId3);
    expect(body.data.components[0].ownerUser.username).toBe(username);
    expect(body.data.components.length).toBe(1);
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
          components(args: {
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
    expect(body.data.components.length).toBe(1); // only 1 favorite - set by default
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
          components(args: {companyUuid: "${companyUuidSupplier}"}) {
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
    expect(components).toBeEmptyArray();
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
          components(args: {componentsUuids: "${componentUuidNoStandard}"}) {
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
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.parentComponentUuid).toBe(parentComponentUuid);
    expect(component.ownerUser.uuid).toBeNonEmptyString();
    expect(component.ownerUser.imageFile.uuid).toBeNonEmptyString();
    expect(component.componentType.componentType).toBeNonEmptyString();
    expect(component.actualStatus.name).toBeNonEmptyString();
    expect(component.subscribers).toBe(0);
    done();
  });

  // Testing add supplier component
  it('/graphql:M addComponentSupplier - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addComponentSupplier(args: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "${descriptionSupplier}",
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            addComponentSupplier(args: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "${descriptionSupplier}",
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
            addComponentSupplier(args: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidNoSupplier}",
                description: "${descriptionSupplier}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The company is not supplier'
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
            addComponentSupplier(args: {
                componentUuid: "${componentUuidNoStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "${descriptionSupplier}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The component is not standard'
    );
    expect(body.errors[0].path[0]).toBe('addComponentSupplier');
    done();
    // return access to private
    await global.knex.raw('UPDATE component_ref SET type_access_id=? WHERE uuid=?', [
      typeAccessIdComponentPrivate,
      componentUuidNoStandard,
    ]);
  });

  it('/graphql:M addComponentSupplier - Ok change description', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addComponentSupplier(args: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "${descriptionSupplierNew}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    // expect(body).toBe(0);
    const {
      data: { addComponentSupplier },
    } = body;
    expect(addComponentSupplier).toBe(true);
    done();
  });

  it('/graphql:Q componentSuppliers - Ok check new description', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentSuppliers(componentUuid:  "${componentUuidStandard}") {
            componentUuid
            supplier {
              uuid
              isSupplier
              shortname
            }
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentSuppliers },
    } = body;
    expect(componentSuppliers).toBeNonEmptyArray();
    expect(componentSuppliers[0].componentUuid).toBe(componentUuidStandard);
    expect(componentSuppliers[0].supplier.uuid).toBe(companyUuidSupplier);
    expect(componentSuppliers[0].description).toBe(descriptionSupplierNew);
    expect(componentSuppliers.length).toBe(1);
    done();
  });

  it('/graphql:M addComponentSupplier - Ok return description', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            addComponentSupplier(args: {
                componentUuid: "${componentUuidStandard}",
                companyUuid: "${companyUuidSupplier}",
                description: "${descriptionSupplier}",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    // expect(body).toBe(0);
    const {
      data: { addComponentSupplier },
    } = body;
    expect(addComponentSupplier).toBe(true);
    done();
  });

  // Testing get components by company (has one component)
  it('/graphql:Q componentSuppliers - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          componentSuppliers(componentUuid:  "${componentUuidNoStandard}") {
            componentUuid
            supplier {
              uuid
              isSupplier
              shortname
            }
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('componentSuppliers');
    done();
  });

  it('/graphql:Q componentSuppliers - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          componentSuppliers(componentUuid:  "${componentUuidNoStandard}") {
            componentUuid
            supplier {
              uuid
              isSupplier
              shortname
            }
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('componentSuppliers');
    done();
  });

  it('/graphql:Q componentSuppliers - Ok without token (no entry)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          componentSuppliers(componentUuid:  "${componentUuidStandard}") {
            componentUuid
            supplier {
              uuid
              isSupplier
              shortname
            }
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentSuppliers },
    } = body;
    expect(componentSuppliers).toBeNonEmptyArray();
    expect(componentSuppliers[0].componentUuid).toBe(componentUuidStandard);
    expect(componentSuppliers[0].supplier.uuid).toBe(companyUuidSupplier);
    expect(componentSuppliers.length).toBe(1);
    done();
  });

  it('/graphql:Q componentSuppliers - Ok by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentSuppliers(componentUuid:  "${componentUuidStandard}") {
            componentUuid
            supplier {
              uuid
              isSupplier
              shortname
            }
            description
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentSuppliers },
    } = body;
    expect(componentSuppliers).toBeNonEmptyArray();
    expect(componentSuppliers[0].componentUuid).toBe(componentUuidStandard);
    expect(componentSuppliers[0].supplier.uuid).toBe(companyUuidSupplier);
    expect(componentSuppliers.length).toBe(1);
    done();
  });

  it('/graphql:Q List components - Ok by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectComponentQuery{
          components(args: {companyUuid:  "${companyUuidSupplier}"}) {
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
            deleteSuppliersComponent(args: {
                componentUuid: "${componentUuidStandard}",
                companiesUuids: "${companyUuidNoSupplier}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteSuppliersComponent(args: {
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
            deleteSuppliersComponent(args: {
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
          registerStandard(args: {
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            publicationAt: "${publicationAt}",
            companyUuid: "${companyUuidSupplier}",
            typeAccessId: ${typeAccessId3},
            standardStatusId: ${standardStatusId},
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard).toBeNonEmptyString();
    standardUuidFirst = registerStandard;
    done();
  });

  // Testing add standard component
  it('/graphql:M addStandardToComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addStandardToComponent(args: {
                componentUuid: "${componentUuidStandard}",
                standardUuid: "${parentStandardUuid}",
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            addStandardToComponent(args: {
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
            addStandardToComponent(args: {
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
            addStandardToComponent(args: {
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
            addStandardToComponent(args: {
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
            addStandardToComponent(args: {
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
              componentStandards (
                paginate: {
                  currentPage: 2
                  perPage: 1
                }
              ) {
                uuid
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

  it('/graphql:Q Get full data Component - OK check add standard component (changed parent type access)', async (done) => {
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
                name
                ownerCompany {
                  uuid
                  shortname
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
    expect(component.componentStandards[0].name).toBeNonEmptyString();
    expect(component.componentStandards[0].ownerCompany.uuid).toBeNonEmptyString();
    expect(component.componentStandards[0].ownerCompany.companyType.shortname).toBeNonEmptyString();
    expect(component.componentStandards[0].standardStatus.name).toBeNonEmptyString();
    expect(component.componentStandards[1].uuid).toBe(standardUuidFirst);
    expect(component.componentStandards[1].name).toBe(nameStandard);
    expect(component.componentStandards[1].ownerCompany.uuid).toBe(companyUuidSupplier);
    // return private type access
    await global.knex.raw('UPDATE standard_ref SET type_access_id=? WHERE uuid=?', [
      1,
      parentStandardUuid,
    ]);
    done();
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
          components(args: {
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
          components(args: {
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
            deleteStandardsComponent(args: {
                componentUuid: "${componentUuidStandard}",
                standardsUuids: "${parentStandardUuid}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteStandardsComponent(args: {
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
            deleteStandardsComponent(args: {
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

  it('/graphql:M uploadComponentFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `mutation {
            uploadComponentFiles(args: {
              filenames: [
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

  it('/graphql:Q Get full data Component - OK set UpdatedAt date', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    updatedAtCheckComponent = component.updatedAt;
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
            uploadComponentFiles(args: {
              filenames: [
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
    fileUuid1 = uploadComponentFiles[0].fileUuid;
    await setFileAsUploadedDb(fileUuid1);
    fileUuid2 = uploadComponentFiles[1].fileUuid;
    await setFileAsUploadedDb(fileUuid2);
    fileUuid3 = uploadComponentFiles[2].fileUuid;
    await setFileAsUploadedDb(fileUuid3);
    fileUuid4 = uploadComponentFiles[3].fileUuid;
    await setFileAsUploadedDb(fileUuid4);
    fileUuid5 = uploadComponentFiles[4].fileUuid;
    await setFileAsUploadedDb(fileUuid5);
    done();
  });

  it('/graphql:M component - Ok check Counting', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filesCount
              paramsCount
              suppliersCount
              standardsCount
              modificationsCount
              componentModifications {
                filesCount
                paramsCount
                filesetsCount
                filesetsForProgram {
                  filesCount
                }
              }
              updatedAt
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql component check count=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.filesCount).toBe(5);
    expect(component.modificationsCount).toBe(1);
    expect(component.paramsCount).toBe(0);
    expect(component.standardsCount).toBe(1);
    expect(component.suppliersCount).toBe(0);
    expect(component.componentModifications.length).toBe(1);
    expect(component.componentModifications[0].filesCount).toBe(0);
    expect(component.componentModifications[0].filesetsCount).toBe(0);
    expect(component.componentModifications[0].filesetsForProgram).toBeEmptyArray();
    expect(component.componentModifications[0].paramsCount).toBe(0);
    expect(component.updatedAt).not.toBe(updatedAtCheckComponent);
    done();
  });

  // Testing get component files
  it('/graphql:Q ComponentFiles - OK 5 files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            componentFiles(args: {
              componentUuid: "${componentUuidNoStandard}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentFiles },
    } = body;
    expect(componentFiles).toBeNonEmptyArray();
    expect(componentFiles[0].uuid).toBe(fileUuid1);
    expect(componentFiles[0].filename).toBe(filename1);
    expect(componentFiles[0].filesize).toBe(0);
    expect(componentFiles[0].downloadUrl).toBeNonEmptyString();
    expect(componentFiles[1].uuid).toBe(fileUuid2);
    expect(componentFiles[1].filename).toBe(filename2);
    expect(componentFiles[2].uuid).toBe(fileUuid3);
    expect(componentFiles[2].filename).toBe(filename3);
    expect(componentFiles[3].uuid).toBe(fileUuid4);
    expect(componentFiles[3].filename).toBe(filename4);
    expect(componentFiles[4].uuid).toBe(fileUuid5);
    expect(componentFiles[4].filename).toBe(filename5);
    expect(componentFiles[4].filesize).toBe(0);
    expect(componentFiles[4].downloadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentFiles - BadRequest not token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          componentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
          }) {
            ${downloadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentFiles');
    done();
  });

  it('/graphql:Q componentFiles - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
          }) {
            ${downloadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentFiles },
    } = body;
    expect(componentFiles[0].uuid).toBe(fileUuid1);
    expect(componentFiles[1].uuid).toBe(fileUuid2);
    expect(componentFiles.length).toBe(5);
    done();
  });

  it('/graphql:Q component - Ok files of component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          component(componentUuid: "${componentUuidNoStandard}") {
            uuid
            name
            files (
              sort: {
                byField: "name"
                asDesc: true
              }
              paginate: {
                currentPage: 2
                perPage: 3
              }
            ) {
              uuid
              filename
              updatedAt
              contentType
            }
            componentModifications {
              modificationName
              files {
                filename
                updatedAt
                contentType
              }
              filesetsForProgram {
                program {
                  name
                }
                files {
                  filename
                  updatedAt
                  contentType
                }
              }
            }
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.files[0].uuid).toBe(fileUuid2);
    expect(component.files[1].uuid).toBe(fileUuid1);
    expect(component.files.length).toBe(2);
    expect(component.componentModifications[0].files).toBeEmptyArray();
    expect(component.componentModifications[0].filesetsForProgram).toBeEmptyArray();
    done();
  });

  it('/graphql:Q componentFiles - Ok filter by uuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filesUuids: "${fileUuid2}"
          }) {
            ${downloadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentFiles },
    } = body;
    expect(componentFiles[0].uuid).toBe(fileUuid2);
    expect(componentFiles.length).toBe(1);
    done();
  });

  it('/graphql:Q componentFiles - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentFiles(args: {
            componentUuid: "${parentComponentUuid}"
          }) {
            ${downloadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('componentFiles');
    done();
  });

  // Testing new revisions
  it('/graphql:M uploadComponentFiles - Ok new revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename0}"
              "${filename2}"
            ]
            commitMsg: "test message"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFiles },
    } = body;
    seconRevFileFileTestUuid = uploadComponentFiles[0].fileUuid;
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename0);
    seconRevFileFileTestUuid2 = uploadComponentFiles[1].fileUuid;
    expect(uploadComponentFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[1].filename).toBe(filename2);
    await setFlagHiddenAsOldRevDb(fileUuid1);
    await setFileAsUploadedDb(seconRevFileFileTestUuid);
    await setFlagHiddenAsOldRevDb(fileUuid2);
    await setFileAsUploadedDb(seconRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revision for new file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    // expect(showFileRevisions[0].uuid).toBe(fileUuid1);
    expect(showFileRevisions[0].filename).toBe(filename0);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[0].commitMsg).toBe("test message");
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  it('/graphql:M uploadComponentFiles - Ok new revision 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename2}"
            ]
            commitMsg: "Very long message to test the length limit of 225 characters. It's important to remember that char represents a Unicode Scalar Value, and may not match your idea of what a 'character' is. Iteration over grapheme clusters may be what you actually want."
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFiles },
    } = body;
    threeRevFileFileTestUuid2 = uploadComponentFiles[0].fileUuid;
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename2);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${threeRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[1].commitMsg).toBe("test message");
    expect(showFileRevisions[2].uuid).toBe(threeRevFileFileTestUuid2);
    expect(showFileRevisions[2].filename).toBe(filename2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[2].commitMsg).toBe("Very long message to test the length limit of 225 characters. It's important to remember that char represents a Unicode Scalar Value, and may not match your idea of what a 'character' is. Iteration over grapheme clusters m...");
    expect(showFileRevisions.length).toBe(3);
    await setFlagDeleteAsOldRevDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadComponentFiles - Ok new revision 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFiles },
    } = body;
    fourthRevFileFileTestUuid2 = uploadComponentFiles[0].fileUuid;
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename2);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(fourthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3/4 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fourthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for hidden file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for delete file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${threeRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for stranger file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fourthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:M uploadComponentFiles - BadRequest stranger component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadComponentFiles');
    done();
  });

  // Testing change active revision for file
  it('/graphql:M changeActiveFileRevision - Ok set revision 2 as active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeActiveFileRevision },
    } = body;
    expect(changeActiveFileRevision).toBe(true);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revisions for new active file revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest set remove revision as active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${threeRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest already active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest stranger Component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M uploadComponentFiles - Ok new revision 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFiles },
    } = body;
    fifthRevFileFileTestUuid2 = uploadComponentFiles[0].fileUuid;
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename2);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 4/5 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fifthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[3].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(4);
    expect(showFileRevisions.length).toBe(4);
    done();
  });

  it('/graphql:Q Get all files of Component - OK check parent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentFilesList(
            args:{componentUuid: "${componentUuidNoStandard}"}
            sort:{byField: "filename"}
          ){
            ${showFilesQuery}
          }
        }`,
      })
    .expect(HttpStatus.OK)
    debug('/graphql componentFilesList=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentFilesList },
    } = body;
    expect(componentFilesList[1].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(componentFilesList[1].filename).toBe(filename2);
    expect(componentFilesList[1].revision).toBe(4);
    // expect(componentFilesList[1].parentFileUuid).toBe(seconRevFileFileTestUuid2);
    expect(componentFilesList[1].parentFileUuid).toBe(fourthRevFileFileTestUuid2);
    expect(componentFilesList.length).toBe(5);
    await setFlagHiddenAsOldRevDb(fileUuid2);
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFlagHiddenAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagHiddenAsOldRevDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadComponentFiles - Ok new revision 6 other versions are hidden', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFiles },
    } = body;
    sixthRevFileFileTestUuid2 = uploadComponentFiles[0].fileUuid;
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename2);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 5/6 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${sixthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[3].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(4);
    expect(showFileRevisions[4].uuid).toBe(sixthRevFileFileTestUuid2);
    expect(showFileRevisions[4].revision).toBe(5);
    expect(showFileRevisions.length).toBe(5);
    await setFlagDeleteAsOldRevDb(fileUuid2);
    await setFlagDeleteAsOldRevDb(seconRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fifthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadComponentFiles - Ok new revision 7 other versions are deleted', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFiles(args: {
            componentUuid: "${componentUuidNoStandard}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFiles },
    } = body;
    seventhRevFileFileTestUuid2 = uploadComponentFiles[0].fileUuid;
    expect(uploadComponentFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadComponentFiles[0].filename).toBe(filename2);
    expect(uploadComponentFiles[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(seventhRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 1/7 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seventhRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(seventhRevFileFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  // Testing change main image (favicon) for component
  it('/graphql:M uploadComponentFavicon - BadRequest not access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFavicon(args: {
            componentUuid: "${componentUuidStandard}"
            filename: "${badFilenameComponentFaviconTest}"
          }) {
            ${uploadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFavicon=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadComponentFavicon');
    done();
  });

  it('/graphql:M uploadComponentFavicon - BadRequest not image', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadComponentFavicon(args: {
            componentUuid: "${componentUuidStandard}"
            filename: "${badFilenameComponentFaviconTest}"
          }) {
            ${uploadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFavicon=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Selected file is not image'
    );
    expect(body.errors[0].path[0]).toBe('uploadComponentFavicon');
    done();
  });

  it('/graphql:M uploadComponentFavicon - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadComponentFavicon(args: {
            componentUuid: "${componentUuidStandard}"
            filename: "${goodFilenameComponentFaviconTest}"
          }) {
            ${uploadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadComponentFavicon=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadComponentFavicon },
    } = body;
    changeComponentFaviconTestUuid = uploadComponentFavicon.fileUuid;
    await setFileAsUploadedDb(changeComponentFaviconTestUuid);
    expect(uploadComponentFavicon.fileUuid).toBeNonEmptyString();
    expect(uploadComponentFavicon.filename).toBe(goodFilenameComponentFaviconTest);
    expect(uploadComponentFavicon.uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q List components - OK check change main image', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          components (args: {
            componentsUuids: "${componentUuidStandard}"
          }) {
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
    expect(components[0].imageFile.uuid).toBe(changeComponentFaviconTestUuid);
    expect(components[0].imageFile.filename).toBe(goodFilenameComponentFaviconTest);
    expect(components.length).toBe(1);
    done();
  });

  it('/graphql:M uploadComponentFavicon - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadComponentFavicon(args: {
            componentUuid: "${componentUuidStandard}"
            filename: "${badFilenameComponentFaviconTest}"
          }) {
            ${uploadFileFields}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  // Testing component files list
  it('/graphql:Q componentFilesList - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentFilesList(args:{
              componentUuid: "${componentUuidNoStandard}"
            }){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFilesList=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentFilesList');
    done();
  });

  it('/graphql:Q componentFilesList - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentFilesList(args:{
              componentUuid: "${parentComponentUuid}"
            }){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentFilesList');
    done();
  });

  it('/graphql:Q componentFilesList - OK 5 files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentFilesList(
              args:{componentUuid: "${componentUuidNoStandard}"}
              sort:{byField: "filename"}
            ){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFilesList=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentFilesList },
    } = body;
    expect(componentFilesList[0].uuid).toBe(seconRevFileFileTestUuid);
    expect(componentFilesList[0].filename).toBe(filename0);
    expect(componentFilesList[0].filesize).toBe(0);
    expect(componentFilesList[1].uuid).toBe(seventhRevFileFileTestUuid2);
    expect(componentFilesList[1].filename).toBe(filename2);
    expect(componentFilesList[2].uuid).toBe(fileUuid3);
    expect(componentFilesList[2].filename).toBe(filename3);
    expect(componentFilesList[3].uuid).toBe(fileUuid4);
    expect(componentFilesList[3].filename).toBe(filename4);
    expect(componentFilesList[4].uuid).toBe(fileUuid5);
    expect(componentFilesList[4].filename).toBe(filename5);
    expect(componentFilesList[4].filesize).toBe(0);
    expect(componentFilesList.length).toBe(5);
    done();
  });

  it('/graphql:Q componentFilesList - OK limit offset (added hash and download url fields)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentFilesList(
              args:{componentUuid: "${componentUuidNoStandard}"}
              sort:{
                byField: "filename"
                asDesc: false
              }
              paginate: {
                currentPage: 2
                perPage: 2
              }
            ){
              ${showFileRelatedDataFields}
              hash
            sha256Hash
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFilesList=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentFilesList },
    } = body;
    expect(componentFilesList).toBeNonEmptyArray();
    expect(componentFilesList[0].uuid).toBe(fileUuid3);
    expect(componentFilesList[0].filename).toBe(filename3);
    expect(componentFilesList[0].hash).toBe("");
    expect(componentFilesList[0].sha256Hash).toBe("");
    expect(componentFilesList[0].downloadUrl).toBeNonEmptyString();
    expect(componentFilesList[1].uuid).toBe(fileUuid4);
    expect(componentFilesList[1].filename).toBe(filename4);
    expect(componentFilesList[1].hash).toBe("");
    expect(componentFilesList[1].sha256Hash).toBe("");
    expect(componentFilesList[1].downloadUrl).toBeNonEmptyString();
    expect(componentFilesList.length).toBe(2);
    done();
  });

  it('/graphql:M deleteComponentFile - OK delete file 0', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteComponentFile(args: {
              fileUuid: "${seconRevFileFileTestUuid}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteComponentFile },
    } = body;
    expect(deleteComponentFile).toBe(true);
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
            deleteComponentFile(args: {
              fileUuid: "${fileUuid1}"
              componentUuid: "${componentUuidNoStandard}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFile=%o', body);
    // expect(body).toBe(0);
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
            deleteComponentFile(args: {
              fileUuid: "${seventhRevFileFileTestUuid2}"
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
            deleteComponentFile(args: {
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
            deleteComponentFile(args: {
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
            deleteComponentFile(args: {
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

  it('/graphql:Q ComponentFiles - Ok not found files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query componentQuery{
            componentFiles(args: {
              componentUuid: "${componentUuidNoStandard}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentFiles=%o', body);
    const {
      data: { componentFiles },
    } = body;
    expect(componentFiles).toBeEmptyArray();
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
            putComponentParams(args: {
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
      'BadRequest: Token not found'
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
            putComponentParams(args: {
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
            putComponentParams(args: {
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
      'BadRequest: Duplication of existing data was detected'
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
            putComponentParams(args: {
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
            deleteComponentParams(args: {
                componentUuid: "${componentUuidNoStandard}"
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteComponentParams(args: {
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
            deleteComponentParams(args: {
                componentUuid: "${componentUuidNoStandard}"
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Cannot delete rows'
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
  //           putComponentParams(args: {
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
          registerComponentModification(args: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidNoStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
          registerComponentModification(args: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    componentModificationUuidFirst = registerComponentModification;
    expect(registerComponentModification).toBeNonEmptyString();
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
          registerComponentModification(args: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidNoStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    componentModificationUuidSecond = registerComponentModification;
    expect(registerComponentModification).toBeNonEmptyString();
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
          registerComponentModification(args: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidStandard}",
            parentModificationUuid: "${componentModificationUuidFirst}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    expect(registerComponentModification).toBeNonEmptyString();
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
          registerComponentModification(args: {
            modificationName: "${modificationName}",
            componentUuid: "${componentUuidNoStandard}",
            parentModificationUuid: "${parentModificationUuid}",
            description: "${descriptionModification}",
            actualStatusId: ${actualStatusIdModification}
          })
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
              args: {
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
      'BadRequest: Token not found'
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
              args: {
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

  it('/graphql:M putComponentModificationUpdate - BadRequest very long description', async (done) => {
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
              args: {
                modificationName: "${nameModificationForUpdate}"
                description: "${veryLongDescriptionModification}"
                actualStatusId: ${actualStatusModificationIdForUpdate}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putComponentModificationUpdate=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Text must be less than 50000 bit (~25000 symbols)'
    );
    expect(body.errors[0].path[0]).toBe('putComponentModificationUpdate');
    done();
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
              args: {
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
    expect(putComponentModificationUpdate).toBe(3);
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
              args: {
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

  // Testing query create multiple component modifications
  it('/graphql:M registerComponentModificationsBulk - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          registerComponentModificationsBulk(args: {
            componentUuid: "${componentUuidStandard}",
            modificationsData: [
              {
                modificationName: "${modificationNameM1}",
                description: "${descriptionM1}",
                actualStatusId: ${actualStatusIdM1}
                parameters: []
              },
              {
                modificationName: "${modificationNameM2}",
                description: "${descriptionM2}",
                actualStatusId: ${actualStatusIdM2}
                parameters: []
              }
            ]
          })
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe(
        'BadRequest: Access denied'
      );
      expect(body.errors[0].path[0]).toBe('registerComponentModificationsBulk');
      done();
  });

  it('/graphql:M registerComponentModificationsBulk - OK with duplicate parameters (they have been omitted)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          registerComponentModificationsBulk(args: {
            componentUuid: "${componentUuidStandard}",
            modificationsData: [
              {
                modificationName: "${modificationNameM3}",
                description: "${descriptionM3}",
                actualStatusId: ${actualStatusIdM3},
                parameters: [
                  { paramId: 26, value: "par26" },
                  { paramId: 26, value: "par26 is duplicate" },
                ]
              }
            ]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerComponentModificationsBulk },
    } = body;
    expect(registerComponentModificationsBulk[0]).toBeNonEmptyString();
    expect(registerComponentModificationsBulk.length).toBe(1);
    done();
  });

  it('/graphql:M registerComponentModificationsBulk - OK with params', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          registerComponentModificationsBulk(args: {
            componentUuid: "${componentUuidStandard}",
            modificationsData: [
              {
                modificationName: "${modificationNameM1}",
                description: "${descriptionM1}",
                actualStatusId: ${actualStatusIdM1},
                parameters: [
                  { paramId: 4, value: "par4" },
                  { paramId: 6, value: "par6" },
                  { paramId: 7, value: "par7" },
                  { paramId: 8, value: "par8" },
                  { paramId: 9, value: "par9" },
                  { paramId: 11, value: "par11" },
                  { paramId: 12, value: "par12" },
                  { paramId: 13, value: "par13" },
                ]
              },
              {
                modificationName: "${modificationNameM2}",
                description: "${descriptionM2}",
                actualStatusId: ${actualStatusIdM2},
                parameters: []
              },
              {
                modificationName: "${modificationNameM3}",
                description: "${descriptionM3}",
                actualStatusId: ${actualStatusIdM3},
                parameters: [
                  { paramId: 20, value: "par20" },
                  { paramId: 21, value: "par21" },
                ]
              },
              {
                modificationName: "${modificationNameM4}",
                description: "${descriptionM4}",
                actualStatusId: ${actualStatusIdM4},
                parameters: [
                  { paramId: 20, value: "par20" },
                  { paramId: 21, value: "par21" },
                  { paramId: 22, value: "par22" },
                  { paramId: 23, value: "par23" },
                  { paramId: 24, value: "par24" },
                  { paramId: 10, value: "par10" },
                  { paramId: 11, value: "par11" },
                  { paramId: 12, value: "par12" },
                  { paramId: 13, value: "par13" },
                ]
              }
            ]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerComponentModificationsBulk },
    } = body;
    expect(registerComponentModificationsBulk[0]).toBeNonEmptyString();
    expect(registerComponentModificationsBulk.length).toBe(3);
    done();
  });

  // Testing add and update param component
  it('/graphql:M putModificationParams - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putModificationParams(args: {
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
      'BadRequest: Token not found'
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
            putModificationParams(args: {
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
            putModificationParams(args: {
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
      'BadRequest: Duplication of existing data was detected'
    );
    expect(body.errors[0].path[0]).toBe('putModificationParams');
    done();
  });

  it('/graphql:Q Get full data Component - OK set UpdatedAt component and modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    updatedAtCheckComponent = component.updatedAt;
    updatedAtCheckModification = component.componentModifications.updatedAt;
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
            putModificationParams(args: {
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

  it('/graphql:Q Get full data Component - OK check UpdatedAt after updated param in modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    expect(component.updatedAt).not.toBe(updatedAtCheckComponent);
    expect(component.componentModifications[0].updatedAt).not.toBe(updatedAtCheckModification);
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
    expect(component.componentModifications[1].modificationParams[0].modificationUuid).toBe(componentModificationUuidSecond);
    expect(component.componentModifications[1].modificationParams[0].param.paramId).toBe(paramnameIndex);
    expect(component.componentModifications[1].modificationParams[0].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[1].modificationParams[0].value).toBe(paramValueTest2);
    expect(component.componentModifications.length).toBe(2);
    done();
  });

  it('/graphql:Q Get full data Component - OK check update modification param', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query componentQuery{
            component(componentUuid: "${componentUuidStandard}") {
              uuid \
              componentModifications (
                  sort:{
                    byField: "name"
                    asDesc: false
                  }
              ){ \
                modificationName \
                modificationParams(
                  sort:{
                    byField: "paramId"
                    asDesc: true
                  }
                ){ \
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
    expect(component.uuid).toBe(componentUuidStandard);
    expect(component.componentModifications.length).toBe(7);
    expect(component.componentModifications[0].modificationName).toBe("N1");
    expect(component.componentModifications[3].modificationName).toBe(modificationNameM1);
    expect(component.componentModifications[3].modificationParams.length).toBe(8);
    expect(component.componentModifications[3].modificationParams[0].param.paramId).toBe(13);
    expect(component.componentModifications[3].modificationParams[0].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[3].modificationParams[0].value).toBe("par13");
    expect(component.componentModifications[3].modificationParams[7].param.paramId).toBe(4);
    expect(component.componentModifications[3].modificationParams[7].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[3].modificationParams[7].value).toBe("par4");
    expect(component.componentModifications[4].modificationName).toBe(modificationNameM2);
    expect(component.componentModifications[4].modificationParams.length).toBe(0);
    expect(component.componentModifications[1].modificationName).toBe(modificationNameM3);
    expect(component.componentModifications[1].modificationParams.length).toBe(1);
    expect(component.componentModifications[1].modificationParams[0].param.paramId).toBe(26);
    expect(component.componentModifications[1].modificationParams[0].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[1].modificationParams[0].value).toBe("par26");
    expect(component.componentModifications[2].modificationName).toBe(modificationNameM4);
    expect(component.componentModifications[2].modificationParams.length).toBe(9);
    expect(component.componentModifications[2].modificationParams[0].param.paramId).toBe(24);
    expect(component.componentModifications[2].modificationParams[0].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[2].modificationParams[0].value).toBe("par24");
    expect(component.componentModifications[2].modificationParams[8].param.paramId).toBe(10);
    expect(component.componentModifications[2].modificationParams[8].param.paramname).toBeNonEmptyString();
    expect(component.componentModifications[2].modificationParams[8].value).toBe("par10");
    expect(component.componentModifications[5].modificationParams).toBeEmptyArray();
    expect(component.componentModifications[6].modificationParams).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Component - OK check filter modification', async (done) => {
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
              componentModifications(filter: ["${componentModificationUuidSecond}"]) { \
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
    expect(component.componentModifications.length).toBe(1);
    done();
  });

  // Testing get component modification
  it('/graphql:Q componentModifications - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModifications(
              componentUuid: "${componentUuidNoStandard}"
            ){
              ${componentModificationFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentModifications');
    done();
  });

  it('/graphql:Q componentModifications - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentModifications(
              componentUuid: "${componentUuidNoStandard}"
            ){
              ${componentModificationFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentModifications');
    done();
  });

  it('/graphql:Q componentModifications - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModifications(
              componentUuid: "${componentUuidNoStandard}"
            ){
              ${componentModificationFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModifications },
    } = body;
    expect(componentModifications[0].componentUuid).toBe(componentUuidNoStandard);
    expect(componentModifications[1].modificationParams[0].modificationUuid).toBe(componentModificationUuidSecond);
    expect(componentModifications[1].modificationParams[0].param.paramId).toBe(paramnameIndex);
    expect(componentModifications[1].modificationParams[0].param.paramname).toBeNonEmptyString();
    expect(componentModifications[1].modificationParams[0].value).toBe(paramValueTest2);
    done();
  });

  it('/graphql:Q componentModifications - OK filter', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentModifications(
              componentUuid: "${componentUuidStandard}"
              filter: ["${componentModificationUuidSecond}"]
            ){
              ${componentModificationFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModifications },
    } = body;
    expect(componentModifications.length).toBe(0);
    done();
  });

  it('/graphql:Q componentModifications - OK filter', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentModifications(
              componentUuid: "${componentUuidStandard}"
              filter: ["${componentModificationUuidFirst}"]
            ){
              ${componentModificationFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModifications },
    } = body;
    expect(componentModifications.length).toBe(1);
    expect(componentModifications[0].componentUuid).toBe(componentUuidStandard);
    expect(componentModifications[0].uuid).toBe(componentModificationUuidFirst);
    expect(componentModifications[0].modificationName).toBe(nameModificationForUpdate);
    done();
  });

  it('/graphql:Q componentModifications - OK sort and paginate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query {
            componentModifications(
              componentUuid: "${componentUuidStandard}"
              sort:{
                byField: "name"
                asDesc: false
              }
              paginate: {
                currentPage: 3
                perPage: 2
              }
            ){
              ${componentModificationFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModifications },
    } = body;
    expect(componentModifications.length).toBe(2);
    expect(componentModifications[0].componentUuid).toBe(componentUuidStandard);
    expect(componentModifications[0].modificationName).toBe(modificationNameM2);
    expect(componentModifications[1].modificationName).toBe(nameModificationForUpdate);
    done();
  });

  // Testing delete param component modification
  it('/graphql:M deleteModificationParams - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteModificationParams(args: {
                modificationUuid: "${componentModificationUuidSecond}",
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteModificationParams(args: {
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
            deleteModificationParams(args: {
                modificationUuid: "${componentModificationUuidSecond}",
                paramIds: [${paramIdsTest}]
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - body =%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Cannot delete rows'
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
  //           putModificationParams(args: {
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
  it('/graphql:Q ModificationFiles - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFiles(args:{
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
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
            componentModificationFiles(args:{
              modificationUuid: "${parentModificationUuid}"
            }){
              ${downloadFileFields}
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

  it('/graphql:Q Get full data Component - OK set UpdatedAt date', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    updatedAtCheckComponent = component.updatedAt;
    updatedAtCheckModification = component.componentModifications.updatedAt;
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
            uploadModificationFiles(args: {
              filenames: [
                "${filename1}",
                "${filename2}",
                "${filename3}",
                "${filename4}",
                "${filename5}"
              ]
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              fileUuid
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
    fileUuid1 = uploadModificationFiles[0].fileUuid;
    fileUuid2 = uploadModificationFiles[1].fileUuid;
    fileUuid3 = uploadModificationFiles[2].fileUuid;
    fileUuid4 = uploadModificationFiles[3].fileUuid;
    fileUuid5 = uploadModificationFiles[4].fileUuid;
    await setFileAsUploadedDb(fileUuid1);
    await setFileAsUploadedDb(fileUuid2);
    await setFileAsUploadedDb(fileUuid3);
    await setFileAsUploadedDb(fileUuid4);
    await setFileAsUploadedDb(fileUuid5);
    done();
  });

  it('/graphql:Q Get full data Component - OK check UpdatedAt for component and modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    expect(component.updatedAt).not.toBe(updatedAtCheckComponent);
    expect(component.componentModifications[0].updatedAt).not.toBe(updatedAtCheckModification);
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
            componentModificationFiles(args:{
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              ${downloadFileFields}
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

  // Testing component modification files list
  it('/graphql:Q componentModificationFilesList - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFilesList(args:{
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesList=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFilesList');
    done();
  });

  it('/graphql:Q componentModificationFilesList - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesList(args:{
              modificationUuid: "${parentModificationUuid}"
            }){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    expect(body.errors[0].path[0]).toBe('componentModificationFilesList');
    done();
  });

  it('/graphql:Q componentModificationFilesList - OK 5 files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesList(args:{
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesList=%o', body);
    const {
      data: { componentModificationFilesList },
    } = body;
    expect(componentModificationFilesList).toBeNonEmptyArray();
    expect(componentModificationFilesList[0].uuid).toBe(fileUuid1);
    expect(componentModificationFilesList[0].parentFileUuid).toBe(fileUuid1);
    expect(componentModificationFilesList[0].filename).toBe(filename1);
    expect(componentModificationFilesList[0].filesize).toBe(0);
    expect(componentModificationFilesList[1].uuid).toBe(fileUuid2);
    expect(componentModificationFilesList[1].parentFileUuid).toBe(fileUuid2);
    expect(componentModificationFilesList[1].filename).toBe(filename2);
    expect(componentModificationFilesList[2].uuid).toBe(fileUuid3);
    expect(componentModificationFilesList[2].parentFileUuid).toBe(fileUuid3);
    expect(componentModificationFilesList[2].filename).toBe(filename3);
    expect(componentModificationFilesList[3].uuid).toBe(fileUuid4);
    expect(componentModificationFilesList[3].parentFileUuid).toBe(fileUuid4);
    expect(componentModificationFilesList[3].filename).toBe(filename4);
    expect(componentModificationFilesList[4].uuid).toBe(fileUuid5);
    expect(componentModificationFilesList[4].parentFileUuid).toBe(fileUuid5);
    expect(componentModificationFilesList[4].filename).toBe(filename5);
    expect(componentModificationFilesList[4].filesize).toBe(0);
    done();
  });

  it('/graphql:Q componentModificationFilesList - OK 5 files (added hash and download url fields)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesList(args:{
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              ${showFileRelatedDataFields}
              hash
            sha256Hash
              downloadUrl
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesList=%o', body);
    const {
      data: { componentModificationFilesList },
    } = body;
    expect(componentModificationFilesList).toBeNonEmptyArray();
    expect(componentModificationFilesList[0].uuid).toBe(fileUuid1);
    expect(componentModificationFilesList[0].parentFileUuid).toBe(fileUuid1);
    expect(componentModificationFilesList[0].filename).toBe(filename1);
    expect(componentModificationFilesList[0].filesize).toBe(0);
    expect(componentModificationFilesList[0].hash).toBe("");
    expect(componentModificationFilesList[0].sha256Hash).toBe("");
    expect(componentModificationFilesList[0].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesList[1].uuid).toBe(fileUuid2);
    expect(componentModificationFilesList[1].parentFileUuid).toBe(fileUuid2);
    expect(componentModificationFilesList[1].filename).toBe(filename2);
    expect(componentModificationFilesList[1].hash).toBe("");
    expect(componentModificationFilesList[1].sha256Hash).toBe("");
    expect(componentModificationFilesList[1].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesList[2].uuid).toBe(fileUuid3);
    expect(componentModificationFilesList[2].parentFileUuid).toBe(fileUuid3);
    expect(componentModificationFilesList[2].filename).toBe(filename3);
    expect(componentModificationFilesList[2].hash).toBe("");
    expect(componentModificationFilesList[2].sha256Hash).toBe("");
    expect(componentModificationFilesList[2].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesList[3].uuid).toBe(fileUuid4);
    expect(componentModificationFilesList[3].parentFileUuid).toBe(fileUuid4);
    expect(componentModificationFilesList[3].filename).toBe(filename4);
    expect(componentModificationFilesList[3].hash).toBe("");
    expect(componentModificationFilesList[3].sha256Hash).toBe("");
    expect(componentModificationFilesList[3].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesList[4].uuid).toBe(fileUuid5);
    expect(componentModificationFilesList[4].parentFileUuid).toBe(fileUuid5);
    expect(componentModificationFilesList[4].filename).toBe(filename5);
    expect(componentModificationFilesList[4].filesize).toBe(0);
    expect(componentModificationFilesList[4].hash).toBe("");
    expect(componentModificationFilesList[4].sha256Hash).toBe("");
    expect(componentModificationFilesList[4].downloadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q componentModificationFilesList - OK limit offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesList(
              args:{modificationUuid: "${componentModificationUuidSecond}"}
              sort:{
                byField: "filename"
                asDesc: false
              }
              paginate: {
                currentPage: 2
                perPage: 2
              }
            ){
              ${showFileRelatedDataFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesList=%o', body);
    const {
      data: { componentModificationFilesList },
    } = body;
    expect(componentModificationFilesList).toBeNonEmptyArray();
    expect(componentModificationFilesList[0].uuid).toBe(fileUuid3);
    expect(componentModificationFilesList[0].filename).toBe(filename3);
    expect(componentModificationFilesList[1].uuid).toBe(fileUuid4);
    expect(componentModificationFilesList[1].filename).toBe(filename4);
    expect(componentModificationFilesList.length).toBe(2);
    done();
  });

  // Testing new revisions
  it('/graphql:M uploadModificationFiles - Ok new revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename0}"
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadModificationFiles },
    } = body;
    seconRevFileFileTestUuid = uploadModificationFiles[0].fileUuid;
    expect(uploadModificationFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[0].filename).toBe(filename0);
    seconRevFileFileTestUuid2 = uploadModificationFiles[1].fileUuid;
    expect(uploadModificationFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[1].filename).toBe(filename2);
    await setFlagHiddenAsOldRevDb(fileUuid1);
    await setFileAsUploadedDb(seconRevFileFileTestUuid);
    await setFlagHiddenAsOldRevDb(fileUuid2);
    await setFileAsUploadedDb(seconRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revision for new file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    // expect(showFileRevisions[0].uuid).toBe(fileUuid1);
    expect(showFileRevisions[0].filename).toBe(filename0);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  it('/graphql:M uploadModificationFiles - Ok new revision 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename2}"
            ]
            commitMsg: "Secon revision (rev.2)"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadModificationFiles },
    } = body;
    threeRevFileFileTestUuid2 = uploadModificationFiles[0].fileUuid;
    expect(uploadModificationFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[0].filename).toBe(filename2);
    expect(uploadModificationFiles[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${threeRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
            hash
            sha256Hash
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[0].hash).toBe("");
    expect(showFileRevisions[0].sha256Hash).toBe("");
    expect(showFileRevisions[0].downloadUrl).toBeNonEmptyString();
    expect(showFileRevisions[0].commitMsg).toBe("");
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(threeRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[2].hash).toBe("");
    expect(showFileRevisions[2].sha256Hash).toBe("");
    expect(showFileRevisions[2].downloadUrl).toBeNonEmptyString();
    expect(showFileRevisions[2].commitMsg).toBe("Secon revision (rev.2)");
    expect(showFileRevisions.length).toBe(3);
    await setFlagDeleteAsOldRevDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadModificationFiles - Ok new revision 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadModificationFiles },
    } = body;
    fourthRevFileFileTestUuid2 = uploadModificationFiles[0].fileUuid;
    expect(uploadModificationFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[0].filename).toBe(filename2);
    expect(uploadModificationFiles[0].uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(fourthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3/4 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fourthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for hidden file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for delete file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${threeRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for stranger file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fourthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:M uploadModificationFiles - BadRequest stranger component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadModificationFiles');
    done();
  });

  // Testing change active revision for file
  it('/graphql:M changeActiveFileRevision - Ok set revision 2 as active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeActiveFileRevision },
    } = body;
    expect(changeActiveFileRevision).toBe(true);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revisions for new active file revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest set remove revision as active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${threeRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest already active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest stranger Modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M uploadModificationFiles - Ok new revision 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadModificationFiles },
    } = body;
    fifthRevFileFileTestUuid2 = uploadModificationFiles[0].fileUuid;
    expect(uploadModificationFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[0].filename).toBe(filename2);
    expect(uploadModificationFiles[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 4/5 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fifthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[3].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(4);
    expect(showFileRevisions.length).toBe(4);
    done();
  });

  it('/graphql:M component - Ok check Counting hide', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filesCount
              paramsCount
              suppliersCount
              standardsCount
              modificationsCount
              componentModifications {
                filesCount
                files {
                  uuid
                }
                paramsCount
                modificationParams {
                  param {
                    paramId
                  }
                }
                filesetsCount
                filesetsForProgram {
                  filesCount
                }
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql component check count=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.filesCount).toBe(0);
    expect(component.paramsCount).toBe(0);
    expect(component.standardsCount).toBe(1);
    expect(component.suppliersCount).toBe(0);
    expect(component.modificationsCount).toBe(2);
    expect(component.componentModifications.length).toBe(2);
    expect(component.componentModifications[0].filesCount).toBe(0);
    expect(component.componentModifications[0].files.length).toBe(0);
    expect(component.componentModifications[0].filesetsCount).toBe(0);
    expect(component.componentModifications[0].filesetsForProgram).toBeEmptyArray();
    expect(component.componentModifications[0].paramsCount).toBe(0);
    expect(component.componentModifications[1].modificationParams.length).toBe(0);
    expect(component.componentModifications[1].filesCount).toBe(5);
    expect(component.componentModifications[1].files.length).toBe(5);
    expect(component.componentModifications[1].filesetsCount).toBe(0);
    expect(component.componentModifications[1].filesetsForProgram.length).toBe(0);
    expect(component.componentModifications[1].paramsCount).toBe(0);
    done();
  });

  it('/graphql:Q Get all files of Modification - OK check parent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentModificationFilesList(
            args:{modificationUuid: "${componentModificationUuidSecond}"}
            sort:{
              byField: "filename"
              asDesc: false
            }
          ){
            ${showFilesQuery}
          }
        }`,
      })
    .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesList=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModificationFilesList },
    } = body;
    expect(componentModificationFilesList[1].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(componentModificationFilesList[1].filename).toBe(filename2);
    expect(componentModificationFilesList[1].revision).toBe(4);
    // expect(componentModificationFilesList[1].parentFileUuid).toBe(seconRevFileFileTestUuid2);
    expect(componentModificationFilesList[1].parentFileUuid).toBe(fourthRevFileFileTestUuid2);
    expect(componentModificationFilesList.length).toBe(5);
    await setFlagHiddenAsOldRevDb(fileUuid2);
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFlagHiddenAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagHiddenAsOldRevDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadModificationFiles - Ok new revision 6 other versions are hidden', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadModificationFiles },
    } = body;
    sixthRevFileFileTestUuid2 = uploadModificationFiles[0].fileUuid;
    expect(uploadModificationFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[0].filename).toBe(filename2);
    expect(uploadModificationFiles[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 5/6 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${sixthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[3].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(4);
    expect(showFileRevisions[4].uuid).toBe(sixthRevFileFileTestUuid2);
    expect(showFileRevisions[4].revision).toBe(5);
    expect(showFileRevisions.length).toBe(5);
    await setFlagDeleteAsOldRevDb(fileUuid2);
    await setFlagDeleteAsOldRevDb(seconRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fifthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadModificationFiles - Ok new revision 7 other versions are deleted', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadModificationFiles(args: {
            modificationUuid: "${componentModificationUuidSecond}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadModificationFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadModificationFiles },
    } = body;
    seventhRevFileFileTestUuid2 = uploadModificationFiles[0].fileUuid;
    expect(uploadModificationFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadModificationFiles[0].filename).toBe(filename2);
    expect(uploadModificationFiles[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(seventhRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 1/7 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seventhRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(seventhRevFileFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  // contionue test for modification files
  it('/graphql:M deleteModificationFile - OK delete file 1', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteModificationFile(args: {
              fileUuid: "${seconRevFileFileTestUuid}"
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
            deleteModificationFile(args: {
              fileUuid: "${seventhRevFileFileTestUuid2}"
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
            deleteModificationFile(args: {
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
            deleteModificationFile(args: {
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
            deleteModificationFile(args: {
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

  it('/graphql:Q ModificationFiles - Ok not found files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFiles(args:{
              modificationUuid: "${componentModificationUuidSecond}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFiles=%o', body);
    const {
      data: { componentModificationFiles },
    } = body;
    expect(componentModificationFiles).toBeEmptyArray();
    done();
  });

  // Testing component modification fileset
  it('/graphql:Q componentModificationFilesets - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFilesets(args: {
              modificationUuid: "${componentModificationUuidSecond}"
            }){
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
      'BadRequest: Access denied'
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
            componentModificationFilesets(args: {
              modificationUuid: "${parentModificationUuid}"
            }){
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
            registerModificationFileset(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 7
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            registerModificationFileset(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 7
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    const {
      data: { registerModificationFileset },
    } = body;
    filesetForProgramUuid = registerModificationFileset;
    expect(registerModificationFileset).toBeNonEmptyString();
    done();
  });

  it('/graphql:M registerModificationFileset - Ok return found uuid of duplicate fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            registerModificationFileset(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 7
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    const {
      data: { registerModificationFileset },
    } = body;
    expect(registerModificationFileset).toBe(filesetForProgramUuid);
    done();
  });

  it('/graphql:Q Get full data Component - OK set UpdatedAt component and modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    updatedAtCheckComponent = component.updatedAt;
    updatedAtCheckModification = component.componentModifications.updatedAt;
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
            registerModificationFileset(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programId: 5
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql registerModificationFileset=%o', body);
    const {
      data: { registerModificationFileset },
    } = body;
    filesetForProgramUuid = registerModificationFileset;
    expect(registerModificationFileset).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q Get full data Component - OK check UpdatedAt after added fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    expect(component.updatedAt).not.toBe(updatedAtCheckComponent);
    expect(component.componentModifications[0].updatedAt).not.toBe(updatedAtCheckModification);
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
            componentModificationFilesets(args: {
              modificationUuid: "${componentModificationUuidSecond}"
            }){
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
    expect(componentModificationFilesets[0].program.id).toBe(5);
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
            componentModificationFilesets(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programIds: [5,7]
            }){
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
    expect(componentModificationFilesets[0].program.id).toBe(5);
    expect(componentModificationFilesets[0].program.name).toBeNonEmptyString();
    expect(componentModificationFilesets[1].uuid).toBeNonEmptyString();
    expect(componentModificationFilesets[1].program.id).toBe(7);
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
            componentModificationFilesets(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programIds: [1,2,5,8]
            }){
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
            componentModificationFilesets(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              programIds: [1,2,3]
            }){
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
  it('/graphql:Q componentModificationFilesOfFileset - BadRequest no token (Access denied)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: []
            }){
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
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
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${baseFilesetUuid}"
            }){
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
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${componentModificationFilesetsSecond}"
            }){
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

  it('/graphql:Q Get full data Component - OK set UpdatedAt component and modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    updatedAtCheckComponent = component.updatedAt;
    updatedAtCheckModification = component.componentModifications.updatedAt;
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
              args: {
                filesetUuid: "${filesetForProgramUuid}"
                filenames: [
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
    await setFileAsUploadedDb(fileUuid1);
    await setFileAsUploadedDb(fileUuid2);
    await setFileAsUploadedDb(fileUuid3);
    await setFileAsUploadedDb(fileUuid4);
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

  it('/graphql:Q Get full data Component - OK check UpdatedAt after upload files to fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              updatedAt
              componentModifications(filter: "${componentModificationUuidSecond}") {
                uuid
                updatedAt
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql set UpdatedAt date component=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.componentModifications[0].uuid).toBe(componentModificationUuidSecond);
    expect(component.updatedAt).not.toBe(updatedAtCheckComponent);
    expect(component.componentModifications[0].updatedAt).not.toBe(updatedAtCheckModification);
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
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${filesetForProgramUuid}"
            }){
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    fileOfFilesetUuid = componentModificationFilesOfFileset[0].uuid;
    expect(componentModificationFilesOfFileset[0].uuid).toBeNonEmptyString();
    expect(componentModificationFilesOfFileset[0].filename).toBeNonEmptyString();
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
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: ["${fileOfFilesetUuid}"]
            }){
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    fileOfFilesetUuid = componentModificationFilesOfFileset[0].uuid;
    expect(componentModificationFilesOfFileset[0].uuid).toBeNonEmptyString();
    expect(componentModificationFilesOfFileset[0].filename).toBeNonEmptyString();
    done();
  });

  // Testing component modification fileset files
  it('/graphql:Q componentModificationFilesetFiles - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${filesetForProgramUuid}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesetFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('componentModificationFilesetFiles');
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${parentModificationUuid}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Not found fileset data");
    expect(body.errors[0].path[0]).toBe('componentModificationFilesetFiles');
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - OK all files of fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${filesetForProgramUuid}"
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesetFiles=%o', body);
    const {
      data: { componentModificationFilesetFiles },
    } = body;
    expect(componentModificationFilesetFiles[0].uuid).toBe(fileUuid1);
    expect(componentModificationFilesetFiles[0].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles[1].uuid).toBe(fileUuid2);
    expect(componentModificationFilesetFiles[1].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles[2].uuid).toBe(fileUuid3);
    expect(componentModificationFilesetFiles[2].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles[3].uuid).toBe(fileUuid4);
    expect(componentModificationFilesetFiles[3].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles.length).toBe(4);
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - OK 2 files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: [
                "${fileUuid2}",
                "${fileUuid3}",
              ]
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesetFiles=%o', body);
    const {
      data: { componentModificationFilesetFiles },
    } = body;
    expect(componentModificationFilesetFiles[0].uuid).toBe(fileUuid2);
    expect(componentModificationFilesetFiles[0].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles[1].uuid).toBe(fileUuid3);
    expect(componentModificationFilesetFiles[1].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles.length).toBe(2);
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - Ok select 2 filesets', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: [
                "${fileUuid2}",
                "${fileUuid3}",
              ]
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesetFiles=%o', body);
    const {
      data: { componentModificationFilesetFiles },
    } = body;
    expect(componentModificationFilesetFiles[0].uuid).toBe(fileUuid2);
    expect(componentModificationFilesetFiles[0].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles[1].uuid).toBe(fileUuid3);
    expect(componentModificationFilesetFiles[1].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles.length).toBe(2);
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - Ok found 1', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: [
                "${filesetForProgramUuid}",
                "${fileUuid4}",
              ]
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesetFiles=%o', body);
    const {
      data: { componentModificationFilesetFiles },
    } = body;
    expect(componentModificationFilesetFiles[0].uuid).toBe(fileUuid4);
    expect(componentModificationFilesetFiles[0].downloadUrl).toBeNonEmptyString();
    expect(componentModificationFilesetFiles.length).toBe(1);
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - Ok not found files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${filesetForProgramUuid}"
              fileUuids: [
                "${componentModificationFilesetsSecond}",
                "${filesetForProgramUuid}",
              ]
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesetFiles=%o', body);
    const {
      data: { componentModificationFilesetFiles },
    } = body;
    expect(componentModificationFilesetFiles).toBeEmptyArray();
    done();
  });

  it('/graphql:Q componentModificationFilesetFiles - BadRequest not found fileset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            componentModificationFilesetFiles(args: {
              filesetUuid: "${fileUuid1}"
              fileUuids: [
                "${fileUuid2}",
                "${fileUuid3}",
              ]
            }){
              ${downloadFileFields}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql  body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Not found fileset data");
    expect(body.errors[0].path[0]).toBe('componentModificationFilesetFiles');
    done();
  });

  // Testing new revisions
  it('/graphql:M uploadFilesToFileset - Ok new revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename0}"
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadFilesToFileset },
    } = body;
    seconRevFileFileTestUuid = uploadFilesToFileset[0].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename0);
    seconRevFileFileTestUuid2 = uploadFilesToFileset[1].fileUuid;
    expect(uploadFilesToFileset[1].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[1].filename).toBe(filename2);
    await setFlagHiddenAsOldRevDb(fileUuid1);
    await setFileAsUploadedDb(seconRevFileFileTestUuid);
    await setFlagHiddenAsOldRevDb(fileUuid2);
    await setFileAsUploadedDb(seconRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revision for new file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    // expect(showFileRevisions[0].uuid).toBe(fileUuid1);
    expect(showFileRevisions[0].filename).toBe(filename0);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  it('/graphql:M uploadFilesToFileset - Ok new revision 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadFilesToFileset },
    } = body;
    threeRevFileFileTestUuid2 = uploadFilesToFileset[0].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename2);
    expect(uploadFilesToFileset[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${threeRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(threeRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    await setFlagDeleteAsOldRevDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadFilesToFileset - Ok new revision 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadFilesToFileset },
    } = body;
    fourthRevFileFileTestUuid2 = uploadFilesToFileset[0].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename2);
    expect(uploadFilesToFileset[0].uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(fourthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3/4 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fourthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for hidden file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for delete file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${threeRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:Q showFileRevisions - BadRequest revisions for stranger file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fourthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:M uploadFilesToFileset - BadRequest stranger component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadFilesToFileset');
    done();
  });

  // Testing change active revision for file
  it('/graphql:M changeActiveFileRevision - Ok set revision 2 as active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeActiveFileRevision },
    } = body;
    expect(changeActiveFileRevision).toBe(true);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revisions for new active file revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seconRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest set remove revision as active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${threeRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest already active', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest stranger Modification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          changeActiveFileRevision(fileUuid: "${seconRevFileFileTestUuid2}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeActiveFileRevision=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M uploadFilesToFileset - Ok new revision 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadFilesToFileset },
    } = body;
    fifthRevFileFileTestUuid2 = uploadFilesToFileset[0].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename2);
    expect(uploadFilesToFileset[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 4/5 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${fifthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[3].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(4);
    expect(showFileRevisions.length).toBe(4);
    await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q Get all files of Fileset - OK check parent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          componentModificationFilesOfFileset(
            args:{filesetUuid: "${filesetForProgramUuid}"}
            sort:{
              byField: "filename"
              asDesc: false
            }
          ){
            ${showFilesQuery}
          }
        }`,
      })
    .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    expect(componentModificationFilesOfFileset[1].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(componentModificationFilesOfFileset[1].filename).toBe(filename2);
    expect(componentModificationFilesOfFileset[1].revision).toBe(4);
    // expect(componentModificationFilesOfFileset[1].parentFileUuid).toBe(seconRevFileFileTestUuid2);
    expect(componentModificationFilesOfFileset[1].parentFileUuid).toBe(fourthRevFileFileTestUuid2);
    expect(componentModificationFilesOfFileset.length).toBe(4);
    await setFlagHiddenAsOldRevDb(fileUuid2);
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    // await setFlagHiddenAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagHiddenAsOldRevDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadFilesToFileset - Ok new revision 6 other versions are hidden', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadFilesToFileset },
    } = body;
    sixthRevFileFileTestUuid2 = uploadFilesToFileset[0].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename2);
    expect(uploadFilesToFileset[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 5/6 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${sixthRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(4);
    expect(showFileRevisions[3].uuid).toBe(sixthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(5);
    expect(showFileRevisions.length).toBe(4);
    await setFlagDeleteAsOldRevDb(fileUuid2);
    await setFlagDeleteAsOldRevDb(seconRevFileFileTestUuid2);
    // await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fifthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadFilesToFileset - Ok new revision 7 other versions are deleted', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadFilesToFileset(args: {
            filesetUuid: "${filesetForProgramUuid}"
            filenames: [
              "${filename2}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFilesToFileset=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadFilesToFileset },
    } = body;
    seventhRevFileFileTestUuid2 = uploadFilesToFileset[0].fileUuid;
    expect(uploadFilesToFileset[0].fileUuid).toBeNonEmptyString();
    expect(uploadFilesToFileset[0].filename).toBe(filename2);
    expect(uploadFilesToFileset[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(seventhRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 1/7 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          showFileRevisions(fileUuid: "${seventhRevFileFileTestUuid2}") {
            ${showFileRevisionsQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showFileRevisions=%o', body);
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(seventhRevFileFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  // Testing component modification file of fileset for delete
  it('/graphql:M deleteFilesFromFileset - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `mutation {
            deleteFilesFromFileset(
              args: {
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
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteFilesFromFileset');
    done();
  });

  it('/graphql:M component - Ok check Counting hide and delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `query {
            component(componentUuid: "${componentUuidNoStandard}") {
              uuid
              filesCount
              paramsCount
              suppliersCount
              standardsCount
              modificationsCount
              componentModifications {
                filesCount
                paramsCount
                filesetsCount
                filesetsForProgram {
                  filesCount
                }
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql component check count=%o', body);
    // expect(body).toBe(0);
    const {
      data: { component },
    } = body;
    expect(component.uuid).toBe(componentUuidNoStandard);
    expect(component.filesCount).toBe(0);
    expect(component.modificationsCount).toBe(2);
    expect(component.paramsCount).toBe(0);
    expect(component.standardsCount).toBe(1);
    expect(component.suppliersCount).toBe(0);
    expect(component.componentModifications[0].filesCount).toBe(0);
    expect(component.componentModifications[0].filesetsCount).toBe(0);
    expect(component.componentModifications[0].filesetsForProgram).toBeEmptyArray();
    expect(component.componentModifications[0].paramsCount).toBe(0);
    expect(component.componentModifications[1].filesCount).toBe(0);
    expect(component.componentModifications[1].filesetsCount).toBe(2);
    expect(component.componentModifications[1].filesetsForProgram[0].filesCount).toBe(4);
    expect(component.componentModifications[1].filesetsForProgram[1].filesCount).toBe(0);
    expect(component.componentModifications[1].paramsCount).toBe(0);
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
              args: {
                filesetUuid: "${filesetForProgramUuid}"
                fileUuids: [
                  "${seconRevFileFileTestUuid}",
                  "${fileUuid1}"
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

  it('/graphql:M deleteFilesFromFileset - Ok delete non-existent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
          query: `mutation {
            deleteFilesFromFileset(
              args: {
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
    expect(deleteFilesFromFileset).toBe(false);
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
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${filesetForProgramUuid}"
            }){
              ${fileDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql componentModificationFilesOfFileset=%o', body);
    const {
      data: { componentModificationFilesOfFileset },
    } = body;
    expect(componentModificationFilesOfFileset.length).toBe(2);
    done();
  });

  // Testing component modification fileset for delete
  it('/graphql:M deleteModificationFileset - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
          query: `mutation {
            deleteModificationFileset(args: {
              modificationUuid: "${componentModificationUuidSecond}"
              filesetUuid: "${filesetForProgramUuid}"
            })
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql deleteModificationFileset=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteModificationFileset(args: {
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
            deleteModificationFileset(args: {
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
            componentModificationFilesOfFileset(args: {
              filesetUuid: "${filesetForProgramUuid}"
            }){
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
              args: {
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
      'BadRequest: Token not found'
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
    expect(putComponentUpdate).toBe(3);
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
              args: {
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
            registerCompanyRole(args: {
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
            addAccessRole(args: {
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
            addCompanyMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addCompanyMember },
    } = body;
    expect(addCompanyMember).toBe(true);
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
            addAccessRole(args: {
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
              args: {
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
    expect(putComponentUpdate).toBe(5);
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
              args: {
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
              args: {
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
              args: {
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
            deleteComponentModification(args: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteComponentModification(args: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            })
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
            deleteComponentModification(args: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentModification=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteComponentModification },
    } = body;
    expect(deleteComponentModification).toBeNonEmptyString();
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
            deleteComponentModification(args: {
              componentUuid: "${componentUuidStandard}"
              modificationUuid: "${componentModificationUuidFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('Internal Server Error');
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
            changeComponentAccess(args: {
              componentUuid: "${componentUuidNoStandard}"
              newTypeAccessId: ${typeAccessId2}
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
            changeComponentAccess(args: {
              componentUuid: "${componentUuidNoStandard}"
              newTypeAccessId: ${typeAccessId2}
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
              typeAccess {
                typeAccessId
              }
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
    expect(component.typeAccess.typeAccessId).toBe(typeAccessId2);
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
            transferComponentOwnership(args: {
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
            transferComponentOwnership(args: {
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
              typeAccess {
                typeAccessId
              }
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
    expect(component.typeAccess.typeAccessId).toBe(typeAccessId2);
    done();
  });

  // Testing delete component
  it('/graphql:M deleteComponent - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteComponent(componentUuid: "${componentUuidStandard}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
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
            deleteComponent(componentUuid: "${componentUuidStandard}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
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
            deleteComponent(componentUuid: "${componentUuidStandard}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponent=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteComponent },
    } = body;
    expect(deleteComponent).toBeNonEmptyString();
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
            deleteComponent(componentUuid: "${componentUuidStandard}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('deleteComponent');
    done();
  });
});
