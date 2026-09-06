const debug = require('debug')('cdbs-back:company.test.js');
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
const userUuidBase = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const userUuid2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";

var authorizationUserFirst = "";
var authorizationTokenFirst = "";
var authorizationUserSecond = "";
var authorizationTokenSecond = "";

var firstAccess = 1;
var secondAccess = 2;

var langId = 1;
var nameRole = "test role";
var newRoleId = 0;
var nameRole2 = "test role2";
var nameRole2 = "test role2";
var newRoleId3 = 0;
var nameRole3 = "Test Role For Deletion";
let testRoleId = 0;

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

const specIdsOk = [10,30,55];
const specIdsDup = [10,22,30,44,55];
const specIdsDel = [10,55];
const idErr = 0;

const descriptionCertificateTest = "test desctiption for certificate";
const badFilenameCertificateTest = "name* file/ certificate.pdf";
const goodFilenameCertificateTest = "name file certificate.pdf";
const tooLongCertificateDescription = 'я'.repeat(501);

var fileCertificateTestUuid = "";

// company data in the database
const supplierCompany1t = "fde011eb-0995-4e6a-b616-7fab2f2cf7ac"; // Builderings SS 1 true
const supplierCompany3f = "0dec6985-c2d1-4941-99c3-2eb570d567d9"; // W.SOPR 3 false
const supplierCompany3t = "4e149874-f680-4726-b0e7-af03a849b407";
const supplierCompany3tShortName = "RI" // 3 true

const companyFullDataQuery = ` \
uuid \
orgname \
shortname \
inn \
phone \
email \
description \
address \
siteUrl \
timeZone \
ownerUser { \
  uuid \
  imageFile { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
} \
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
companyCertificates { \
  companyUuid
  file { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
  description \
} \
companySpecs { \
  specId \
  langId \
  spec \
} \
typeAccess { \
  typeAccessId \
  langId \
  name \
} \
isSupplier \
isEmailVerified \
subscribers \
isFollowed \
createdAt \
updatedAt \
`;

const companiesListQuery = ` \
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
typeAccess { \
  typeAccessId \
  langId \
  name \
} \
isSupplier \
isFollowed \
updatedAt \
`;

const companyCertificatesQuery = ` \
companyCertificates { \
  companyUuid
  file { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
  description \
} \
`;

const orgnameUpdate = "orgname test for update";
const shortnameUpdate = "shortname test for update";
const innUpdate = "inn test for update";
const phoneUpdate = "phone test for update";
const emailUpdate = "email_test@mail.test";
const descriptionUpdate = "description test for update";
const addressUpdate = "address test for update";
const siteUrlUpdate = "site-url.test.update";
const timeZoneUpdate = "UTC";
const regionUpdateId = 5;
const companyTypeUpdateId = 2;

// data for represent
const regionIdRepresentation = 8;
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
const companyRepresentsListQuery = ` \
uuid \
companyUuid \
region { \
  regionId \
  langId \
  region \
} \
representationType { \
  representationTypeId \
  langId \
  representationType \
} \
name \
address \
phone \
`;
const companyRepresentQuery = ` \
uuid \
companyUuid \
name \
address \
phone \
`;

const companyRolesQuery = ` \
role {  \
  roleMemberId  \
  langId  \
  name  \
} \
permissions {  \
  typeAccessId  \
  langId  \
  name  \
} \
`;

const companyMembersQuery = ` \
companyUuid \
user { \
  uuid
  username
}  \
companyRole {  \
  role {  \
    roleMemberId  \
    langId  \
    name  \
  } \
  permissions {  \
    typeAccessId  \
    langId  \
    name  \
  } \
} \
isEnabled \
createdAt \
updatedAt \
`;

const companyMembersContainKeys = ["companyUuid", "createdAt", "isEnabled", "companyRole", "updatedAt", "user"];

// data for component
const parentComponentUuid = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const nameComponent = "M Series Geared Motor";
const nameComponent2 = "X Custom Geared Motor";
const descriptionComponent = "graphqlcomment for component";
const typeAccessIdComponent = 3;
const typeAccessIdComponentPrivate = 1;
const typeAccessId3 = 3;
const typeAccessId2 = 2;
const typeAccessId1 = 1;
const componentTypeId = 2;
const actualStatusIdComponent = 1;
const isBaseComponent = true;
const isBaseComponent0 = false;
const subscribersCount = 1;
const keywordIdsOk = [1,3,5];
const keywordIdsDup = [1,2,3,4,5];
const licenseIdOk = 1;
const licenseIdErr = 2;
const filename1 = "file-test-name 1.pdf";
const filename2 = "file-test-name 2.pdf";
const filename3 = "file-test-name 3.pdf";
const filename4 = "file-test-name 4.pdf";
const filename5 = "file-test-name 5.pdf";

var componentUuidStandard = "";
var componentUuidNoStandard = "";

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname in (?,?)', [
    orgname,
    orgname2,
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

// Sets a mark in the database that the file has been uploaded and verified
async function setFileAsUploadedDb(fileUuid) {
  return global.knex.raw('UPDATE file_ref SET is_checked=true, is_hidden=false WHERE uuid=?', [
    fileUuid,
  ]);
}

describe('company', () => {
  beforeAll(() => {
    cleanupCompanyRepresentDb();
    cleanupCompanyDb();
    // cleanupTokenDb();
    cleanupUserDb();
    return;
  });
  afterAll(() => {
    cleanupCompanyRepresentDb();
    cleanupCompanyDb();
    // cleanupTokenDb();
    cleanupUserDb();
    return;
  });

  const agent = request.agent(url);

  it('/graphql:Q company - Bad public supplier company (not public)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query GetSupplierCompany {
          supplierCompany (companyUuid: "${supplierCompany1t}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql supplierCompany=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'Internal Server Error'
    );
    expect(body.errors[0].path[0]).toBe('supplierCompany');
    done();
  });

  it('/graphql:Q company - Bad public supplier company (not supplier)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query GetSupplierCompany {
          supplierCompany (companyUuid: "${supplierCompany3f}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql supplierCompany=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'Internal Server Error'
    );
    expect(body.errors[0].path[0]).toBe('supplierCompany');
    done();
  });

  it('/graphql:Q company - OK public supplier company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query GetSupplierCompany {
          supplierCompany (companyUuid: "${supplierCompany3t}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql supplierCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { supplierCompany },
    } = body;
    expect(supplierCompany.uuid).toBe(supplierCompany3t);
    expect(supplierCompany.shortname).toBe(supplierCompany3tShortName);
    expect(supplierCompany.typeAccess.name).toBe("Public");
    expect(supplierCompany.isSupplier).toBe(true);
    done();
  });

  it('/graphql:Q company - OK public supplier company (set lang)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set('Accept-Language', 'ru')
      .send({
        query: `query GetSupplierCompany {
          supplierCompany (companyUuid: "${supplierCompany3t}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql supplierCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { supplierCompany },
    } = body;
    expect(supplierCompany.uuid).toBe(supplierCompany3t);
    expect(supplierCompany.shortname).toBe(supplierCompany3tShortName);
    expect(supplierCompany.typeAccess.name).toBe("Публичный");
    expect(supplierCompany.isSupplier).toBe(true);
    done();
  });

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

  it('/graphql:Q company - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query company {
          company (companyUuid: "${companyUuidSupplier}") {
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    // expect(response1.body).toBe(0);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(response1.body.errors[0].path[0]).toBe('company');
    done();
  });

  it('/graphql:Q company - OK Select no supplier company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query company {
        	company (companyUuid: "${companyUuidNoSupplier}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql company=%o', body);
    const {
      data: { company },
    } = body;
    expect(company.uuid).toBe(companyUuidNoSupplier);
    expect(company.orgname).toBe(orgname2);
    expect(company.isSupplier).toBe(false);
    done();
  });

  it('/graphql:Q company - OK Select supplier company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query company {
        	company (companyUuid: "${companyUuidSupplier}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql company=%o', body);
    const {
      data: { company },
    } = body;
    expect(company.uuid).toBe(companyUuidSupplier);
    expect(company.orgname).toBe(orgname);
    expect(company.isSupplier).toBe(true);
    done();
  });

  it('/graphql:Q companies - UNAUTHORIZED List all', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query {
        	companies (args: {companiesUuids: [
            "${companyUuidSupplier}",
            "${companyUuidNoSupplier}"
          ]}) {
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(response1.body.errors[0].path[0]).toBe('companies');
    done();
  });

  it('/graphql:Q companies - OK List with uuids', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
        	companies (args: {companiesUuids: [
            "${companyUuidSupplier}",
            "${companyUuidNoSupplier}"
          ]}) {
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeNonEmptyArray();
    expect(companies[0].uuid).toBe(companyUuidSupplier);
    expect(companies[1].uuid).toBe(companyUuidNoSupplier);
    done();
  });

  // Test companies list
  it('/graphql:Q companies - OK List all public', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
        	companies {
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    // var index = companies.length - 1;
    expect(companies).toBeNonEmptyArray();
    expect(companies[0].uuid).toBe(companyUuidBase);
    // expect(companies.length).toBe(1);
    done();
  });

  it('/graphql:M CompanyFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            addCompanyFav(companyUuid: "${companyUuidBase}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyFav body=%o', body);
    expect(body.data.addCompanyFav).toBe(true);
    done();
  });

  it('/graphql:Q companies - OK List favorite', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
        	companies (args: {
            favorite: true
          }){
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeNonEmptyArray();
    expect(companies[0].uuid).toBe(companyUuidBase);
    expect(companies.length).toBe(1);
    done();
  });

  it('/graphql:M CompanyFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            deleteCompanyFav(companyUuid: "${companyUuidBase}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyFav body=%o', body);
    expect(body.data.deleteCompanyFav).toBe(true);
    done();
  });

  it('/graphql:Q companies - OK List no favorite', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          companies (args: {
            favorite: true
          }){
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeEmptyArray();
    done();
  });

  it('/graphql:Q companies - OK List by user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
          companies (args: {
            userUuid: "${authorizationUserFirst}"
          }){
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeNonEmptyArray();
    expect(companies.length).toBe(2);
    done();
  });

  it('/graphql:Q companies - OK List by other user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          companies (args: {
            userUuid: "${authorizationUserFirst}"
          }){
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeEmptyArray();
    done();
  });

  it('/graphql:Q companies - OK List by other user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
          companies (args: {
            userUuid: "${userUuidBase}"
          }){
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeNonEmptyArray();
    expect(companies[0].uuid).toBe(companyUuidBase);
    expect(companies.length).toBe(1);
    done();
  });

  // Testing update company favicon
  it('/graphql:M uploadCompanyFavicon - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            uploadCompanyFavicon(
              companyUuid: "${companyUuidNoSupplier}"
              filename: "test.test"
            ){
              fileUuid
              filename
              uploadUrl
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('uploadCompanyFavicon');
    done();
  });

  it('/graphql:M uploadCompanyFavicon - Ok favicon updated', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            uploadCompanyFavicon(
              companyUuid: "${companyUuidNoSupplier}"
              filename: "${goodFilenameCertificateTest}"
            ){
              fileUuid
              filename
              uploadUrl
            }
          }`,
        })
        .expect(HttpStatus.OK)
      debug('/graphql uploadCompanyFavicon=%o', body);
      const {
        data: { uploadCompanyFavicon },
      } = body;
      expect(uploadCompanyFavicon.fileUuid).toBeNonEmptyString();
      await setFileAsUploadedDb(uploadCompanyFavicon.fileUuid);
      expect(uploadCompanyFavicon.filename).toBe(goodFilenameCertificateTest);
      expect(uploadCompanyFavicon.uploadUrl).toBeNonEmptyString();
      done();
  });

  // Testing company data  update
  it('/graphql:M putCompanyUpdate - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putCompanyUpdate(
              companyUuid: "${companyUuidNoSupplier}"
              args: {
                orgname: "${orgnameUpdate}"
                shortname: "${shortnameUpdate}"
                inn: "${innUpdate}"
                phone: "${phoneUpdate}"
                email: "${emailUpdate}"
                description: "${descriptionUpdate}"
                address: "${addressUpdate}"
                siteUrl: "${siteUrlUpdate}"
                timeZone: "${timeZoneUpdate}"
                regionId: ${regionUpdateId}
                companyTypeId: ${companyTypeUpdateId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('putCompanyUpdate');
    done();
  });

  it('/graphql:M putCompanyUpdate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            putCompanyUpdate(
              companyUuid: "${companyUuidNoSupplier}"
              args: {
                orgname: "${orgnameUpdate}"
                shortname: "${shortnameUpdate}"
                inn: "${innUpdate}"
                phone: "${phoneUpdate}"
                email: "${emailUpdate}"
                description: "${descriptionUpdate}"
                address: "${addressUpdate}"
                siteUrl: "${siteUrlUpdate}"
                timeZone: "${timeZoneUpdate}"
                regionId: ${regionUpdateId}
                companyTypeId: ${companyTypeUpdateId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('putCompanyUpdate');
    done();
  });

  it('/graphql:M putCompanyUpdate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putCompanyUpdate(
              companyUuid: "${companyUuidNoSupplier}"
              args: {
                orgname: "${orgnameUpdate}"
                shortname: "${shortnameUpdate}"
                inn: "${innUpdate}"
                phone: "${phoneUpdate}"
                email: "${emailUpdate}"
                description: "${descriptionUpdate}"
                address: "${addressUpdate}"
                siteUrl: "${siteUrlUpdate}"
                timeZone: "${timeZoneUpdate}"
                regionId: ${regionUpdateId}
                companyTypeId: ${companyTypeUpdateId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putCompanyUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putCompanyUpdate },
    } = body;
    expect(putCompanyUpdate).toBe(9);
    done();
  });

  it('/graphql:M putCompanyUpdate - BadRequest data has already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            putCompanyUpdate(
              companyUuid: "${companyUuidNoSupplier}"
              args: {
                orgname: "${orgnameUpdate}"
                shortname: "${shortnameUpdate}"
                inn: "${innUpdate}"
                phone: "${phoneUpdate}"
                email: "${emailUpdate}"
                description: "${descriptionUpdate}"
                address: "${addressUpdate}"
                siteUrl: "${siteUrlUpdate}"
                timeZone: "${timeZoneUpdate}"
                regionId: ${regionUpdateId}
                companyTypeId: ${companyTypeUpdateId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The data has already'
    );
    expect(body.errors[0].path[0]).toBe('putCompanyUpdate');
    done();
  });

  it('/graphql:Q company - OK check update data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query company {
        	company (companyUuid: "${companyUuidNoSupplier}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql company=%o', body);
    const {
      data: { company },
    } = body;
    expect(company.orgname).toBe(orgnameUpdate);
    expect(company.shortname).toBe(shortnameUpdate);
    expect(company.inn).toBe(innUpdate);
    expect(company.phone).toBe(phoneUpdate);
    expect(company.email).toBe(emailUpdate);
    expect(company.description).toBe(descriptionUpdate);
    expect(company.address).toBe(addressUpdate);
    expect(company.siteUrl).toBe(siteUrlUpdate);
    expect(company.timeZone).toBe(timeZoneUpdate);
    expect(company.region.regionId).toBe(regionUpdateId);
    expect(company.companyType.companyTypeId).toBe(companyTypeUpdateId);
    done();
  });

  // Testing change company access
  it('/graphql:M changeCompanyAccess - BadRequest fake uuid company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            changeCompanyAccess(args: {
              companyUuid: "${uuidFake}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeCompanyAccess=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeCompanyAccess');
    done();
  });

  it('/graphql:M changeCompanyAccess - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
            changeCompanyAccess(args: {
              companyUuid: "${companyUuidSupplier}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeCompanyAccess=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeCompanyAccess');
    done();
  });

  it('/graphql:M changeCompanyAccess - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            changeCompanyAccess(args: {
              companyUuid: "${companyUuidSupplier}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeCompanyAccess=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeCompanyAccess },
    } = body;
    expect(changeCompanyAccess).toBe(true);
    done();
  });

  it('/graphql:Q Get full data Company - OK check change access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
          query: `query companyQuery{
            company(companyUuid: "${companyUuidSupplier}") {
              uuid
              ownerUser {
                uuid
              }
              typeAccess {
                typeAccessId
                langId
                name
              }
            }
          }`,
        })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { company },
    } = body;
    expect(company.uuid).toBe(companyUuidSupplier);
    expect(company.ownerUser.uuid).toBe(authorizationUserFirst);
    expect(company.typeAccess.typeAccessId).toBe(typeAccessId2);
    done();
  });

  it('/graphql:M changeCompanyAccess - OK access already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            changeCompanyAccess(args: {
              companyUuid: "${companyUuidSupplier}"
              newTypeAccessId: ${typeAccessId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeCompanyAccess=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeCompanyAccess },
    } = body;
    expect(changeCompanyAccess).toBe(false);
    done();
  });

  it('/graphql:M changeCompanyAccess - OK return private access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
            changeCompanyAccess(args: {
              companyUuid: "${companyUuidSupplier}"
              newTypeAccessId: ${typeAccessId1}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeCompanyAccess=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeCompanyAccess },
    } = body;
    expect(changeCompanyAccess).toBe(true);
    done();
  });

  // Test for company certificates
  it('/graphql:Q companyCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            company(companyUuid: "${companyUuidNoSupplier}") {
              ${companyCertificatesQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql CompanyCertificate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('company');
    done();
  });

  it('/graphql:M CompanyCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          uploadCompanyCertificate(certData: {
            companyUuid: "${companyUuidNoSupplier}"
            description: "${descriptionCertificateTest}"
        		filename: "${badFilenameCertificateTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql CompanyCertificate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('uploadCompanyCertificate');
    done();
  });

  it('/graphql:M CompanyCertificate - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadCompanyCertificate(certData: {
            companyUuid: "${companyUuidNoSupplier}"
            description: "${descriptionCertificateTest}"
        		filename: "${badFilenameCertificateTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql CompanyCertificate=%o', body);
    const {
      data: { uploadCompanyCertificate },
    } = body;
    fileCertificateTestUuid = uploadCompanyCertificate.fileUuid;
    await setFileAsUploadedDb(fileCertificateTestUuid);
    expect(uploadCompanyCertificate.fileUuid).toBeNonEmptyString();
    expect(uploadCompanyCertificate.filename).toBe(goodFilenameCertificateTest);
    expect(uploadCompanyCertificate.uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:M CompanyCertificate - BadRequest description too long (upload)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          uploadCompanyCertificate(certData: {
            companyUuid: "${companyUuidNoSupplier}"
            description: "${tooLongCertificateDescription}"
            filename: "${badFilenameCertificateTest}"
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
    expect(errors[0].message).toBe("BadRequest: Text must be less than 500 characters");
    done();
  });

  it('/graphql:M CompanyCertificate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          uploadCompanyCertificate(certData: {
            companyUuid: "${companyUuidNoSupplier}"
            description: "${descriptionCertificateTest}"
        		filename: "${badFilenameCertificateTest}"
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

  // Test update company certificate description
  it('/graphql:M updateCompanyCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          updateCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('updateCompanyCertificate');
    done();
  });

  it('/graphql:M updateCompanyCertificate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          updateCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
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

  it('/graphql:M updateCompanyCertificate - BadRequest description too long', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          updateCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
            description: "${tooLongCertificateDescription}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Text must be less than 500 characters");
    done();
  });

  it('/graphql:M updateCompanyCertificate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          updateCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
            description: "test of the test description"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { updateCompanyCertificate },
    } = body;
    expect(updateCompanyCertificate).toBe(true);
    done();
  });

  it('/graphql:M updateCompanyCertificate - OK return old description', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          updateCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { updateCompanyCertificate },
    } = body;
    expect(updateCompanyCertificate).toBe(true);
    done();
  });

  it('/graphql:M updateCompanyCertificate - OK duplicate data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          updateCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { updateCompanyCertificate },
    } = body;
    expect(updateCompanyCertificate).toBe(false);
    done();
  });

  it('/graphql:Q companyCertificate - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            company(companyUuid: "${companyUuidNoSupplier}") {
              ${companyCertificatesQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql CompanyCertificate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { company },
    } = body;
    expect(company.companyCertificates[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(company.companyCertificates[0].file.filename).toBe(goodFilenameCertificateTest);
    expect(company.companyCertificates[0].file.downloadUrl).toBeNonEmptyString();
    expect(company.companyCertificates[0].description).toBe(descriptionCertificateTest);
    done();
  });

  it('/graphql:Q companyCertificate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            company(companyUuid: "${companyUuidNoSupplier}") {
              ${companyCertificatesQuery}
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

  // Testing adding company specs
  it('/graphql:M addCompanySpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          addCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('addCompanySpecs');
    done();
  });

  it('/graphql:M addCompanySpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { addCompanySpecs },
    } = body;
    expect(addCompanySpecs).toBe(3);
    done();
  });

  it('/graphql:M addCompanySpecs - OK with duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsDup}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { addCompanySpecs },
    } = body;
    expect(addCompanySpecs).toBe(2);
    done();
  });

  it('/graphql:M addCompanySpecs - BadRequest all duplicates', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsOk}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: This ids [10, 30, 55] already has"
    );
    expect(body.errors[0].path[0]).toBe('addCompanySpecs');
    done();
  });

  it('/graphql:M addCompanySpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          addCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('addCompanySpecs');
    done();
  });

  it('/graphql:M addCompanySpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          addCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('addCompanySpecs');
    done();
  });

  it('/graphql:Q company - OK check update data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query company {
          company (companyUuid: "${companyUuidNoSupplier}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql company=%o', body);
    // expect(body).toBe(0);
    const {
      data: { company },
    } = body;
    expect(company.companySpecs[0].specId).toBe(10);
    expect(company.companySpecs[0].spec).toBeNonEmptyString();
    expect(company.companySpecs[1].specId).toBe(22);
    expect(company.companySpecs[1].spec).toBeNonEmptyString();
    expect(company.companySpecs[2].specId).toBe(30);
    expect(company.companySpecs[2].spec).toBeNonEmptyString();
    expect(company.companySpecs[3].specId).toBe(44);
    expect(company.companySpecs[3].spec).toBeNonEmptyString();
    expect(company.companySpecs[4].specId).toBe(55);
    expect(company.companySpecs[4].spec).toBeNonEmptyString();
    done();
  });

  // Testing get specs for company
  it('/graphql:Q companySpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
          companySpecs(
            companyUuid: "${companyUuidNoSupplier}"
          ){
            specId
            langId
            spec
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('companySpecs');
    done();
  });

  it('/graphql:Q companySpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          companySpecs(
            companyUuid: "${companyUuidNoSupplier}"
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
      data: { companySpecs },
    } = body;
    expect(companySpecs.length).toBe(5);
    expect(companySpecs[0].specId).toBe(10);
    expect(companySpecs[0].spec).toBeNonEmptyString();
    expect(companySpecs[1].specId).toBe(22);
    expect(companySpecs[1].spec).toBeNonEmptyString();
    expect(companySpecs[2].specId).toBe(30);
    expect(companySpecs[2].spec).toBeNonEmptyString();
    expect(companySpecs[3].specId).toBe(44);
    expect(companySpecs[3].spec).toBeNonEmptyString();
    expect(companySpecs[4].specId).toBe(55);
    expect(companySpecs[4].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q companySpecs - OK with limit and offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          companySpecs(
            companyUuid: "${companyUuidNoSupplier}"
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
      data: { companySpecs },
    } = body;
    expect(companySpecs.length).toBe(1);
    expect(companySpecs[0].specId).toBe(44);
    expect(companySpecs[0].spec).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q companySpecs - OK not found specs', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
          companySpecs(
            companyUuid: "${companyUuidNoSupplier}"
            paginate: {
              currentPage: 50
              perPage: 500
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
      data: { companySpecs },
    } = body;
    expect(companySpecs).toBeEmptyArray();
    done();
  });

  // Testing delete company specs
  it('/graphql:M deleteCompanySpecs - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanySpecs');
    done();
  });

  it('/graphql:M deleteCompanySpecs - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { deleteCompanySpecs },
    } = body;
    expect(deleteCompanySpecs).toBe(2);
    done();
  });

  it('/graphql:M deleteCompanySpecs - OK data already delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${specIdsDel}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { deleteCompanySpecs },
    } = body;
    expect(deleteCompanySpecs).toBe(0);
    done();
  });

  it('/graphql:M deleteCompanySpecs - BadRequest not found id', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Not found specs"
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanySpecs');
    done();
  });

  it('/graphql:M deleteCompanySpecs - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteCompanySpecs(args: {
            companyUuid: "${companyUuidNoSupplier}"
            specIds: [${idErr}]
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanySpecs=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      "BadRequest: Access denied"
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanySpecs');
    done();
  });

  it('/graphql:Q company - OK check update data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query company {
          company (companyUuid: "${companyUuidNoSupplier}"){
            ${companyFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql company=%o', body);
    // expect(body).toBe(0);
    const {
      data: { company },
    } = body;
    expect(company.companySpecs.length).toBe(3);
    expect(company.companySpecs[0].specId).toBe(22);
    expect(company.companySpecs[0].spec).toBeNonEmptyString();
    expect(company.companySpecs[1].specId).toBe(30);
    expect(company.companySpecs[1].spec).toBeNonEmptyString();
    expect(company.companySpecs[2].specId).toBe(44);
    expect(company.companySpecs[2].spec).toBeNonEmptyString();
    done();
  });

  // Test for represent
  it('/graphql:M registerCompanyRepresent - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerCompanyRepresent(args: {
                companyUuid: "${companyUuidNoSupplier}",
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                regionId: ${regionIdRepresentation},
                representationTypeId: ${representationTypeId}
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

  it('/graphql:M registerCompanyRepresent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation companyRepresentQuery {
          registerCompanyRepresent(args: {
            companyUuid: "${companyUuidSupplier}",
            name: "${nameRepresentationFirst}",
            address: "${addressRepresentation}",
            phone: "${phoneRepresentation}",
            regionId: ${regionIdRepresentation},
            representationTypeId: ${representationTypeId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompanyRepresent body=%o', body);
    const {
      data: { registerCompanyRepresent },
    } = body;
    expect(registerCompanyRepresent.length).toBe(36);
    uuidRepresentFirst = registerCompanyRepresent;
    done();
  });

  it('/graphql:Q List companyRepresents - OK company uuid', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyRepresents(args: {
              companyUuid: "${companyUuidSupplier}"
            }){
                ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql all body=%o', body);
    // expect(body).toBe(0);
    // for test delete represent not owned user
    expect(body.data.companyRepresents).toBeNonEmptyArray();
    expect(body.data.companyRepresents[0].uuid).toBe(uuidRepresentFirst);
    expect(body.data.companyRepresents[0].companyUuid).toBe(companyUuidSupplier);
    done();
  });

  it('/graphql:M registerCompanyRepresent - OK Not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerCompanyRepresent(args: {
                companyUuid: "${companyUuidNoSupplier}",
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                regionId: ${regionIdRepresentation},
                representationTypeId: ${representationTypeId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    expect(body.data.registerCompanyRepresent.length).toBe(36);
    done();
  });

  // Test for represent
  it('/graphql:M updateCompanyRepresent - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            updateCompanyRepresent(
              companyUuid: "${companyUuidNoSupplier}",
              companyRepresentUuid: "${uuidRepresentFirst}",
              args: {
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                regionId: ${regionIdRepresentation},
                representationTypeId: ${representationTypeId}
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

  it('/graphql:M updateCompanyRepresent - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            updateCompanyRepresent(
              companyUuid: "${companyUuidNoSupplier}",
              companyRepresentUuid: "${uuidRepresentFirst}",
              args: {
                name: "random data",
                address: "random data",
                phone: "+7777777777777777",
                regionId: 5,
                representationTypeId: 2
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { updateCompanyRepresent },
    } = body;
    expect(updateCompanyRepresent).toBe(5);
    done();
  });

  it('/graphql:M updateCompanyRepresent - Return data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            updateCompanyRepresent(
              companyUuid: "${companyUuidNoSupplier}",
              companyRepresentUuid: "${uuidRepresentFirst}",
              args: {
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                regionId: ${regionIdRepresentation},
                representationTypeId: ${representationTypeId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { updateCompanyRepresent },
    } = body;
    expect(updateCompanyRepresent).toBe(5);
    done();
  });

  it('/graphql:M updateCompanyRepresent - BadRequest data duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            updateCompanyRepresent(
              companyUuid: "${companyUuidNoSupplier}",
              companyRepresentUuid: "${uuidRepresentFirst}",
              args: {
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                regionId: ${regionIdRepresentation},
                representationTypeId: ${representationTypeId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: The data has already");
    done();
  });

  it('/graphql:Q List companyRepresents - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query {
            companyRepresents(args: {
              representsUuids: ["${uuidRepresentFirst}"]
            }){
                ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(response1.body.errors[0].path[0]).toBe('companyRepresents');
    done();
  });

  it('/graphql:Q List companyRepresents - OK company uuid', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyRepresents(args: {
              companyUuid: "${companyUuidSupplier}"
            }){
                ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql all body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeNonEmptyArray();
    expect(response1.body.data.companyRepresents[0].companyUuid).toBe(companyUuidSupplier);
    done();
  });

  it('/graphql:Q List companyRepresents - OK by represent uuid', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyRepresents(args: {
              representsUuids: ["${uuidRepresentFirst}"]
            }){
                ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql all body=%o', response1.body);
    // expect(response1.body).toBe(0);
    expect(response1.body.data.companyRepresents).toBeNonEmptyArray();
    expect(response1.body.data.companyRepresents[0].companyUuid).toBe(companyUuidSupplier);
    done();
  });

  it('/graphql:M deleteCompanyRepresent - BadRequest not access', async (done) => {
    response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation deleteCompanyRepresentQuery {
          deleteCompanyRepresent(
            companyUuid: "${companyUuidSupplier}",
            companyRepresentUuid: "${uuidRepresentFirst}"
          )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(response1.body.errors[0].path[0]).toBe('deleteCompanyRepresent');
    done();
  });

  it('/graphql:M deleteCompanyRepresent - BadRequest fake uuid company', async (done) => {
    response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation deleteCompanyRepresentQuery {
          deleteCompanyRepresent(
            companyUuid: "${uuidFake}",
            companyRepresentUuid: "${uuidRepresentFirst}"
          )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(response1.body.errors[0].path[0]).toBe('deleteCompanyRepresent');
    done();
  });

  it('/graphql:M deleteCompanyRepresent - BadRequest fake uuid represent', async (done) => {
    response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation deleteCompanyRepresentQuery {
          deleteCompanyRepresent(
            companyUuid: "${companyUuidSupplier}",
            companyRepresentUuid: "${uuidFake}"
          )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Not found representative'
    );
    expect(response1.body.errors[0].path[0]).toBe('deleteCompanyRepresent');
    done();
  });

  it('/graphql:Q companyRepresents - OK with companyUuid', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyRepresents(args: {
              companyUuid: "${companyUuidSupplier}"
            }){
              ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter body=%o', response1.body);
    // expect(response1.body).toBe(0);
    expect(response1.body.data.companyRepresents).toBeNonEmptyArray();
    expect(response1.body.data.companyRepresents[0].companyUuid).toBe(companyUuidSupplier);
    expect(response1.body.data.companyRepresents.pop().companyUuid).toBe(companyUuidSupplier);
    done();
  });

  it('/graphql:Q  - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query {
            companyRepresents(args: {
              companyUuid: "${companyUuidSupplier}"
            }){
              ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompanyRole=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('companyRepresents');
    done();
  });

  it('/graphql:M deleteCompanyRepresent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation deleteCompanyRepresentQuery {
          deleteCompanyRepresent(
            companyUuid: "${companyUuidSupplier}",
            companyRepresentUuid: "${uuidRepresentFirst}"
          )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyRepresent body=%o', body);
    const {
      data: { deleteCompanyRepresent },
    } = body;
    expect(deleteCompanyRepresent).toBe(true);
    done();
  });

  it('/graphql:Q Select remove companyRepresent - EmptyArray', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            companyRepresents(args: {
              companyUuid: "${companyUuidSupplier}"
            }){
              ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeEmptyArray();
    done();
  });

  // Test delete company certificate
  it('/graphql:M deleteCompanyCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          deleteCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyCertificate');
    done();
  });

  it('/graphql:M deleteCompanyCertificate - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation {
          deleteCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
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

  it('/graphql:M deleteCompanyCertificate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { deleteCompanyCertificate },
    } = body;
    expect(deleteCompanyCertificate).toBe(true);
    done();
  });

  it('/graphql:M deleteCompanyCertificate - BadRequest not found data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation {
          deleteCompanyCertificate(args: {
            companyUuid: "${companyUuidNoSupplier}"
            fileUuid: "${fileCertificateTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'Internal Server Error'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyCertificate');
    done();
  });

  // Testing add role access
  it('/graphql:M registerCompanyRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
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
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('registerCompanyRole');
    done();
  });

  it('/graphql:M registerCompanyRole - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
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
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('registerCompanyRole');
    done();
  });

  it('/graphql:M registerCompanyRole - OK create company role', async (done) => {
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

  it('/graphql:M registerCompanyRole - OK create second role', async (done) => {
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
              name: "${nameRole2}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompanyRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerCompanyRole },
    } = body;
    newRoleId2 = registerCompanyRole;
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
    expect(registerCompanyRole).toBe(newRoleId);
    done();
  });

  // Testing change name role access
  it('/graphql:M changeNameRoleCompany - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            changeNameRoleCompany(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId2}
              langId: ${langId}
              name: "${nameRole2}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('changeNameRoleCompany');
    done();
  });

  it('/graphql:M changeNameRoleCompany - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            changeNameRoleCompany(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId2}
              langId: ${langId}
              name: "${nameRole2}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeNameRoleCompany=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeNameRoleCompany');
    done();
  });

  it('/graphql:M changeNameRoleCompany - OK change name role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeNameRoleCompany(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId2}
              langId: ${langId}
              name: "temp test nam2e"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeNameRoleCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeNameRoleCompany },
    } = body;
    expect(changeNameRoleCompany).toBe(true);
    done();
  });

  it('/graphql:M changeNameRoleCompany - OK return name role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeNameRoleCompany(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId2}
              langId: ${langId}
              name: "${nameRole2}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeNameRoleCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeNameRoleCompany },
    } = body;
    expect(changeNameRoleCompany).toBe(true);
    done();
  });

  it('/graphql:M changeNameRoleCompany - OK name role duplicate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeNameRoleCompany(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId2}
              langId: ${langId}
              name: "${nameRole2}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeNameRoleCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeNameRoleCompany },
    } = body;
    expect(changeNameRoleCompany).toBe(false);
    done();
  });

  // Testing get role access
  it('/graphql:Q companyRoles - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
            companyRoles(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyRolesQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('companyRoles');
    done();
  });

  it('/graphql:Q companyRoles - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
            companyRoles(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyRolesQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companyRoles=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('companyRoles');
    done();
  });

  it('/graphql:Q companyRoles - OK show company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
            companyRoles(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyRolesQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companyRoles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companyRoles },
    } = body;
    expect(companyRoles[0].role.roleMemberId).toBe(newRoleId);
    expect(companyRoles[0].role.langId).toBe(langId);
    expect(companyRoles[0].role.name).toBe(nameRole);
    expect(companyRoles[0].permissions).toBeEmptyArray();
    done();
  });

  // Test for add access for company role
  it('/graphql:M addAccessRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addAccessRole(args: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('addAccessRole');
    done();
  });

  it('/graphql:M addAccessRole - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            addAccessRole(args: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('addAccessRole');
    done();
  });

  it('/graphql:M addAccessRole - OK add access for role', async (done) => {
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
              typesAccessIds: [1, 3, 2]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { addAccessRole },
    } = body;
    expect(addAccessRole).toBe(3);
    done();
  });

  it('/graphql:Q companyRoles - OK show company roles', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
            companyRoles(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyRolesQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companyRoles=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companyRoles },
    } = body;
    expect(companyRoles[0].role.roleMemberId).toBe(newRoleId);
    expect(companyRoles[0].role.langId).toBe(langId);
    expect(companyRoles[0].role.name).toBe(nameRole);
    // expect(companyRoles[0].permissions).toBeEmptyArray();
    expect(companyRoles[0].permissions[0].typeAccessId).toBe(1);
    expect(companyRoles[0].permissions[0].name).toBe("Manage");
    expect(companyRoles[0].permissions[1].typeAccessId).toBe(2);
    expect(companyRoles[0].permissions[1].name).toBe("Write");
    expect(companyRoles[0].permissions[2].typeAccessId).toBe(3);
    expect(companyRoles[0].permissions[2].name).toBe("Read");
    done();
  });

  it('/graphql:M addAccessRole - BadRequest duplicate data', async (done) => {
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
              typesAccessIds: [1, 3]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Found duplicate data'
    );
    expect(body.errors[0].path[0]).toBe('addAccessRole');
    done();
  });

  // Register components for testing add company suppliers
  it('/graphql:M registerComponent - OK standard', async (done) => {
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

  // Testing add company owner supplier
  it('/graphql:M setCompanyOwnerSupplier - OK', async (done) => {
    // change owner company
    await global.knex.raw('UPDATE company_ref SET user_uuid=? WHERE uuid=?', [
      authorizationUserSecond,
      companyUuidNoSupplier,
    ]);
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            setCompanyOwnerSupplier(args: {
                componentUuid: "${componentUuidNoStandard}",
                companyUuid: "${companyUuidNoSupplier}",
                description: "description for supplier component",
            })
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql setCompanyOwnerSupplier=%o', body);
    // expect(body).toBe(0);
    const {
      data: { setCompanyOwnerSupplier },
    } = body;
    expect(setCompanyOwnerSupplier).toBe(true);
    done();
    // return owner component
    await global.knex.raw('UPDATE company_ref SET user_uuid=? WHERE uuid=?', [
      authorizationUserFirst,
      companyUuidNoSupplier,
    ]);
  });

  it('/graphql:M setCompanyOwnerSupplier - BadRequest standard component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyOwnerSupplier(args: {
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
      'BadRequest: This not work for base component'
    );
    expect(body.errors[0].path[0]).toBe('setCompanyOwnerSupplier');
    done();
  });

  it('/graphql:M setCompanyOwnerSupplier - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            setCompanyOwnerSupplier(args: {
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
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('setCompanyOwnerSupplier');
    done();
  });

  // Test for delete company of component suppliers list
  it('/graphql:M deleteSupplierCompany - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
          deleteSupplierCompany(args: {
              companyUuid: "${companyUuidNoSupplier}",
              componentUuid: "${componentUuidNoStandard}",
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteSupplierCompany');
    done();
  });

  it('/graphql:M deleteSupplierCompany - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
          deleteSupplierCompany(args: {
              companyUuid: "${companyUuidNoSupplier}",
              componentUuid: "${componentUuidNoStandard}",
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteSupplierCompany=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('deleteSupplierCompany');
    done();
  });

  it('/graphql:M deleteSupplierCompany - OK delete company role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteSupplierCompany(args: {
              companyUuid: "${companyUuidNoSupplier}",
              componentUuid: "${componentUuidNoStandard}",
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteSupplierCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteSupplierCompany },
    } = body;
    expect(deleteSupplierCompany).toBe(true);
    done();
  });

  it('/graphql:M deleteSupplierCompany - Ok delete fantom', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
          deleteSupplierCompany(args: {
              companyUuid: "${companyUuidNoSupplier}",
              componentUuid: "${componentUuidNoStandard}",
          })
        }`,
      })
      .expect(HttpStatus.OK)
      // expect(body).toBe(0);
      const {
        data: { deleteSupplierCompany },
      } = body;
      expect(deleteSupplierCompany).toBe(false);
      done();
  });

  // Test for delete access for company role
  it('/graphql:M deleteAccessRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteAccessRole(args: {
              roleId: ${newRoleId}
              typesAccessIds: [4, 5]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteAccessRole');
    done();
  });

  it('/graphql:M deleteAccessRole - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteAccessRole(args: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3, 4, 5, 6]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteAccessRole=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('deleteAccessRole');
    done();
  });

  it('/graphql:M deleteAccessRole - OK delete company role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteAccessRole(args: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3, 4, 5, 6]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteAccessRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteAccessRole },
    } = body;
    expect(deleteAccessRole).toBe(2);
    done();
  });

  it('/graphql:M deleteAccessRole - Ok delete fantom', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteAccessRole(args: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3, 4, 5, 6]
            })
        }`,
      })
      .expect(HttpStatus.OK)
      // expect(body).toBe(0);
      const {
        data: { deleteAccessRole },
      } = body;
      expect(deleteAccessRole).toBe(0);
      done();
  });

  // Test for add memeber to company
  it('/graphql:M addCompanyMember - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
              roleId: ${firstAccess}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('addCompanyMember');
    done();
  });

  it('/graphql:M addCompanyMember - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            addCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
              roleId: ${firstAccess}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyMember=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('addCompanyMember');
    done();
  });

  it('/graphql:M addCompanyMember - BadRequest set role other company', async (done) => {
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
              roleId: ${firstAccess}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyMember=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Role not found'
    );
    expect(body.errors[0].path[0]).toBe('addCompanyMember');
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

  it('/graphql:M addCompanyMember - OK already has', async (done) => {
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
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The user has already member in the company'
    );
    done();
  });

  // Test for change role memeber in company
  it('/graphql:M changeRoleMember - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            changeRoleMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${firstAccess}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('changeRoleMember');
    done();
  });

  it('/graphql:M changeRoleMember - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            changeRoleMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${firstAccess}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeRoleMember=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('changeRoleMember');
    done();
  });

  it('/graphql:M changeRoleMember - BadRequest set role other company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeRoleMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${firstAccess}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeRoleMember=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Role not found'
    );
    expect(body.errors[0].path[0]).toBe('changeRoleMember');
    done();
  });

  it('/graphql:M changeRoleMember - OK change role member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeRoleMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${newRoleId2}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeRoleMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeRoleMember },
    } = body;
    expect(changeRoleMember).toBe(true);
    done();
  });

  it('/graphql:M changeRoleMember - OK return role member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeRoleMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeRoleMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeRoleMember },
    } = body;
    expect(changeRoleMember).toBe(true);
    done();
  });

  it('/graphql:M changeRoleMember - OK old role member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            changeRoleMember(args: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql changeRoleMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { changeRoleMember },
    } = body;
    expect(changeRoleMember).toBe(false);
    done();
  });

  // Testing company members list
  it('/graphql:Q companyMembers - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query  {
            companyMembers(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyMembersQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('companyMembers');
    done();
  });

  it('/graphql:Q companyMembers - OK user member company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
            companyMembers(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyMembersQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companyMembers=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companyMembers },
    } = body;
    expect(companyMembers[0]).toContainAllKeys(companyMembersContainKeys);
    // "langId", "name", "typeAccessId", "role", "langId", "name", "roleMemberId",
    expect(companyMembers[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(companyMembers[0].user.uuid).toBe(authorizationUserSecond);
    expect(companyMembers[0].isEnabled).toBe(true);
    expect(companyMembers[0].companyRole.role.roleMemberId).toBe(newRoleId);
    expect(companyMembers[0].companyRole.permissions[0].name).toBe("Write");
    done();
  });

  it('/graphql:Q companyMembers - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query  {
            companyMembers(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyMembersQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companyMembers=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companyMembers },
    } = body;
    expect(companyMembers[0]).toContainAllKeys(companyMembersContainKeys);
    expect(companyMembers[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(companyMembers[0].user.uuid).toBe(authorizationUserSecond);
    expect(companyMembers[0].isEnabled).toBe(true);
    expect(companyMembers[0].companyRole.role.roleMemberId).toBe(newRoleId);
    expect(companyMembers[0].companyRole.permissions[0].name).toBe("Write");
    done();
  });

  // delete member company
  it('/graphql:M deleteCompanyMember - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyMember');
    done();
  });

  it('/graphql:M deleteCompanyMember - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyMember=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyMember');
    done();
  });

  it('/graphql:M deleteCompanyMember - OK del company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyMember },
    } = body;
    expect(deleteCompanyMember).toBe(true);
    done();
  });

  it('/graphql:M deleteCompanyMember - OK not found company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyMember=%o', body);
    // expect(body).toBe(0);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: The user not found in the company'
    );
    done();
  });

  // after delete user of company
  it('/graphql:Q companyMembers - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query  {
            companyMembers(
              companyUuid: "${companyUuidNoSupplier}"
            ) {
              ${companyMembersQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companyMembers=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('companyMembers');
    done();
  });

  // Testing delete role access
  // Testing delete company role mutation
  describe('deleteCompanyRole', () => {
    // Test: No authentication token provided
    it('/graphql:M deleteCompanyRole - BadRequest no token', async () => {
      const { body } = await agent
        .post('/graphql')
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: 1
            })
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql deleteCompanyRole no token=%o', body);
      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Token not found');
      expect(body.errors[0].path[0]).toBe('deleteCompanyRole');
    });

    // Test: User without proper permissions tries to delete a role
    it('/graphql:M deleteCompanyRole - BadRequest access denied', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenSecond}`)
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: 1
            })
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql deleteCompanyRole access denied=%o', body);
      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Access denied');
      expect(body.errors[0].path[0]).toBe('deleteCompanyRole');
    });

    // Test: Successfully delete a company role
    it('/graphql:M deleteCompanyRole - OK delete company role', async () => {
      // Create a test role first
      const createResponse = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            registerCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              langId: 1
              name: "Test Role For Deletion ${Date.now()}"
            })
          }`,
        })
        .expect(HttpStatus.OK);

      testRoleId = createResponse.body.data.registerCompanyRole;
      expect(testRoleId).toBeGreaterThan(0);

      // Delete the created role
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${testRoleId}
            })
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql deleteCompanyRole success=%o', body);
      const { data: { deleteCompanyRole } } = body;
      expect(deleteCompanyRole).toBe(true);
    });

    // Test: Attempt to delete a role that has members assigned to it
    it('/graphql:M deleteCompanyRole - BadRequest role is in use', async () => {
      // Create a role
      const roleName = `Role With Members ${Date.now()}`;
      const createResponse = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            registerCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              langId: 1
              name: "${roleName}"
            })
          }`,
        })
        .expect(HttpStatus.OK);

      const roleId = createResponse.body.data.registerCompanyRole;
      expect(roleId).toBeGreaterThan(0);
      console.log(`Created role with ID: ${roleId}`);

      // Add the test user as a company member with this role
      const addMemberResponse = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            addCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
              roleId: ${roleId}
            })
          }`,
        })
        .expect(HttpStatus.OK);
      expect(addMemberResponse.body.data.addCompanyMember).toBe(true);
      console.log(`Added user ${authorizationUserSecond} to role ${roleId}`);

      // Try to delete the role that has members assigned
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${roleId}
            })
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql deleteCompanyRole role in use=%o', body);
      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Role is assigned to members');
      expect(body.errors[0].path[0]).toBe('deleteCompanyRole');

      // Cleanup: Remove the member from company first
      const removeMemberResponse = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            deleteCompanyMember(args: {
              companyUuid: "${companyUuidNoSupplier}"
              userUuid: "${authorizationUserSecond}"
            })
          }`,
        })
        .expect(HttpStatus.OK);

      expect(removeMemberResponse.body.data.deleteCompanyMember).toBe(true);

      // Now delete the test role (should succeed as no members assigned)
      const deleteResponse = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${roleId}
            })
          }`,
        })
        .expect(HttpStatus.OK);

      expect(deleteResponse.body.data.deleteCompanyRole).toBe(true);
      console.log(`Deleted role ${roleId}`);
    });

    // Test: Delete a non-existent role returns false
    it('/graphql:M deleteCompanyRole - returns false for non-existent role', async () => {
      const nonExistentRoleId = 99999;

      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${nonExistentRoleId}
            })
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql deleteCompanyRole non-existent=%o', body);
      const { data: { deleteCompanyRole } } = body;
      expect(deleteCompanyRole).toBe(false);
    });

    // Test: Delete with invalid company UUID
    it('/graphql:M deleteCompanyRole - BadRequest invalid company uuid', async () => {
      const invalidCompanyUuid = '00000000-0000-0000-0000-000000000000';

      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `mutation {
            deleteCompanyRole(args: {
              companyUuid: "${invalidCompanyUuid}"
              roleId: 1
            })
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql deleteCompanyRole invalid company=%o', body);
      expect(body.data).toBeNull();
      expect(body.errors[0].message).toBe('BadRequest: Access denied');
      expect(body.errors[0].path[0]).toBe('deleteCompanyRole');
    });
  });

  it('/graphql:M deleteCompanyRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyRole');
    done();
  });

  it('/graphql:M deleteCompanyRole - BadRequest access denied', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyRole=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompanyRole');
    done();
  });

  it('/graphql:M deleteCompanyRole - OK delete company role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyRole },
    } = body;
    expect(deleteCompanyRole).toBe(true);
    done();
  });

  it('/graphql:M deleteCompanyRole - OK fantom delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyRole(args: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyRole=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyRole },
    } = body;
    expect(deleteCompanyRole).toBe(false);
    done();
  });

  it('/graphql:Q companies - OK List all public suppliers', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
        	companies (args: {
            supplier: true
          }){
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { companies },
    } = body;
    // var index = companies.length - 1;
    expect(companies).toBeNonEmptyArray();
    expect(companies[0].uuid).toBe(companyUuidBase);
    // expect(companies.length).toBe(1);
    done();
  });

  // Test companies list with search functionality
  describe('companies query with search', () => {
    // Test: Search companies by orgname
    it('/graphql:Q companies - OK search by orgname', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "${orgnameUpdate.slice(0, 10)}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search by orgname=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeNonEmptyArray();
      const found = companies.some(c => c.uuid === companyUuidNoSupplier);
      expect(found).toBe(true);
    });

    // Test: Search companies by shortname
    it('/graphql:Q companies - OK search by shortname', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "${shortnameUpdate}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search by shortname=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeNonEmptyArray();
      const found = companies.some(c => c.uuid === companyUuidNoSupplier);
      expect(found).toBe(true);
    });

    // Test: Search companies by INN
    it('/graphql:Q companies - OK search by inn', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "${innUpdate}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search by inn=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeNonEmptyArray();
      const found = companies.some(c => c.uuid === companyUuidNoSupplier);
      expect(found).toBe(true);
    });

    // Test: Search with no results
    it('/graphql:Q companies - OK search no results', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "nonexistentcompany12345"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search no results=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeEmptyArray();
    });

    // Test: Search with exclude_uuids
    it('/graphql:Q companies - OK search with exclude uuids', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "${shortnameUpdate}"
              excludeUuids: ["${companyUuidNoSupplier}"]
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search with exclude=%o', body);
      const { data: { companies } } = body;
      const found = companies.some(c => c.uuid === companyUuidNoSupplier);
      expect(found).toBe(false);
    });

    // Test: Search with specific company UUIDs filter
    it('/graphql:Q companies - OK search with companies uuids filter', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              companiesUuids: ["${companyUuidNoSupplier}"]
              search: "${shortnameUpdate}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search with uuids=%o', body);
      const { data: { companies } } = body;
      expect(companies).toHaveLength(1);
      expect(companies[0].uuid).toBe(companyUuidNoSupplier);
    });

    // Test: Search with companies UUIDs that don't match
    it('/graphql:Q companies - OK search with non-matching uuids', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              companiesUuids: ["${uuidFake}"]
              search: "${shortnameUpdate}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search non-matching uuids=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeEmptyArray();
    });

    // Test: Search with supplier filter
    it('/graphql:Q companies - OK search with supplier filter', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              supplier: true
              search: "${supplierCompany3tShortName}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search with supplier=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeNonEmptyArray();
      expect(companies.every(c => c.isSupplier)).toBe(true);
      const found = companies.some(c => c.uuid === supplierCompany3t);
      expect(found).toBe(true);
    });

    // Test: Search by case-insensitive
    it('/graphql:Q companies - OK case insensitive search', async () => {
      const upperCaseSearch = shortnameUpdate.toUpperCase();
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "${upperCaseSearch}"
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies case insensitive search=%o', body);
      const { data: { companies } } = body;
      expect(companies).toBeNonEmptyArray();
      const found = companies.some(c => c.uuid === companyUuidNoSupplier);
      expect(found).toBe(true);
    });

    // Test: Search with pagination
    it('/graphql:Q companies - OK search with pagination', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(
              args: {
                search: "${shortnameUpdate}"
              }
              paginate: {
                currentPage: 1
                perPage: 1
              }
            ) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies search with pagination=%o', body);
      const { data: { companies } } = body;
      expect(companies.length).toBe(1);
      expect(companies[0].uuid).toBe(companyUuidNoSupplier);
    });

    // Test: Search with exclude multiple uuids
    it('/graphql:Q companies - OK exclude multiple uuids', async () => {
      const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${authorizationTokenFirst}`)
        .send({
          query: `query {
            companies(args: {
              search: "${shortnameUpdate}"
              excludeUuids: ["${companyUuidNoSupplier}", "${uuidFake}"]
            }) {
              ${companiesListQuery}
            }
          }`,
        })
        .expect(HttpStatus.OK);

      debug('/graphql companies exclude multiple uuids=%o', body);
      const { data: { companies } } = body;
      const found = companies.some(c => c.uuid === companyUuidNoSupplier);
      expect(found).toBe(false);
    });
  });

  // Testing delete company
  it('/graphql:M deleteCompany - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteCompany(companyUuid: "${companyUuidSupplier}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompany');
    done();
  });

  it('/graphql:M deleteCompany - BadRequest not owner user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            deleteCompany(companyUuid: "${companyUuidSupplier}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('deleteCompany');
    done();
  });

  it('/graphql:M deleteCompany - OK supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompany(companyUuid: "${companyUuidSupplier}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompany },
    } = body;
    expect(deleteCompany).toBe(companyUuidSupplier);
    done();
  });

  it('/graphql:M deleteCompany - BadRequest not found company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompany(companyUuid: "${companyUuidSupplier}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe('BadRequest: Access denied');
    expect(body.errors[0].path[0]).toBe('deleteCompany');
    done();
  });
});
