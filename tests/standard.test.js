const debug = require('debug')('cdbs-back:standard.test.js');
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

// data for standard
const parentStandardUuid = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const classifierStandard = "GOST-2012-Test";
const nameStandard = "GOST 2012 Test standard";
const descriptionStandard = "Test GOST standard";
const specifiedTolerance = "C";
const technicalCommittee = "GOST";
const publicationAt = "2021-07-31T00:00:00";
const standardStatusId = 1;
const regionId = 5;
const classifierStandard2 = "GOST-2012-Test 2222";
const nameStandard2 = "GOST 2012 Test standard 2222";
const descriptionStandard2 = "Test GOST standard 2222";
const specifiedTolerance2 = "C 2222";
const technicalCommittee2 = "GOST 2222";
const publicationAt2 = "2011-08-31T00:00:00";
const standardStatusId2 =  3;
const regionId2 = 5;
var standardUuidFirst = "";
var standardUuidSecond = "";

const standardFullDataQuery = ` \
uuid \
parentStandardUuid \
classifier \
name \
description \
specifiedTolerance \
technicalCommittee \
publicationAt \
imageFile { \
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
typeAccess {
  typeAccessId
  langId
  name
} \
standardStatus { \
  standardStatusId \
  langId \
  name \
} \
region { \
  regionId \
  langId \
  region \
} \
standardFiles { \
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
standardSpecs { \
  specId \
  langId \
  spec \
} \
standardKeywords { \
  id \
	keyword \
} \
subscribers \
isFollowed \
createdAt \
updatedAt \
`;

const standardsListQuery = ` \
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
imageFile { \
  uuid \
  filename \
  filesize \
  downloadUrl \
} \
updatedAt \
isFollowed  \
`;

const showFileRevisionsQuery = ` \
uuid \
filename \
revision \
parentFileUuid \
ownerUser { \
  username \
} \
`;

const showStandardFilesQuery = ` \
standardFiles { \
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

// standard type access
const typeAccessId3 = 3;
const typeAccessId2 = 2;
const typeAccessId1 = 1;
const typesAccessIds23 = [2,3];

// standard keywords
const keywordIdsOk = [1,3,5];
const keywordIdsDup = [1,2,3,4,5];
const keywordNames = ["asd2","asd3","asd4"];
const keywordNamesDup = ["asd2","asd3","asd4","asd5","asd6"];
const keywordNamesBad = ["asd11","asd12345678","asd12"];

// standard specs
const specIdsOk = [10,30,55];
const specIdsDup = [10,22,30,44,55];
const specIdsDel = [10,55];
const idErr = 0;

// standard files
const filename1 = "file-test-name 1.pdf";
const filename2 = "file-test-name 2.pdf";
const filename3 = "file-test-name 3.pdf";
const filename4 = "file-test-name 4.pdf";
const filename5 = "file-test-name 5.pdf";

const descriptionStandardFileTest = "test desctiption for standard";
const filenameStandardFileTest = "second name file for standard.pdf";
const badFilenameStandardFileTest = "name* file/ standard.pdf";
const goodFilenameStandardFileTest = "name file standard.pdf";
const badFilenameStandardFaviconTest = "no image file.pdf";
const goodFilenameStandardFaviconTest = "image file.png";

var fileStandardFileTestUuid = "";
var fileStandardFileTestUuid2 = "";
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

describe('company', () => {
  beforeAll(() => {
    cleanupCompanyRepresentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
    // cleanupTokenDb();
    cleanupUserDb();
    cleanupKeywordsDb();
    return;
  });
  afterAll(() => {
    cleanupCompanyRepresentDb();
    cleanupStandardDb();
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

  it('/graphql:M registerStandard - BadRequest without token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation standardQuery {
          registerStandard(args: {
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
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found.");
    done();
  });

  it('/graphql:M registerStandard - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard(args: {
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
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
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
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard).toBeNonEmptyString();
    standardUuidFirst = registerStandard;
    done();
  });

  it('/graphql:M registerStandard - OK not set parent', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard(args: {
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            companyUuid: "${companyUuidSupplier}",
            typeAccessId: ${typeAccessId1},
            standardStatusId: ${standardStatusId},
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard).toBeNonEmptyString();
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
            parentStandardUuid: "${standardUuidFirst}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            companyUuid: "${companyUuidSupplier}",
            typeAccessId: ${typeAccessId1},
            standardStatusId: ${standardStatusId},
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard).toBeNonEmptyString();
    standardUuidSecond = registerStandard;
    done();
  });

  it('/graphql:M registerStandard - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard(args: {
            parentStandardUuid: "${standardUuidFirst}",
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

  it('/graphql:M registerStandard - BadRequest not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard(args: {
            parentStandardUuid: "${standardUuidFirst}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            companyUuid: "${companyUuidNoSupplier}",
            typeAccessId: ${typeAccessId3},
            standardStatusId: ${standardStatusId},
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: The company is not supplier.");
    done();
  });

  // Testing update standard data
  it('/graphql:M putStandardUpdate - OK rand data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putStandardUpdate(
            standardUuid: "${standardUuidSecond}"
            args: {
              classifier: "BES-test-2021",
              name: "name test for upda",
              description: "description test for upda",
              specifiedTolerance: "A",
              technicalCommittee: "GMONSTER",
              publicationAt: "1994-01-01T00:00:00",
              standardStatusId: 2,
              regionId: 3
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putStandardUpdate },
    } = body;
    expect(putStandardUpdate).toBe(8);
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          putStandardUpdate(
            standardUuid: "${standardUuidSecond}"
            args: {
              classifier: "${classifierStandard}",
              name: "${nameStandard}",
              description: "${descriptionStandard}",
              specifiedTolerance: "${specifiedTolerance}",
              technicalCommittee: "${technicalCommittee}",
              publicationAt: "${publicationAt}",
              companyUuid: "${companyUuidSupplier}",
              standardStatusId: ${standardStatusId},
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

  it('/graphql:M putStandardUpdate - BadRequest not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putStandardUpdate(
            standardUuid: "${standardUuidSecond}"
            args: {
              classifier: "${classifierStandard}",
              name: "${nameStandard}",
              description: "${descriptionStandard}",
              specifiedTolerance: "${specifiedTolerance}",
              technicalCommittee: "${technicalCommittee}",
              publicationAt: "${publicationAt}",
              companyUuid: "${companyUuidNoSupplier}",
              standardStatusId: ${standardStatusId},
              regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: The company is not supplier.");
    done();
  });

  it('/graphql:M putStandardUpdate - OK return old data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putStandardUpdate(
            standardUuid: "${standardUuidSecond}"
            args: {
              classifier: "${classifierStandard}",
              name: "${nameStandard}",
              description: "${descriptionStandard}",
              specifiedTolerance: "${specifiedTolerance}",
              technicalCommittee: "${technicalCommittee}",
              publicationAt: "${publicationAt}",
              standardStatusId: ${standardStatusId},
              regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    const {
      data: { putStandardUpdate },
    } = body;
    expect(putStandardUpdate).toBe(8);
    done();
  });

  it('/graphql:M putStandardUpdate - OK data already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          putStandardUpdate(
            standardUuid: "${standardUuidSecond}"
            args: {
              classifier: "${classifierStandard}",
              name: "${nameStandard}",
              description: "${descriptionStandard}",
              specifiedTolerance: "${specifiedTolerance}",
              technicalCommittee: "${technicalCommittee}",
              publicationAt: "${publicationAt}",
              standardStatusId: ${standardStatusId},
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

  // Testing change main image (favicon) for standard
  it('/graphql:M uploadStandardFavicon - BadRequest not access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadStandardFavicon(args: {
            standardUuid: "${standardUuidSecond}"
            filename: "${badFilenameStandardFaviconTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFavicon=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadStandardFavicon');
    done();
  });

  it('/graphql:M uploadStandardFavicon - BadRequest not image', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFavicon(args: {
            standardUuid: "${standardUuidSecond}"
            filename: "${badFilenameStandardFaviconTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFavicon=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Selected file is not image.'
    );
    expect(body.errors[0].path[0]).toBe('uploadStandardFavicon');
    done();
  });

  it('/graphql:M uploadStandardFavicon - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFavicon(args: {
            standardUuid: "${standardUuidSecond}"
            filename: "${goodFilenameStandardFaviconTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFavicon=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFavicon },
    } = body;
    changeStandardFaviconTestUuid = uploadStandardFavicon.fileUuid;
    expect(uploadStandardFavicon.fileUuid).toBeNonEmptyString();
    expect(uploadStandardFavicon.filename).toBe(goodFilenameStandardFaviconTest);
    expect(uploadStandardFavicon.uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(changeStandardFaviconTestUuid);
    done();
  });

  it('/graphql:Q List standards - OK check change main image', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          standards (args: {
            standardsUuids: "${standardUuidSecond}"
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards[0].uuid).toBe(standardUuidSecond);
    expect(standards[0].imageFile.uuid).toBe(changeStandardFaviconTestUuid);
    expect(standards[0].imageFile.filename).toBe(goodFilenameStandardFaviconTest);
    expect(standards.length).toBe(1);
    done();
  });

  it('/graphql:M uploadStandardFavicon - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadStandardFavicon(args: {
            standardUuid: "${standardUuidSecond}"
            filename: "${badFilenameStandardFaviconTest}"
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

  // Testing add files for standard
  it('/graphql:M uploadStandardFiles - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: "${badFilenameStandardFileTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('uploadStandardFiles');
    done();
  });

  it('/graphql:M uploadStandardFiles - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${badFilenameStandardFileTest}"
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    fileStandardFileTestUuid = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(goodFilenameStandardFileTest);
    expect(uploadStandardFiles[0].uploadUrl).toBeNonEmptyString();
    fileStandardFileTestUuid2 = uploadStandardFiles[1].fileUuid;
    expect(uploadStandardFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[1].filename).toBe(filenameStandardFileTest);
    expect(uploadStandardFiles[1].uploadUrl).toBeNonEmptyString();
    await setFileAsUploadedDb(fileStandardFileTestUuid);
    await setFileAsUploadedDb(fileStandardFileTestUuid2);
    done();
  });

  it('/graphql:M uploadStandardFiles - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: "${badFilenameStandardFileTest}"
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

  // Testing get standard files
  it('/graphql:Q standardFiles - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          standardFiles(args: {
            standardUuid: "${standardUuidSecond}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standardFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('standardFiles');
    done();
  });

  it('/graphql:Q standardFiles - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          standardFiles(args: {
            standardUuid: "${standardUuidSecond}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standardFiles },
    } = body;
    expect(standardFiles[0].uuid).toBe(fileStandardFileTestUuid);
    expect(standardFiles[1].uuid).toBe(fileStandardFileTestUuid2);
    expect(standardFiles.length).toBe(2);
    done();
  });

  it('/graphql:Q standardFiles - Ok filter by uuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          standardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filesUuids: "${fileStandardFileTestUuid2}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standardFiles },
    } = body;
    expect(standardFiles[0].uuid).toBe(fileStandardFileTestUuid2);
    expect(standardFiles.length).toBe(1);
    done();
  });

  it('/graphql:Q standardFiles - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          standardFiles(args: {
            standardUuid: "${standardUuidSecond}"
          }) {
            uuid
            filename
            filesize
            downloadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standardFiles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('standardFiles');
    done();
  });

  // Testing delete files of standard
  it('/graphql:M deleteStandardFile - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          deleteStandardFile(args: {
            standardUuid: "${standardUuidSecond}"
            fileUuid: "${fileStandardFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardFile=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardFile');
    done();
  });

  it('/graphql:M deleteStandardFile - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteStandardFile(args: {
            standardUuid: "${standardUuidSecond}"
            fileUuid: "${fileStandardFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardFile=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteStandardFile },
    } = body;
    expect(deleteStandardFile).toBe(true);
    done();
  });

  it('/graphql:M deleteStandardFile - Ok not found file', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteStandardFile(args: {
            standardUuid: "${standardUuidSecond}"
            fileUuid: "${fileStandardFileTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardFile=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteStandardFile },
    } = body;
    expect(deleteStandardFile).toBe(false);
    done();
  });

  it('/graphql:M deleteStandardFile - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteStandardFile(args: {
            standardUuid: "${standardUuidSecond}"
            fileUuid: "${fileStandardFileTestUuid}"
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
  it('/graphql:Q Get full data Standard - OK check add/del files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query standard {
          standard (standardUuid: "${standardUuidSecond}"){
            ${standardFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standard },
    } = body;
    expect(standard.uuid).toBe(standardUuidSecond);
    expect(standard.standardFiles[0].uuid).toBe(fileStandardFileTestUuid2);
    expect(standard.standardFiles[0].filename).toBe(filenameStandardFileTest);
    expect(standard.standardFiles[0].parentFileUuid).toBeNonEmptyString();
    expect(standard.standardFiles[0].ownerUser.uuid).toBe(authorizationUserFirst);
    expect(standard.standardFiles[0].contentType).toBeNonEmptyString();
    done();
  });

  // Testing new revisions
  it('/graphql:M uploadStandardFiles - Ok new revision', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${goodFilenameStandardFileTest}"
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    seconRevFileFileTestUuid = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(goodFilenameStandardFileTest);
    seconRevFileFileTestUuid2 = uploadStandardFiles[1].fileUuid;
    expect(uploadStandardFiles[1].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[1].filename).toBe(filenameStandardFileTest);
    await setFlagHiddenAsOldRevDb(fileStandardFileTestUuid);
    await setFileAsUploadedDb(seconRevFileFileTestUuid);
    await setFlagHiddenAsOldRevDb(fileStandardFileTestUuid2);
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
    // expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid);
    expect(showFileRevisions[0].filename).toBe(goodFilenameStandardFileTest);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions.length).toBe(1);
    done();
  });

  it('/graphql:M uploadStandardFiles - Ok new revision 3', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    threeRevFileFileTestUuid2 = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(filenameStandardFileTest);
    expect(uploadStandardFiles[0].uploadUrl).toBeNonEmptyString();
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
    expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid2);
    expect(showFileRevisions[0].revision).toBe(1);
    expect(showFileRevisions[1].uuid).toBe(seconRevFileFileTestUuid2);
    expect(showFileRevisions[1].revision).toBe(2);
    expect(showFileRevisions[2].uuid).toBe(threeRevFileFileTestUuid2);
    expect(showFileRevisions[2].revision).toBe(3);
    expect(showFileRevisions.length).toBe(3);
    await setFlagDeleteAsOldRevDb(threeRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadStandardFiles - Ok new revision 4', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    fourthRevFileFileTestUuid2 = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(filenameStandardFileTest);
    expect(uploadStandardFiles[0].uploadUrl).toBeNonEmptyString();
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
    expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid2);
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

  it('/graphql:M uploadStandardFiles - BadRequest stranger standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadStandardFiles');
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
    expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid2);
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

  it('/graphql:M changeActiveFileRevision - BadRequest stranger standard', async (done) => {
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

  it('/graphql:M uploadStandardFiles - Ok new revision 5', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    fifthRevFileFileTestUuid2 = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(filenameStandardFileTest);
    expect(uploadStandardFiles[0].uploadUrl).toBeNonEmptyString();
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
    expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid2);
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

  it('/graphql:Q Get all files of Standard - OK check parent files', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query standard {
          standard (standardUuid: "${standardUuidSecond}"){
            ${showStandardFilesQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql showStandardFilesQuery=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standard },
    } = body;
    expect(standard.standardFiles[1].uuid).toBe(fifthRevFileFileTestUuid2);
    expect(standard.standardFiles[1].filename).toBe(filenameStandardFileTest);
    expect(standard.standardFiles[1].revision).toBe(4);
    // expect(standard.standardFiles[1].parentFileUuid).toBe(seconRevFileFileTestUuid2);
    expect(standard.standardFiles[1].parentFileUuid).toBe(fourthRevFileFileTestUuid2);
    expect(standard.standardFiles.length).toBe(2);
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
    await setFlagHiddenAsOldRevDb(fileStandardFileTestUuid2);
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
    expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid2);
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

  it('/graphql:M uploadStandardFiles - Ok new revision 6 other versions are hidden', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    sixthRevFileFileTestUuid2 = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(filenameStandardFileTest);
    expect(uploadStandardFiles[0].uploadUrl).toBeNonEmptyString();
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
    expect(showFileRevisions[0].uuid).toBe(fileStandardFileTestUuid2);
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
    await setFlagDeleteAsOldRevDb(fileStandardFileTestUuid2);
    await setFlagDeleteAsOldRevDb(seconRevFileFileTestUuid2);
    // await setFlagDeleteAsOldRevDb(fourthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(fifthRevFileFileTestUuid2);
    await setFlagDeleteAsOldRevDb(sixthRevFileFileTestUuid2);
    done();
  });

  it('/graphql:M uploadStandardFiles - Ok new revision 7 other versions are deleted', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadStandardFiles(args: {
            standardUuid: "${standardUuidSecond}"
            filenames: [
              "${filenameStandardFileTest}"
            ]
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadStandardFiles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadStandardFiles },
    } = body;
    seventhRevFileFileTestUuid2 = uploadStandardFiles[0].fileUuid;
    expect(uploadStandardFiles[0].fileUuid).toBeNonEmptyString();
    expect(uploadStandardFiles[0].filename).toBe(filenameStandardFileTest);
    expect(uploadStandardFiles[0].uploadUrl).toBeNonEmptyString();
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

  // Testing adding standard specs
  it('/graphql:M addStandardSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addStandardSpecs');
    done();
  });

  it('/graphql:M addStandardSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { addStandardSpecs },
    } = body;
    expect(addStandardSpecs).toBe(3);
    done();
  });

  it('/graphql:M addStandardSpecs - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { addStandardSpecs },
    } = body;
    expect(addStandardSpecs).toBe(2);
    done();
  });

  it('/graphql:M addStandardSpecs - BadRequest all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: This ids [10, 30, 55] already has"
    );
    expect(body.errors[0].path[0]).toBe('addStandardSpecs');
    done();
  });

  it('/graphql:M addStandardSpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('addStandardSpecs');
    done();
  });

  it('/graphql:M addStandardSpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addStandardSpecs');
    done();
  });

  it('/graphql:Q Get full data Standard - OK check add specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query standard {
          standard (standardUuid: "${standardUuidFirst}"){
            ${standardFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standard },
    } = body;
    expect(standard.standardSpecs[0].specId).toBe(10);
    expect(standard.standardSpecs[0].spec).toBeNonEmptyString();
    expect(standard.standardSpecs[1].specId).toBe(22);
    expect(standard.standardSpecs[1].spec).toBeNonEmptyString();
    expect(standard.standardSpecs[2].specId).toBe(30);
    expect(standard.standardSpecs[2].spec).toBeNonEmptyString();
    expect(standard.standardSpecs[3].specId).toBe(44);
    expect(standard.standardSpecs[3].spec).toBeNonEmptyString();
    expect(standard.standardSpecs[4].specId).toBe(55);
    expect(standard.standardSpecs[4].spec).toBeNonEmptyString();
    done();
  });

  // Testing get specs for standard
  it('/graphql:Q standardSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          standardSpecs(args: {
            standardUuid: "${standardUuidSecond}"
          }){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('standardSpecs');
    done();
  });

  it('/graphql:Q standardSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          standardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
          }){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standardSpecs },
    } = body;
    expect(standardSpecs.length).toBe(5);
    expect(standardSpecs[0].specId).toBe(10);
    expect(standardSpecs[0].spec).toBeNonEmptyString();
    expect(standardSpecs[1].specId).toBe(22);
    expect(standardSpecs[1].spec).toBeNonEmptyString();
    expect(standardSpecs[2].specId).toBe(30);
    expect(standardSpecs[2].spec).toBeNonEmptyString();
    expect(standardSpecs[3].specId).toBe(44);
    expect(standardSpecs[3].spec).toBeNonEmptyString();
    expect(standardSpecs[4].specId).toBe(55);
    expect(standardSpecs[4].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q standardSpecs - OK with limit and offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          standardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            limit: 1
            offset: 3
          }){
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
      data: { standardSpecs },
    } = body;
    expect(standardSpecs.length).toBe(1);
    expect(standardSpecs[0].specId).toBe(22);
    expect(standardSpecs[0].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q standardSpecs - OK not found specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          standardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            limit: 50
            offset: 500
          }){
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
      data: { standardSpecs },
    } = body;
    expect(standardSpecs).toBeEmptyArray();
    done();
  });

  // Testing delete standard specs
  it('/graphql:M deleteStandardSpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardSpecs');
    done();
  });

  it('/graphql:M deleteStandardSpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { deleteStandardSpecs },
    } = body;
    expect(deleteStandardSpecs).toBe(2);
    done();
  });

  it('/graphql:M deleteStandardSpecs - OK data already delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { deleteStandardSpecs },
    } = body;
    expect(deleteStandardSpecs).toBe(0);
    done();
  });

  it('/graphql:M deleteStandardSpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardSpecs');
    done();
  });

  it('/graphql:M deleteStandardSpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteStandardSpecs(args: {
            standardUuid: "${standardUuidFirst}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardSpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardSpecs');
    done();
  });

  it('/graphql:Q Get full data Standard - OK check delete specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query standard {
          standard (standardUuid: "${standardUuidFirst}"){
            ${standardFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standard },
    } = body;
    expect(standard.standardSpecs.length).toBe(3);
    expect(standard.standardSpecs[0].specId).toBe(22);
    expect(standard.standardSpecs[0].spec).toBeNonEmptyString();
    expect(standard.standardSpecs[1].specId).toBe(30);
    expect(standard.standardSpecs[1].spec).toBeNonEmptyString();
    expect(standard.standardSpecs[2].specId).toBe(44);
    expect(standard.standardSpecs[2].spec).toBeNonEmptyString();
    done();
  });

  // Testing adding standard keywords
  it('/graphql:M addStandardKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addStandardKeywords(args: {
            standardUuid: "${standardUuidSecond}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addStandardKeywords');
    done();
  });

  it('/graphql:M addStandardKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addStandardKeywords },
    } = body;
    expect(addStandardKeywords).toBe(3);
    done();
  });

  it('/graphql:M addStandardKeywords - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${keywordIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addStandardKeywords },
    } = body;
    expect(addStandardKeywords).toBe(2);
    done();
  });

  it('/graphql:M addStandardKeywords - OK all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywords=%o', body);
    const {
      data: { addStandardKeywords },
    } = body;
    expect(addStandardKeywords).toBe(0);
    done();
  });

  it('/graphql:M addStandardKeywords - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found keywords"
    );
    expect(body.errors[0].path[0]).toBe('addStandardKeywords');
    done();
  });

  it('/graphql:M addStandardKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addStandardKeywords');
    done();
  });

  // Testing adding standard keywords by names
  it('/graphql:M addStandardKeywordsByNames - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addStandardKeywordsByNames(args: {
            standardUuid: "${standardUuidSecond}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywordsByNames=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('addStandardKeywordsByNames');
    done();
  });

  it('/graphql:M addStandardKeywordsByNames - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywordsByNames(args: {
            standardUuid: "${standardUuidFirst}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addStandardKeywordsByNames },
    } = body;
    expect(addStandardKeywordsByNames).toBe(3);
    done();
  });

  it('/graphql:M addStandardKeywordsByNames - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywordsByNames(args: {
            standardUuid: "${standardUuidFirst}"
            keywords: ["asd2","asd3","asd4","asd5","asd6"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { addStandardKeywordsByNames },
    } = body;
    expect(addStandardKeywordsByNames).toBe(2);
    done();
  });

  it('/graphql:M addStandardKeywordsByNames - OK all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywordsByNames(args: {
            standardUuid: "${standardUuidFirst}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywordsByNames=%o', body);
    const {
      data: { addStandardKeywordsByNames },
    } = body;
    expect(addStandardKeywordsByNames).toBe(0);
    done();
  });

  it('/graphql:M addStandardKeywordsByNames - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addStandardKeywordsByNames(args: {
            standardUuid: "${standardUuidFirst}"
            keywords: ["asd11","asd12345678","asd12"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywordsByNames=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Keywords must be less than 10 symbols"
    );
    expect(body.errors[0].path[0]).toBe('addStandardKeywordsByNames');
    done();
  });

  it('/graphql:M addStandardKeywordsByNames - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addStandardKeywordsByNames(args: {
            standardUuid: "${standardUuidFirst}"
            keywords: ["asd2","asd3","asd4"]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardKeywordsByNames=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addStandardKeywordsByNames');
    done();
  });

  it('/graphql:Q Get full data Standard - OK check add keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query standardQuery{
            standard(standardUuid: "${standardUuidFirst}") {
              ${standardFullDataQuery}
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql filter standard=%o', body.data.standard);
    expect(body.data.standard.uuid).toBe(standardUuidFirst);
    expect(body.data.standard.standardKeywords[0].id).toBe(1);
    expect(body.data.standard.standardKeywords[0].keyword).toBeNonEmptyString();
    expect(body.data.standard.standardKeywords[1].id).toBe(2);
    expect(body.data.standard.standardKeywords[1].keyword).toBeNonEmptyString();
    expect(body.data.standard.standardKeywords[2].id).toBe(3);
    expect(body.data.standard.standardKeywords[2].keyword).toBeNonEmptyString();
    expect(body.data.standard.standardKeywords[3].id).toBe(4);
    expect(body.data.standard.standardKeywords[3].keyword).toBeNonEmptyString();
    expect(body.data.standard.standardKeywords[4].id).toBe(5);
    expect(body.data.standard.standardKeywords[4].keyword).toBeNonEmptyString();
    done();
  });

  // Testing get keywords for standard
  it('/graphql:Q standardKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          standardKeywords(args: {
            standardUuid: "${standardUuidSecond}"
          }){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql standardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('standardKeywords');
    done();
  });

  it('/graphql:Q standardKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          standardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
          }){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standardKeywords },
    } = body;
    expect(standardKeywords[0].id).toBe(1);
    expect(standardKeywords[0].keyword).toBeNonEmptyString();
    expect(standardKeywords[1].id).toBe(2);
    expect(standardKeywords[1].keyword).toBeNonEmptyString();
    expect(standardKeywords[2].id).toBe(3);
    expect(standardKeywords[2].keyword).toBeNonEmptyString();
    expect(standardKeywords[3].id).toBe(4);
    expect(standardKeywords[3].keyword).toBeNonEmptyString();
    expect(standardKeywords[4].id).toBe(5);
    expect(standardKeywords[4].keyword).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q standardKeywords - OK with limit and offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          standardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            limit: 2
            offset: 3
          }){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standardKeywords },
    } = body;
    expect(standardKeywords.length).toBe(2);
    expect(standardKeywords[0].id).toBe(2);
    expect(standardKeywords[0].keyword).toBeNonEmptyString();
    expect(standardKeywords[1].id).toBe(4);
    expect(standardKeywords[1].keyword).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q standardKeywords - OK not found keywords', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          standardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            limit: 5
            offset: 500
          }){
            id
            keyword
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standardKeywords },
    } = body;
    expect(standardKeywords).toBeEmptyArray();
    done();
  });

  // Testing delete standard keywords
  it('/graphql:M deleteStandardKeywords - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardKeywords');
    done();
  });

  it('/graphql:M deleteStandardKeywords - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${keywordIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardKeywords=%o', body);
    const {
      data: { deleteStandardKeywords },
    } = body;
    expect(deleteStandardKeywords).toBe(3);
    done();
  });

  it('/graphql:M deleteStandardKeywords - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found keywords"
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardKeywords');
    done();
  });

  it('/graphql:M deleteStandardKeywords - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteStandardKeywords(args: {
            standardUuid: "${standardUuidFirst}"
            keywordIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardKeywords=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('deleteStandardKeywords');
    done();
  });

  // Testing get standards data
  it('/graphql:Q Get full data Standard - BadRequest without token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            standardsUuids: "${standardUuidSecond}"
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found.");
    done();
  });

  it('/graphql:Q Get full data Standard - OK ShowStandardShort', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectStandardQuery{
          standard (standardUuid: "${standardUuidFirst}") {
            ${standardFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standard },
    } = body;
    expect(standard.uuid).toBe(standardUuidFirst);
    expect(standard.classifier).toBe(classifierStandard);
    done();
  });

  it('/graphql:Q Get full data Standard - OK Select with uuid (public access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            standardsUuids: ["${standardUuidFirst}"]
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standards },
    } = body;
    expect(standards[0].uuid).toBe(standardUuidFirst);
    expect(standards[0].classifier).toBe(classifierStandard);
    done();
  });

  it('/graphql:Q Get full data Standard - OK no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            standardsUuids: "${standardUuidSecond}"
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const {
      data: { standards },
    } = body;
    expect(standards).toBeEmptyArray();
    done();
  });

  it('/graphql:Q List standards - BadRequest not correct arguments', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectComponentQuery{
          standards (args: {
            companyUuid: "${companyUuidNoSupplier}"
            favorite:  true
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessStandard=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed match arguments'
    );
    expect(body.errors[0].path[0]).toBe('standards');
    done();
  });

  it('/graphql:Q Get full data Standard - OK without arguments', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards).toBeNonEmptyArray();
    index = standards.length - 1;
    expect(standards[index].uuid).toBe(standardUuidFirst);
    expect(standards[index].name).toBe(nameStandard);
    done();
  });

  it('/graphql:Q Get full data Standard - OK by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            companyUuid: "${companyUuidSupplier}"
          }){
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards).toBeNonEmptyArray();
    expect(standards[0].uuid).toBe(standardUuidFirst);
    expect(standards[0].name).toBe(nameStandard);
    done();
  });

  it('/graphql:Q Get full data Standard - OK by company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            companyUuid: "${companyUuidNoSupplier}"
          }){
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards).toBeEmptyArray();
    done();
  });

  // Testing add access for company
  it('/graphql:M setCompanyAccessStandard - OK add low access company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
                typeAccessId: ${typeAccessId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setCompanyAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyAccessStandard },
    } = body;
    expect(setCompanyAccessStandard).toBe(true);
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest need higher access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard}",
                description: "${descriptionStandard}",
                specifiedTolerance: "${specifiedTolerance}",
                technicalCommittee: "${technicalCommittee}",
                publicationAt: "${publicationAt}",
                standardStatusId: ${standardStatusId},
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
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  it('/graphql:M setCompanyAccessStandard - OK add access company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
                typeAccessId: ${typeAccessId1}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setCompanyAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyAccessStandard },
    } = body;
    expect(setCompanyAccessStandard).toBe(true);
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

  it('/graphql:Q getCompaniesListAccessStandard - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            getCompaniesListAccessStandard(
              standardUuid: "${standardUuidFirst}"
            ) {
              standardUuid
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
    debug('/graphql getCompaniesListAccessStandard=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('getCompaniesListAccessStandard');
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
    expect(addAccessRole).toBe(true);
    done();
  });

  it('/graphql:Q getCompaniesListAccessStandard - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            getCompaniesListAccessStandard(
              standardUuid: "${standardUuidFirst}"
            ) {
              standardUuid
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
    debug('/graphql getCompaniesListAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { getCompaniesListAccessStandard },
    } = body;
    expect(getCompaniesListAccessStandard[0].standardUuid).toBe(standardUuidFirst);
    expect(getCompaniesListAccessStandard[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(getCompaniesListAccessStandard[0].typeAccess.typeAccessId).toBe(typeAccessId1);
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest not access for change company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard}",
                description: "${descriptionStandard}",
                specifiedTolerance: "${specifiedTolerance}",
                technicalCommittee: "${technicalCommittee}",
                publicationAt: "${publicationAt}",
                companyUuid: "${companyUuidNoSupplier}",
                standardStatusId: ${standardStatusId},
                regionId: ${regionId}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  it('/graphql:M putStandardUpdate - OK with access from company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard2}",
                name: "${nameStandard2}",
                description: "${descriptionStandard2}",
                specifiedTolerance: "${specifiedTolerance2}",
                technicalCommittee: "${technicalCommittee2}",
                publicationAt: "${publicationAt2}",
                standardStatusId: ${standardStatusId2},
                regionId: ${regionId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putStandardUpdate },
    } = body;
    expect(putStandardUpdate).toBe(7);
    done();
  });

  // disable access for authorizationTokenSecond
  it('/graphql:M deleteCompanyAccessStandard - OK delete access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyAccessStandard },
    } = body;
    expect(deleteCompanyAccessStandard).toBe(true);
    done();
  });

  it('/graphql:M deleteCompanyAccessStandard - BadRequest not found access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                companyUuid: "${companyUuidNoSupplier}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyAccessStandard=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access not found for company'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyAccessStandard');
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard}",
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
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest no access for company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard}",
                description: "${descriptionStandard}",
                specifiedTolerance: "${specifiedTolerance}",
                technicalCommittee: "${technicalCommittee}",
                publicationAt: "${publicationAt}",
                companyUuid: "${companyUuidNoSupplier}",
                standardStatusId: ${standardStatusId},
                regionId: ${regionId}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  // Testing add access for user
  // add access for authorizationTokenSecond
  it('/graphql:M setUserAccessStandard - OK add low access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                userUuid: "${authorizationUserSecond}"
                typeAccessId: ${typeAccessId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setUserAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setUserAccessStandard },
    } = body;
    expect(setUserAccessStandard).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Standard - OK Select with fake uuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            standardsUuids: "${uuidFake}"
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards).toBeEmptyArray();
    done();
  });

  it('/graphql:Q Get full data Standard - OK Select with uuid (private access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (args: {
            standardsUuids: "${standardUuidFirst}"
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards[0].uuid).toBe(standardUuidFirst);
    expect(standards[0].classifier).toBe(classifierStandard2);
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest need higher access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard}",
                description: "${descriptionStandard}",
                specifiedTolerance: "${specifiedTolerance}",
                technicalCommittee: "${technicalCommittee}",
                publicationAt: "${publicationAt}",
                companyUuid: "${companyUuidNoSupplier}",
                standardStatusId: ${standardStatusId},
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
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  it('/graphql:M setUserAccessStandard - OK add access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                userUuid: "${authorizationUserSecond}"
                typeAccessId: ${typeAccessId1}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql setUserAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setUserAccessStandard },
    } = body;
    expect(setUserAccessStandard).toBe(true);
    done();
  });

  it('/graphql:Q getUsersListAccessStandard - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            getUsersListAccessStandard(
              standardUuid: "${standardUuidFirst}"
            ) {
              standardUuid
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
    debug('/graphql getUsersListAccessStandard=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('getUsersListAccessStandard');
    done();
  });

  it('/graphql:Q getUsersListAccessStandard - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            getUsersListAccessStandard(
              standardUuid: "${standardUuidFirst}"
            ) {
              standardUuid
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
    debug('/graphql getUsersListAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { getUsersListAccessStandard },
    } = body;
    expect(getUsersListAccessStandard[0].standardUuid).toBe(standardUuidFirst);
    expect(getUsersListAccessStandard[0].userUuid).toBe(authorizationUserSecond);
    expect(getUsersListAccessStandard[0].typeAccess.typeAccessId).toBe(typeAccessId1);
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard2}",
                name: "${nameStandard2}",
                description: "${descriptionStandard2}",
                specifiedTolerance: "${specifiedTolerance2}",
                technicalCommittee: "${technicalCommittee2}",
                publicationAt: "${publicationAt2}",
                companyUuid: "${companyUuidNoSupplier}",
                standardStatusId: ${standardStatusId2},
                regionId: ${regionId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  it('/graphql:M putStandardUpdate - OK with access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard2}",
                description: "${descriptionStandard}",
                specifiedTolerance: "${specifiedTolerance2}",
                technicalCommittee: "${technicalCommittee}",
                publicationAt: "${publicationAt2}",
                standardStatusId: ${standardStatusId},
                regionId: ${regionId2}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putStandardUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putStandardUpdate },
    } = body;
    expect(putStandardUpdate).toBe(4);
    done();
  });

  // Testing favorite standards search
  it('/graphql:M StandardFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            addStandardFav(standardUuid: "${standardUuidFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardFav body=%o', body);
    expect(body.data.addStandardFav).toBe(true);
    done();
  });

  it('/graphql:Q Fav list standards - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          standards (args: {
            favorite: true
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards[0].uuid).toBe(standardUuidFirst);
    expect(standards[0].name).toBe(nameStandard2);
    expect(standards[0].isFollowed).toBe(true);
    done();
  });

  it('/graphql:M StandardFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            deleteStandardFav(standardUuid: "${standardUuidFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardFav body=%o', body);
    expect(body.data.deleteStandardFav).toBe(true);
    done();
  });

  it('/graphql:Q Fav list standards - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          standards (args: {
            favorite: true
          }) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standards },
    } = body;
    expect(standards).toBeEmptyArray();
    done();
  });

  // disable access for authorizationTokenSecond
  it('/graphql:M deleteUserAccessStandard - OK delete access user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                userUuid: "${authorizationUserSecond}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteUserAccessStandard },
    } = body;
    expect(deleteUserAccessStandard).toBe(true);
    done();
  });

  it('/graphql:M deleteUserAccessStandard - BadRequest not found access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserAccessStandard(
              args: {
                standardUuid: "${standardUuidFirst}"
                userUuid: "${authorizationUserSecond}"
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserAccessStandard=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access not found for user'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserAccessStandard');
    done();
  });

  it('/graphql:M putStandardUpdate - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putStandardUpdate(
              standardUuid: "${standardUuidFirst}"
              args: {
                classifier: "${classifierStandard}",
                name: "${nameStandard}",
                description: "${descriptionStandard}",
                specifiedTolerance: "${specifiedTolerance}",
                technicalCommittee: "${technicalCommittee}",
                publicationAt: "${publicationAt}",
                companyUuid: "${companyUuidNoSupplier}",
                standardStatusId: ${standardStatusId},
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
    expect(body.errors[0].path[0]).toBe('putStandardUpdate');
    done();
  });

  // Testing change standard access
  it('/graphql:M changeStandardAccess - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            changeStandardAccess(args: {
              standardUuid: "${standardUuidSecond}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeStandardAccess=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeStandardAccess');
    done();
  });

  it('/graphql:M changeStandardAccess - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            changeStandardAccess(args: {
              standardUuid: "${standardUuidSecond}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeStandardAccess=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeStandardAccess },
    } = body;
    expect(changeStandardAccess).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Standard - OK check change access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query standardQuery{
            standard(standardUuid: "${standardUuidSecond}") {
              uuid
              ownerUser {
                uuid
              }
              typeAccess{
                typeAccessId
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standard },
    } = body;
    expect(standard.uuid).toBe(standardUuidSecond);
    expect(standard.ownerUser.uuid).toBe(authorizationUserFirst);
    expect(standard.typeAccess.typeAccessId).toBe(typeAccessId2);
    done();
  });

  // Testing transfer standard ownership
  it('/graphql:M transferStandardOwnership - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            transferStandardOwnership(args: {
              standardUuid: "${standardUuidSecond}"
              newOwnerUserUuid: "${authorizationUserFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql transferStandardOwnership=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('transferStandardOwnership');
    done();
  });

  it('/graphql:M transferStandardOwnership - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            transferStandardOwnership(args: {
              standardUuid: "${standardUuidSecond}"
              newOwnerUserUuid: "${authorizationUserFirst}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql transferStandardOwnership=%o', body);
    // expect(body).toBe(0);
    const {
      data: { transferStandardOwnership },
    } = body;
    expect(transferStandardOwnership).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Standard - OK check change owner', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query standardQuery{
            standard(standardUuid: "${standardUuidSecond}") {
              uuid
              ownerUser {
                uuid
              }
              typeAccess{
                typeAccessId
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { standard },
    } = body;
    expect(standard.uuid).toBe(standardUuidSecond);
    expect(standard.ownerUser.uuid).toBe(authorizationUserFirst);
    expect(standard.typeAccess.typeAccessId).toBe(typeAccessId2);
    done();
  });

  // Testing delete standard data
  it('/graphql:M deleteStandard - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteStandard(standardUuid: "${standardUuidFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M deleteStandard - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteStandard(standardUuid: "${standardUuidSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Access denied");
    done();
  });

  it('/graphql:M deleteStandard - OK delete second standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteStandard(standardUuid: "${standardUuidSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
      // expect(body).toBe(0);
      const {
        data: { deleteStandard },
      } = body;
      expect(deleteStandard).toBe(standardUuidSecond);
      done();
  });

  it('/graphql:M deleteStandard - OK delete first', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteStandard(standardUuid: "${standardUuidFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandard=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteStandard },
    } = body;
    expect(deleteStandard).toBe(standardUuidFirst);
    done();
  });

  it('/graphql:M deleteStandard - BadRequest data not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteStandard(standardUuid: "${standardUuidSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      "BadRequest: Not found standard"
    );
    done();
  });

  // Testing get standard statuses
  it('/graphql:Q standardStatuses - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
          standardStatuses {
            standardStatusId
            langId
            name
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found.");
    done();
  });

  it('/graphql:Q standardStatuses - OK get all', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          standardStatuses {
            standardStatusId
            langId
            name
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {data: { standardStatuses }} = body;
    expect(standardStatuses).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q standardStatuses - OK get with filter', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          standardStatuses(
            filter: [1,3,5555]
          ){
            standardStatusId
            langId
            name
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {data: { standardStatuses }} = body;
    expect(standardStatuses[1].standardStatusId).toBe(3);
    expect(standardStatuses.length).toBe(2);
    done();
  });
});
