const debug = require('debug')('cdbs-back:service.test.js');
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
var authorizationUserFirst = "";
var authorizationUserSecond = "";
var authorizationTokenFirst = "";
var authorizationTokenSecond = "";

// data for service
const nameService = "GOST 2012 Test service";
const descriptionService = "Test GOST service";
const serviceStatusId = 1;
const regionId = 5;
const nameService2 = "GOST 2012 Test service 2222";
const descriptionService2 = "Test GOST service 2222";
const serviceStatusId2 =  3;
const regionId2 = 5;
const tooLongKeyword = 'я'.repeat(101);
var serviceUuidFirst = "";
var serviceUuidSecond = "";
var serviceUuidSupplier = "";

const usersListAccessServiceQuery = ` \
serviceUuid \
user { \
  uuid \
  username \
} \
permission { \
  typeAccessId \
  langId \
  name \
} \
isEnabled \
createdAt \
updatedAt \
`;

const companiesListAccessServiceQuery = ` \
serviceUuid \
company { \
  uuid \
  shortname \
} \
permission { \
  typeAccessId \
  langId \
  name \
} \
isEnabled \
createdAt \
updatedAt \
`;

const serviceFullDataQuery = ` \
uuid \
name \
description \
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
serviceStatus { \
  serviceStatusId \
  langId \
  name \
} \
region { \
  regionId \
  langId \
  region \
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
serviceSpecs { \
  specId \
  langId \
  spec \
} \
serviceKeywords { \
  id \
	keyword \
} \
createdAt \
updatedAt \
`;

const servicesListQuery = ` \
uuid \
name \
description \
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
serviceStatus { \
  serviceStatusId \
  langId \
  name \
} \
files { \
  uuid \
  filename \
  filesize \
  downloadUrl \
} \
updatedAt \
`;

const showFileRevisionsQuery = ` \
uuid \
filename \
revision \
ownerUser { \
  username \
} \
`;

const showServiceFilesQuery = ` \
files { \
  uuid \
  filename \
  revision \
  parentFileUuid \
  createdAt \
} \
`;

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

// data for represent
const regionIdRepresentation = 15;
const representationTypeId = 1;
const nameRepresentationFirst = "test first additional office";
const nameRepresentationSecond = "test second additional office";
const addressRepresentation = "Fake str, Fantom";
const phoneRepresentation = "+743874487556";
const uuidFake = "2cd385e1-8f7e-4908-8235-dfe42938b888";
const uuidRepresentArray = [];
var companyUuidFirst = "";
var uuidRepresentFirst = "";
var uuidRepresentDelete = "";

// role company member
var langId = 1;
var nameRole = "test role";
var newRoleId = 0;

// service type access
const typeAccessId3 = 3;
const typeAccessId2 = 2;
const typeAccessId1 = 1;
const typesAccessIds23 = [2,3];

// service keywords
const keywordIdsOk = [1,3,5];
const keywordIdsDup = [1,2,3,4,5];
const keywordNames = ["asd2","asd3","asd4"];
const keywordNamesDup = ["asd2","asd3","asd4","asd5","asd6"];
const keywordNamesBad = ["asd11","asd12345678","asd12"];

// service specs
const specIdsOk = [10,30,55];
const specIdsDup = [10,22,30,44,55];
const specIdsDel = [10,55];
const idErr = 0;

// service files
const filename1 = "file-test-name 1.pdf";
const filename2 = "file-test-name 2.pdf";
const filename3 = "file-test-name 3.pdf";
const filename4 = "file-test-name 4.pdf";
const filename5 = "file-test-name 5.pdf";

const descriptionServiceFileTest = "test desctiption for service";
const filenameServiceFileTest = "second name file for service.pdf";
const badFilenameServiceFileTest = "name* file/ service.pdf";
const goodFilenameServiceFileTest = "name file service.pdf";
const badFilenameServiceFaviconTest = "no image file.pdf";
const goodFilenameServiceFaviconTest = "image file.png";

var fileServiceFileTestUuid = "";
var fileServiceFileTestUuid2 = "";
var seconRevFileFileTestUuid = "";
var seconRevFileFileTestUuid2 = "";
var threeRevFileFileTestUuid2 = "";
var fourthRevFileFileTestUuid2 = "";
var fifthRevFileFileTestUuid2 = "";
var sixthRevFileFileTestUuid2 = "";
var seventhRevFileFileTestUuid2 = "";

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname in (?,?);', [
    orgname,
    orgname2,
  ]);
}

async function cleanupServiceDb() {
  return global.knex.raw('DELETE FROM service_ref WHERE name in (?)', [
    nameService,
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
  return global.knex.raw('DELETE FROM user_ref WHERE username in (?,?)', [
    username,
    username2,
  ]);
}

async function cleanupKeywordsDb() {
  return global.knex.raw('DELETE FROM keyword_ref WHERE keyword in (?,?,?,?,?)', [
    "asd2","asd3","asd4","asd5","asd6"
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

describe('service', () => {
  beforeAll(() => {
    cleanupCompanyRepresentDb();
    // cleanupServiceDb();
    cleanupCompanyDb();
    // cleanupTokenDb();
    cleanupUserDb();
    cleanupKeywordsDb();
    return;
  });
  afterAll(() => {
    cleanupCompanyRepresentDb();
    // cleanupServiceDb();
    cleanupCompanyDb();
    // cleanupTokenDb();
    cleanupUserDb();
    cleanupKeywordsDb();
    return;
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
        debug('/login headers=%o', headers);
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
        debug('/login headers=%o', headers);
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
    // change supplier status on 1
    await global.knex.raw('UPDATE company_ref SET is_supplier=? WHERE uuid=?', [
      't',
      companyUuidSupplier,
    ]);
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

  it('/graphql:M serviceRequest - BadRequest without token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found");
    done();
  });

  it('/graphql:M serviceRequest - Ok with token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceRequest=%o', body);
    // expect(body).toBe(0);
    const {
      data: { serviceRequest },
    } = body;
    expect(serviceRequest).toBeNonEmptyString();
    serviceUuidSupplier = serviceRequest;
    done();
  });

  it('/graphql:M serviceRequest - OK user is owner company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceRequest=%o', body);
    // expect(body).toBe(0);
    const {
      data: { serviceRequest },
    } = body;
    expect(serviceRequest).toBeNonEmptyString();
    serviceUuidFirst = serviceRequest;
    done();
  });

  it('/graphql:M serviceRequest - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceRequest=%o', body);
    const {
      data: { serviceRequest },
    } = body;
    expect(serviceRequest).toBeNonEmptyString();
    serviceUuidSecond = serviceRequest;
    done();
  });

  it('/graphql:M serviceRequest - Ok without access to company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceRequest=%o', body);
    // expect(body).toBe(0);
    const {
      data: { serviceRequest },
    } = body;
    expect(serviceRequest).toBeNonEmptyString();
    done();
  });

  it('/graphql:M serviceRequest - BadRequest not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidNoSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: The company is not supplier");
    done();
  });

  // Testing update service data
  it('/graphql:M putServiceUpdate - OK rand data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putServiceUpdate(
            serviceUuid: "${serviceUuidSecond}"
            args: {
              name: "name test for upda",
              description: "description test for upda",
              regionId: 3
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putServiceUpdate },
    } = body;
    expect(putServiceUpdate).toBe(3);
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          putServiceUpdate(
            serviceUuid: "${serviceUuidSecond}"
            args: {
              name: "${nameService}",
              description: "${descriptionService}",
              regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M putServiceUpdate - OK return old data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putServiceUpdate(
            serviceUuid: "${serviceUuidSecond}"
            args: {
              name: "${nameService}",
              description: "${descriptionService}",
              regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    const {
      data: { putServiceUpdate },
    } = body;
    expect(putServiceUpdate).toBe(3);
    done();
  });

  it('/graphql:M putServiceUpdate - OK data already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putServiceUpdate(
            serviceUuid: "${serviceUuidSecond}"
            args: {
              name: "${nameService}",
              description: "${descriptionService}",
              regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      "BadRequest: The data has already"
    );
    done();
  });

  // Testing add files for service
  it('/graphql:M uploadServiceFiles - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: "${badFilenameServiceFileTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('uploadServiceFiles');
    done();
  });

  it('/graphql:M uploadServiceFiles - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${badFilenameServiceFileTest}"
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    fileServiceFileTestUuid = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(goodFilenameServiceFileTest);
    expect(uploadServiceFiles[0].uploadUrl).toBeNonEmptyString();
    fileServiceFileTestUuid2 = uploadServiceFiles[1].fileUuid;
    expect(uploadServiceFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[1].filename).toBe(filenameServiceFileTest);
    expect(uploadServiceFiles[1].uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(fileServiceFileTestUuid);
    await setFileAsUploadedDb(fileServiceFileTestUuid2);
    done();
  });

  it('/graphql:M uploadServiceFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: "${badFilenameServiceFileTest}"
          }) {
            fileUuid
            filename
            uploadUrl
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

  // Testing get service files
  it('/graphql:Q serviceFiles - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          serviceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('serviceFiles');
    done();
  });

  it('/graphql:Q serviceFiles - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          serviceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { serviceFiles },
    } = body;
    expect(serviceFiles[0].uuid).toBe(fileServiceFileTestUuid);
    expect(serviceFiles[1].uuid).toBe(fileServiceFileTestUuid2);
    expect(serviceFiles.length).toBe(2);
    done();
  });

  it('/graphql:Q serviceFiles - Ok filter by uuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          serviceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filesUuids: "${fileServiceFileTestUuid2}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { serviceFiles },
    } = body;
    expect(serviceFiles[0].uuid).toBe(fileServiceFileTestUuid2);
    expect(serviceFiles.length).toBe(1);
    done();
  });

  it('/graphql:Q serviceFiles - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          serviceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('serviceFiles');
    done();
  });

  // Testing delete files of service
  it('/graphql:M deleteServiceFile - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          deleteServiceFile(args: {
            serviceUuid: "${serviceUuidSecond}"
            fileUuid: "${fileServiceFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceFile=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceFile');
    done();
  });

  it('/graphql:M deleteServiceFile - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteServiceFile(args: {
            serviceUuid: "${serviceUuidSecond}"
            fileUuid: "${fileServiceFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceFile=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteServiceFile },
    } = body;
    expect(deleteServiceFile).toBe(true);
    done();
  });

  it('/graphql:M deleteServiceFile - Ok not found file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteServiceFile(args: {
            serviceUuid: "${serviceUuidSecond}"
            fileUuid: "${fileServiceFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceFile=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteServiceFile },
    } = body;
    expect(deleteServiceFile).toBe(false);
    done();
  });

  it('/graphql:M deleteServiceFile - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteServiceFile(args: {
            serviceUuid: "${serviceUuidSecond}"
            fileUuid: "${fileServiceFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  // check add and delete files
  it('/graphql:Q Get full data Service - OK check add/del files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query service {
          service (serviceUuid: "${serviceUuidSecond}"){
            ${serviceFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql service=%o', body);
    // expect(body).toBe(0);
    const {
      data: { service },
    } = body;
    expect(service.uuid).toBe(serviceUuidSecond);
    expect(service.files[0].uuid).toBe(fileServiceFileTestUuid2);
    expect(service.files[0].filename).toBe(filenameServiceFileTest);
    expect(service.files[0].parentFileUuid).toBeNonEmptyString();
    expect(service.files[0].ownerUser.uuid).toBe(authorizationUserFirst);
    expect(service.files[0].contentType).toBeNonEmptyString();
    done();
  });

  // Testing new revisions
  it('/graphql:M uploadServiceFiles - Ok new revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${goodFilenameServiceFileTest}"
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    seconRevFileFileTestUuid = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(goodFilenameServiceFileTest);
    seconRevFileFileTestUuid2 = uploadServiceFiles[1].fileUuid;
    expect(uploadServiceFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[1].filename).toBe(filenameServiceFileTest);
    await setFlagHiddenAsOldRevDb(fileServiceFileTestUuid);
    await setFileAsUploadedDb(seconRevFileFileTestUuid);
    await setFlagHiddenAsOldRevDb(fileServiceFileTestUuid2);
    await setFileAsUploadedDb(seconRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show revision for new file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    // expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid);
    expect(showFileRevisions[0].filename).toBe(goodFilenameServiceFileTest);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  it('/graphql:M uploadServiceFiles - Ok new revision 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    threeRevFileFileTestUuid2 = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(filenameServiceFileTest);
    expect(uploadServiceFiles[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(threeRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    await setFlagDeleteAsOldRevDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadServiceFiles - Ok new revision 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    fourthRevFileFileTestUuid2 = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(filenameServiceFileTest);
    expect(uploadServiceFiles[0].uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(fourthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 3/4 revisions for second file', async (done) => {
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
    // expect(body).toBe(0);
    const {
      data: { showFileRevisions },
    } = body;
    expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid2);
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('showFileRevisions');
    done();
  });

  it('/graphql:M uploadServiceFiles - BadRequest stranger service', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadServiceFiles');
    done();
  });

  // Testing change active revision for file
  it('/graphql:M changeActiveFileRevision - Ok set revision 2 as active', async (done) => {
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
        `Bearer ${authorizationTokenFirst}`
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
    expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid2);
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
        `Bearer ${authorizationTokenFirst}`
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
      'BadRequest: Revision already active or deleted'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M changeActiveFileRevision - BadRequest stranger service', async (done) => {
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
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeActiveFileRevision');
    done();
  });

  it('/graphql:M uploadServiceFiles - Ok new revision 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    fifthRevFileFileTestUuid2 = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(filenameServiceFileTest);
    expect(uploadServiceFiles[0].uploadUrl).toBeNonEmptyString();
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 4/5 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid2);
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

  it('/graphql:Q Get all files of Service - OK check parent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query service {
          service (serviceUuid: "${serviceUuidSecond}"){
            ${showServiceFilesQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showServiceFilesQuery=%o', body);
    // expect(body).toBe(0);
    const {
      data: { service },
    } = body;
    expect(service.files[1].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(service.files[1].filename).toBe(filenameServiceFileTest);
    expect(service.files[1].revision).toBe(4);
    // expect(service.files[1].parentFileUuid).toBe(seconRevFileFileTestUuid2);
    expect(service.files[1].parentFileUuid).toBe(fourthRevFileFileTestUuid2);
    expect(service.files.length).toBe(2);
    done();
  });

  it('/graphql:M changeActiveFileRevision - Ok set revision 2 as active', async (done) => {
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
    // expect(body).toBe(0);
    const {
      data: { changeActiveFileRevision },
    } = body;
    expect(changeActiveFileRevision).toBe(true);
    await setFlagHiddenAsOldRevDb(fileServiceFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagHiddenAsOldRevDb(fifthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok checking the grandchild after delete child', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    // expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    // expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[2].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(4);
    expect(showFileRevisions.length).toBe(3);
    await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadServiceFiles - Ok new revision 6 other versions are hidden', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    sixthRevFileFileTestUuid2 = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(filenameServiceFileTest);
    expect(uploadServiceFiles[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 5/6 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(showFileRevisions[0].uuid).toBe(fileServiceFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    // expect(showFileRevisions[2].uuid).toBe(fourthRevFileFileTestUuid2);
    // expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions[2].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(4);
    expect(showFileRevisions[3].uuid).toBe(sixthRevFileFileTestUuid2);
    expect(showFileRevisions[3].revision).toBe(5);
    expect(showFileRevisions.length).toBe(4);
    await setFlagDeleteAsOldRevDb(fileServiceFileTestUuid2);
    await setFlagDeleteAsOldRevDb(seconRevFileFileTestUuid2);
    // await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fifthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadServiceFiles - Ok new revision 7 other versions are deleted', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadServiceFiles(args: {
            serviceUuid: "${serviceUuidSecond}"
            filenames: [
              "${filenameServiceFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadServiceFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadServiceFiles },
    } = body;
    seventhRevFileFileTestUuid2 = uploadServiceFiles[0].fileUuid;
    expect(uploadServiceFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadServiceFiles[0].filename).toBe(filenameServiceFileTest);
    expect(uploadServiceFiles[0].uploadUrl).toBeNonEmptyString();
    // await setFlagHiddenAsOldRevDb(seconRevFileFileTestUuid2);
    await setFileAsUploadedDb(seventhRevFileFileTestUuid2);
    done();
  });

  it('/graphql:Q showFileRevisions - Ok show 1/7 revisions for second file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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

  // Testing adding service specs
  it('/graphql:M addServiceSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('addServiceSpecs');
    done();
  });

  it('/graphql:M addServiceSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { addServiceSpecs },
    } = body;
    expect(addServiceSpecs).toBe(3);
    done();
  });

  it('/graphql:M addServiceSpecs - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { addServiceSpecs },
    } = body;
    expect(addServiceSpecs).toBe(2);
    done();
  });

  it('/graphql:M addServiceSpecs - BadRequest all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: This ids [10, 30, 55] already has"
    );
    expect(body.errors[0].path[0]).toBe('addServiceSpecs');
    done();
  });

  it('/graphql:M addServiceSpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('addServiceSpecs');
    done();
  });

  it('/graphql:M addServiceSpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addServiceSpecs');
    done();
  });

  it('/graphql:Q Get full data Service - OK check add specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query service {
          service (serviceUuid: "${serviceUuidFirst}"){
            ${serviceFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql service=%o', body);
    // expect(body).toBe(0);
    const {
      data: { service },
    } = body;
    expect(service.serviceSpecs[0].specId).toBe(10);
    expect(service.serviceSpecs[0].spec).toBeNonEmptyString();
    expect(service.serviceSpecs[1].specId).toBe(22);
    expect(service.serviceSpecs[1].spec).toBeNonEmptyString();
    expect(service.serviceSpecs[2].specId).toBe(30);
    expect(service.serviceSpecs[2].spec).toBeNonEmptyString();
    expect(service.serviceSpecs[3].specId).toBe(44);
    expect(service.serviceSpecs[3].spec).toBeNonEmptyString();
    expect(service.serviceSpecs[4].specId).toBe(55);
    expect(service.serviceSpecs[4].spec).toBeNonEmptyString();
    done();
  });

  // Testing get specs for service
  it('/graphql:Q serviceSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          serviceSpecs(
            serviceUuid: "${serviceUuidSecond}"
          ){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('serviceSpecs');
    done();
  });

  it('/graphql:Q serviceSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          serviceSpecs(
            serviceUuid: "${serviceUuidFirst}"
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
      data: { serviceSpecs },
    } = body;
    expect(serviceSpecs.length).toBe(5);
    expect(serviceSpecs[0].specId).toBe(10);
    expect(serviceSpecs[0].spec).toBeNonEmptyString();
    expect(serviceSpecs[1].specId).toBe(22);
    expect(serviceSpecs[1].spec).toBeNonEmptyString();
    expect(serviceSpecs[2].specId).toBe(30);
    expect(serviceSpecs[2].spec).toBeNonEmptyString();
    expect(serviceSpecs[3].specId).toBe(44);
    expect(serviceSpecs[3].spec).toBeNonEmptyString();
    expect(serviceSpecs[4].specId).toBe(55);
    expect(serviceSpecs[4].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q serviceSpecs - OK with limit and offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          serviceSpecs(
            serviceUuid: "${serviceUuidFirst}"
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
      data: { serviceSpecs },
    } = body;
    expect(serviceSpecs.length).toBe(1);
    expect(serviceSpecs[0].specId).toBe(44);
    expect(serviceSpecs[0].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q serviceSpecs - OK not found specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          serviceSpecs(
            serviceUuid: "${serviceUuidFirst}"
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
      data: { serviceSpecs },
    } = body;
    expect(serviceSpecs).toBeEmptyArray();
    done();
  });

  // Testing delete service specs
  it('/graphql:M deleteServiceSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceSpecs');
    done();
  });

  it('/graphql:M deleteServiceSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { deleteServiceSpecs },
    } = body;
    expect(deleteServiceSpecs).toBe(2);
    done();
  });

  it('/graphql:M deleteServiceSpecs - OK data already delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { deleteServiceSpecs },
    } = body;
    expect(deleteServiceSpecs).toBe(0);
    done();
  });

  it('/graphql:M deleteServiceSpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceSpecs');
    done();
  });

  it('/graphql:M deleteServiceSpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteServiceSpecs(args: {
            serviceUuid: "${serviceUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceSpecs');
    done();
  });

  it('/graphql:Q Get full data Service - OK check delete specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query service {
          service (serviceUuid: "${serviceUuidFirst}"){
            ${serviceFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql service=%o', body);
    // expect(body).toBe(0);
    const {
      data: { service },
    } = body;
    expect(service.serviceSpecs.length).toBe(3);
    expect(service.serviceSpecs[0].specId).toBe(22);
    expect(service.serviceSpecs[0].spec).toBeNonEmptyString();
    expect(service.serviceSpecs[1].specId).toBe(30);
    expect(service.serviceSpecs[1].spec).toBeNonEmptyString();
    expect(service.serviceSpecs[2].specId).toBe(44);
    expect(service.serviceSpecs[2].spec).toBeNonEmptyString();
    done();
  });

  // Testing adding service keywords
  it('/graphql:M addServiceKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addServiceKeywords(args: {
            serviceUuid: "${serviceUuidSecond}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('addServiceKeywords');
    done();
  });

  it('/graphql:M addServiceKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addServiceKeywords },
    } = body;
    expect(addServiceKeywords).toBe(3);
    done();
  });

  it('/graphql:M addServiceKeywords - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${keywordIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addServiceKeywords },
    } = body;
    expect(addServiceKeywords).toBe(2);
    done();
  });

  it('/graphql:M addServiceKeywords - OK all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywords=%o', body);
    const {
      data: { addServiceKeywords },
    } = body;
    expect(addServiceKeywords).toBe(0);
    done();
  });

  it('/graphql:M addServiceKeywords - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found keywords"
    );
    expect(body.errors[0].path[0]).toBe('addServiceKeywords');
    done();
  });

  it('/graphql:M addServiceKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addServiceKeywords');
    done();
  });

  // Testing adding service keywords by names
  it('/graphql:M addServiceKeywordsByNames - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addServiceKeywordsByNames(args: {
            serviceUuid: "${serviceUuidSecond}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywordsByNames=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('addServiceKeywordsByNames');
    done();
  });

  it('/graphql:M addServiceKeywordsByNames - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywordsByNames(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addServiceKeywordsByNames },
    } = body;
    expect(addServiceKeywordsByNames).toBe(3);
    done();
  });

  it('/graphql:M addServiceKeywordsByNames - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywordsByNames(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywords: ["asd2","asd3","asd4","asd5","asd6"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addServiceKeywordsByNames },
    } = body;
    expect(addServiceKeywordsByNames).toBe(2);
    done();
  });

  it('/graphql:M addServiceKeywordsByNames - OK all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywordsByNames(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywordsByNames=%o', body);
    const {
      data: { addServiceKeywordsByNames },
    } = body;
    expect(addServiceKeywordsByNames).toBe(0);
    done();
  });

  it('/graphql:M addServiceKeywordsByNames - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addServiceKeywordsByNames(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywords: ["asd11","${tooLongKeyword}","asd12345678","asd12"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywordsByNames=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Text must be less than 100 characters"
    );
    expect(body.errors[0].path[0]).toBe('addServiceKeywordsByNames');
    done();
  });

  it('/graphql:M addServiceKeywordsByNames - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addServiceKeywordsByNames(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addServiceKeywordsByNames=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addServiceKeywordsByNames');
    done();
  });

  it('/graphql:Q Get full data Service - OK check add keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query serviceQuery{
            service(serviceUuid: "${serviceUuidFirst}") {
              ${serviceFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter service=%o', body.data.service);
    // expect(body).toBe(0);
    expect(body.data.service.uuid).toBe(serviceUuidFirst);
    expect(body.data.service.serviceKeywords[0].id).toBe(1);
    expect(body.data.service.serviceKeywords[0].keyword).toBeNonEmptyString();
    expect(body.data.service.serviceKeywords[1].id).toBe(2);
    expect(body.data.service.serviceKeywords[1].keyword).toBeNonEmptyString();
    expect(body.data.service.serviceKeywords[2].id).toBe(3);
    expect(body.data.service.serviceKeywords[2].keyword).toBeNonEmptyString();
    expect(body.data.service.serviceKeywords[3].id).toBe(4);
    expect(body.data.service.serviceKeywords[3].keyword).toBeNonEmptyString();
    expect(body.data.service.serviceKeywords[4].id).toBe(5);
    expect(body.data.service.serviceKeywords[4].keyword).toBeNonEmptyString();
    done();
  });

  // Testing get keywords for service
  it('/graphql:Q serviceKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          serviceKeywords(
            serviceUuid: "${serviceUuidSecond}"
          ){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('serviceKeywords');
    done();
  });

  it('/graphql:Q serviceKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          serviceKeywords(
            serviceUuid: "${serviceUuidFirst}"
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
      data: { serviceKeywords },
    } = body;
    expect(serviceKeywords[0].id).toBe(1);
    expect(serviceKeywords[0].keyword).toBeNonEmptyString();
    expect(serviceKeywords[1].id).toBe(2);
    expect(serviceKeywords[1].keyword).toBeNonEmptyString();
    expect(serviceKeywords[2].id).toBe(3);
    expect(serviceKeywords[2].keyword).toBeNonEmptyString();
    expect(serviceKeywords[3].id).toBe(4);
    expect(serviceKeywords[3].keyword).toBeNonEmptyString();
    expect(serviceKeywords[4].id).toBe(5);
    expect(serviceKeywords[4].keyword).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q serviceKeywords - OK with limit and offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          serviceKeywords(
            serviceUuid: "${serviceUuidFirst}"
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
      data: { serviceKeywords },
    } = body;
    expect(serviceKeywords.length).toBe(2);
    expect(serviceKeywords[0].id).toBe(3);
    expect(serviceKeywords[0].keyword).toBeNonEmptyString();
    expect(serviceKeywords[1].id).toBe(4);
    expect(serviceKeywords[1].keyword).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q serviceKeywords - OK not found keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          serviceKeywords(
            serviceUuid: "${serviceUuidFirst}"
            paginate: {
              currentPage: 10
              perPage: 50
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
      data: { serviceKeywords },
    } = body;
    expect(serviceKeywords).toBeEmptyArray();
    done();
  });

  // Testing delete service keywords
  it('/graphql:M deleteServiceKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceKeywords');
    done();
  });

  it('/graphql:M deleteServiceKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceKeywords=%o', body);
    const {
      data: { deleteServiceKeywords },
    } = body;
    expect(deleteServiceKeywords).toBe(3);
    done();
  });

  it('/graphql:M deleteServiceKeywords - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found keywords"
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceKeywords');
    done();
  });

  it('/graphql:M deleteServiceKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteServiceKeywords(args: {
            serviceUuid: "${serviceUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteServiceKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('deleteServiceKeywords');
    done();
  });

  // Testing get services data
  it('/graphql:Q Get full data Service - BadRequest without token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query selectServiceQuery{
          services (args: {
            servicesUuids: "${serviceUuidSecond}"
          }) {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found");
    done();
  });

  it('/graphql:Q Get full data Service - OK ShowServiceShort', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectServiceQuery{
          service (serviceUuid: "${serviceUuidFirst}") {
            ${serviceFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { service },
    } = body;
    expect(service.uuid).toBe(serviceUuidFirst);
    done();
  });

  it('/graphql:Q Get full data Service - OK Select with uuid (no access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            servicesUuids: ["${serviceUuidFirst}"]
          }) {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { services },
    } = body;
    expect(services).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Service - OK Select with uuid (has access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            servicesUuids: ["${serviceUuidFirst}"]
          }) {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { services },
    } = body;
    expect(services[0].uuid).toBe(serviceUuidFirst);
    done();
  });

  it('/graphql:Q Get full data Service - OK no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            servicesUuids: "${serviceUuidSecond}"
          }) {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const {
      data: { services },
    } = body;
    expect(services).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Service - OK without arguments', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { services },
    } = body;
    expect(services).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Service - OK by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            companyUuid: "${companyUuidSupplier}"
          }){
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { services },
    } = body;
    expect(services.length).toBe(2);
    expect(services[0].uuid).toBe(serviceUuidSupplier);
    expect(services[0].name).toBe(nameService);
    // expect(services[1].uuid).toBe(serviceUuidSecond);
    done();
  });

  it('/graphql:Q Get full data Service - OK by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            companyUuid: "${companyUuidNoSupplier}"
          }){
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { services },
    } = body;
    expect(services).toBeEmptyArray();
    done();
  });

  // Testing add access for company
  it('/graphql:M setCompanyAccessService - OK add low access company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
                typeAccessId: ${typeAccessId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setCompanyAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyAccessService },
    } = body;
    expect(setCompanyAccessService).toBe(true);
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest need higher access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService}",
                description: "${descriptionService}",
                regionId: ${regionId}
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
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  it('/graphql:M setCompanyAccessService - OK add access company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
                typeAccessId: ${typeAccessId1}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setCompanyAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyAccessService },
    } = body;
    expect(setCompanyAccessService).toBe(true);
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
              typesAccessIds: [${typesAccessIds23}]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addAccessRole },
    } = body;
    expect(addAccessRole).toBe(2);
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

  it('/graphql:Q getCompaniesListAccessService - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            getCompaniesListAccessService(
              serviceUuid: "${serviceUuidFirst}"
            ) {
              ${companiesListAccessServiceQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getCompaniesListAccessService=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('getCompaniesListAccessService');
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
              typesAccessIds: ${typeAccessId1}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addAccessRole },
    } = body;
    expect(addAccessRole).toBe(1);
    done();
  });

  it('/graphql:Q getCompaniesListAccessService - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            getCompaniesListAccessService(
              serviceUuid: "${serviceUuidFirst}"
            ) {
              ${companiesListAccessServiceQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getCompaniesListAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { getCompaniesListAccessService },
    } = body;
    expect(getCompaniesListAccessService[0].serviceUuid).toBe(serviceUuidFirst);
    expect(getCompaniesListAccessService[0].company.uuid).toBe(companyUuidNoSupplier);
    expect(getCompaniesListAccessService[0].permission.typeAccessId).toBe(typeAccessId1);
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest not access for change company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService}",
                description: "${descriptionService}",
                regionId: ${regionId}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  it('/graphql:M putServiceUpdate - OK with access from company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService2}",
                description: "${descriptionService2}",
                regionId: ${regionId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  // disable access for authorizationTokenSecond
  it('/graphql:M deleteCompanyAccessService - OK delete access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyAccessService },
    } = body;
    expect(deleteCompanyAccessService).toBe(true);
    done();
  });

  it('/graphql:M deleteCompanyAccessService - BadRequest not found access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyAccessService=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access not found for company'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyAccessService');
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService}",
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
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest no access for company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService}",
                description: "${descriptionService}",
                regionId: ${regionId}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  // Testing add access for user
  // add access for authorizationTokenSecond
  it('/graphql:M setUserAccessService - OK add low access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                userUuid: "${authorizationUserSecond}"
                typeAccessId: ${typeAccessId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setUserAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setUserAccessService },
    } = body;
    expect(setUserAccessService).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Service - OK Select with fake uuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            servicesUuids: "${uuidFake}"
          }) {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { services },
    } = body;
    expect(services).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Service - OK Select with uuid (private access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectServiceQuery{
          services (args: {
            servicesUuids: "${serviceUuidFirst}"
          }) {
            ${servicesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { services },
    } = body;
    expect(services[0].uuid).toBe(serviceUuidFirst);
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest need higher access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService}",
                description: "${descriptionService}",
                regionId: ${regionId}
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
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  it('/graphql:M setUserAccessService - OK add access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                userUuid: "${authorizationUserSecond}"
                typeAccessId: ${typeAccessId1}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setUserAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setUserAccessService },
    } = body;
    expect(setUserAccessService).toBe(true);
    done();
  });

  it('/graphql:Q getUsersListAccessService - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            getUsersListAccessService(
              serviceUuid: "${serviceUuidFirst}"
            ) {
              ${usersListAccessServiceQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getUsersListAccessService=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('getUsersListAccessService');
    done();
  });

  it('/graphql:Q getUsersListAccessService - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            getUsersListAccessService(
              serviceUuid: "${serviceUuidFirst}"
            ) {
              ${usersListAccessServiceQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql getUsersListAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { getUsersListAccessService },
    } = body;
    expect(getUsersListAccessService[0].serviceUuid).toBe(serviceUuidFirst);
    expect(getUsersListAccessService[0].user.uuid).toBe(authorizationUserSecond);
    expect(getUsersListAccessService[0].permission.typeAccessId).toBe(typeAccessId1);
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService2}",
                description: "${descriptionService2}",
                regionId: ${regionId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest not owner service (with access user)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService2}",
                description: "${descriptionService}",
                regionId: ${regionId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putServiceUpdate=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  // disable access for authorizationTokenSecond
  it('/graphql:M deleteUserAccessService - OK delete access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                userUuid: "${authorizationUserSecond}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessService=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteUserAccessService },
    } = body;
    expect(deleteUserAccessService).toBe(true);
    done();
  });

  it('/graphql:M deleteUserAccessService - BadRequest not found access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserAccessService(
              args: {
                serviceUuid: "${serviceUuidFirst}"
                userUuid: "${authorizationUserSecond}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessService=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access not found for user'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserAccessService');
    done();
  });

  it('/graphql:M putServiceUpdate - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putServiceUpdate(
              serviceUuid: "${serviceUuidFirst}"
              args: {
                name: "${nameService}",
                description: "${descriptionService}",
                regionId: ${regionId}
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
    expect(body.errors[0].path[0]).toBe('putServiceUpdate');
    done();
  });

  // Testing change service access
  it('/graphql:M changeServiceAccess - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            changeServiceAccess(args: {
              serviceUuid: "${serviceUuidSecond}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeServiceAccess=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeServiceAccess');
    done();
  });

  it('/graphql:M changeServiceAccess - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            changeServiceAccess(args: {
              serviceUuid: "${serviceUuidSecond}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeServiceAccess=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeServiceAccess },
    } = body;
    expect(changeServiceAccess).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Service - OK check change access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query serviceQuery{
            service(serviceUuid: "${serviceUuidSecond}") {
              uuid
              ownerUser {
                uuid
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { service },
    } = body;
    expect(service.uuid).toBe(serviceUuidSecond);
    expect(service.ownerUser.uuid).toBe(authorizationUserFirst);
    done();
  });

  // Testing transfer service ownership
  it('/graphql:M transferServiceOwnership - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            transferServiceOwnership(args: {
              serviceUuid: "${serviceUuidSecond}"
              newOwnerUserUuid: "${authorizationUserFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql transferServiceOwnership=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('transferServiceOwnership');
    done();
  });

  it('/graphql:M transferServiceOwnership - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            transferServiceOwnership(args: {
              serviceUuid: "${serviceUuidSecond}"
              newOwnerUserUuid: "${authorizationUserFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql transferServiceOwnership=%o', body);
    // expect(body).toBe(0);
    const {
      data: { transferServiceOwnership },
    } = body;
    expect(transferServiceOwnership).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Service - OK check change owner', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query serviceQuery{
            service(serviceUuid: "${serviceUuidSecond}") {
              uuid
              ownerUser {
                uuid
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { service },
    } = body;
    expect(service.uuid).toBe(serviceUuidSecond);
    expect(service.ownerUser.uuid).toBe(authorizationUserFirst);
    done();
  });

  // Testing delete service data
  it('/graphql:M deleteService - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteService(serviceUuid: "${serviceUuidFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M deleteService - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteService(serviceUuid: "${serviceUuidSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M deleteService - OK delete second service', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteService(serviceUuid: "${serviceUuidSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
      // expect(body).toBe(0);
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M deleteService - OK delete first', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteService(serviceUuid: "${serviceUuidFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteService=%o', body);
    // expect(body).toBe(0);
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M deleteService - BadRequest data not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteService(serviceUuid: "${serviceUuidSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    done();
  });

  // Testing get service statuses
  it('/graphql:Q serviceStatuses - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          serviceStatuses {
            serviceStatusId
            langId
            name
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found");
    done();
  });

  it('/graphql:Q serviceStatuses - OK get all', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          serviceStatuses {
            serviceStatusId
            langId
            name
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {data: { serviceStatuses }} = body;
    expect(serviceStatuses).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q serviceStatuses - OK get with filter', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          serviceStatuses(
            filter: [1,3,5555]
          ){
            serviceStatusId
            langId
            name
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {data: { serviceStatuses }} = body;
    expect(serviceStatuses[1].serviceStatusId).toBe(3);
    expect(serviceStatuses.length).toBe(2);
    done();
  });
});
