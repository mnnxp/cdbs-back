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
const userUuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const userUuid2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";

var firstAccess = 1;
var secondAccess = 2;

var langId = 1;
var nameRole = "test role";
var newRoleId = 0;

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

const descriptionCertificateTest = "test desctiption for certificate";
const badFilenameCertificateTest = "name* file/ certificate.pdf";
const goodFilenameCertificateTest = "name file certificate.pdf";

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
  } \
} \
imageFile { \
  uuid \
  filename \
  filesize \
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
companyType { \
  companyTypeId \
	langId \
  shortname \
} \
companyCertificates { \
  file { \
    uuid \
    filename \
    filesize \
  } \
  description \
} \
companySpecs { \
  companyUuid \
  spec { \
    specId \
    langId \
    spec \
  } \
} \
isSupplier \
isEmailVerified \
subscribers \
isFollowed \
isEnabled \
isDelete \
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
  filesize \
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
`;

const companyCertificatesQuery = ` \
companyCertificates { \
  file { \
    uuid \
    filename \
    filesize \
  } \
  description \
} \
companyCertificates { \
  file { \
    uuid \
    filename \
    filesize \
  } \
  description \
} \
companyCertificates { \
  file { \
    uuid \
    filename \
    filesize \
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
const regionIdRepresentation = 10;
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
access {  \
  typeAccessId  \
  langId  \
  name  \
} \
`;

const companyMembersQuery = ` \
companyUuid \
userUuid  \
role {  \
  role {  \
    roleMemberId  \
    langId  \
    name  \
  } \
  access {  \
    typeAccessId  \
    langId  \
    name  \
  } \
} \
isEnabled \
createdAt \
updatedAt \
`;

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

describe('company', () => {
  beforeAll(() => {
    cleanupCompanyRepresentDb();
    cleanupCompanyDb();
    cleanupTokenDb();
    cleanupUserDb();
    return;
  });
  afterAll(() => {
    cleanupCompanyRepresentDb();
    cleanupCompanyDb();
    cleanupTokenDb();
    cleanupUserDb();
    return;
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
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
        query: `query companies {
        	companies (companiesUuids: [
            "${companyUuidSupplier}",
            "${companyUuidNoSupplier}"
          ]) {
            ${companiesListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(response1.body.errors[0].path[0]).toBe('companies');
    done();
  });

  it('/graphql:Q companies - OK List all', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query companies {
        	companies (companiesUuids: [
            "${companyUuidSupplier}",
            "${companyUuidNoSupplier}"
          ]) {
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

  // Testing company data  update
  it('/graphql:M putCompanyUpdate - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putCompanyUpdate(
              companyUuid: "${companyUuidNoSupplier}"
              data: {
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
              data: {
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
              data: {
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
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putCompanyUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putCompanyUpdate },
    } = body;
    expect(putCompanyUpdate).toBe(10);
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
              data: {
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

  // Test for company certificates
  it('/graphql:Q CompanyCertificate - BadRequest not token', async (done) => {
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
      'BadRequest: Token not found.'
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
      'BadRequest: Token not found.'
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
    expect(uploadCompanyCertificate.fileUuid).toBeNonEmptyString();
    expect(uploadCompanyCertificate.filename).toBe(goodFilenameCertificateTest);
    expect(uploadCompanyCertificate.uploadUrl).toBeNonEmptyString();
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

  it('/graphql:Q CompanyCertificate - Ok', async (done) => {
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
    expect(company.companyCertificates[0].file.filename).toBe(goodFilenameCertificateTest);
    expect(company.companyCertificates[0].description).toBe(descriptionCertificateTest);
    done();
  });

  it('/graphql:Q CompanyCertificate - BadRequest no access', async (done) => {
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

  // Test for represent
  it('/graphql:M registerCompanyRepresent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation companyRepresentQuery {
          registerCompanyRepresent( data: {
            companyUuid: "${companyUuidSupplier}",
            name: "${nameRepresentationFirst}",
            address: "${addressRepresentation}",
            phone: "${phoneRepresentation}",
            regionId: ${regionIdRepresentation},
            representationTypeId: ${representationTypeId}
          }) {
            uuid
            companyUuid
            address
            phone
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompanyRepresent body=%o', body);
    const {
      data: { registerCompanyRepresent },
    } = body;
    expect(registerCompanyRepresent).toContainAllKeys(
      ["address", "phone", "uuid", "companyUuid"]
    );
    expect(registerCompanyRepresent.uuid).toBeNonEmptyString();
    expect(registerCompanyRepresent.companyUuid).toBe(companyUuidSupplier);
    expect(registerCompanyRepresent.address).toBe(addressRepresentation);
    expect(registerCompanyRepresent.phone).toBe(phoneRepresentation);
    // for test delete represent not owned user
    // uuidRepresentArray.push(registerCompanyRepresent.uuid);
    uuidRepresentFirst = registerCompanyRepresent.uuid;
    done();
  });

  it('/graphql:M registerCompanyRepresent - BadRequest Not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation  {
            registerCompanyRepresent( data: {
                companyUuid: "${companyUuidNoSupplier}",
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                regionId: ${regionIdRepresentation},
                representationTypeId: ${representationTypeId}
            }) {
                uuid
                companyUuid
                name
                address
                phone
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: The company is not supplier.");
    done();
  });

  it('/graphql:Q List companyRepresents - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListcompanyRepresents {
            companyRepresents (representsUuids: [
              "${uuidRepresentFirst}"
            ]){
                ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
        query: `query ListcompanyRepresents {
            companyRepresents (companyUuid: "${companyUuidSupplier}"){
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
        query: `query ListcompanyRepresents {
            companyRepresents (representsUuids: [
              "${uuidRepresentFirst}"
            ]){
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
          ){
              ${companyRepresentQuery}
          }
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
          ){
              ${companyRepresentQuery}
          }
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
          ){
              ${companyRepresentQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: The representative not you or not found.'
    );
    expect(response1.body.errors[0].path[0]).toBe('deleteCompanyRepresent');
    done();
  });

  it('/graphql:Q Select companyRepresents with companyUuid - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query ListcompanyRepresents {
            companyRepresents (companyUuid: "${companyUuidSupplier}") {
              ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeNonEmptyArray();
    expect(response1.body.data.companyRepresents[0].companyUuid).toBe(companyUuidSupplier);
    expect(response1.body.data.companyRepresents.pop().companyUuid).toBe(companyUuidSupplier);
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
          ){
              ${companyRepresentQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyRepresent body=%o', body);
    const {
      data: { deleteCompanyRepresent },
    } = body;
    expect(deleteCompanyRepresent).toContainAllKeys(
      ["address", "name", "phone", "uuid", "companyUuid"]
    );
    expect(deleteCompanyRepresent.uuid).toBe(uuidRepresentFirst);
    expect(deleteCompanyRepresent.companyUuid).toBe(companyUuidSupplier);
    expect(deleteCompanyRepresent.address).toBe(addressRepresentation);
    expect(deleteCompanyRepresent.phone).toBe(phoneRepresentation);
    // for test delete represent not owned user
    uuidRepresentDelete = deleteCompanyRepresent.uuid;
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
        query: `query ListcompanyRepresents {
            companyRepresents (companyUuid: "${companyUuidSupplier}") {
              ${companyRepresentsListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeEmptyArray();
    done();
  });

  // Testing add role access
  it('/graphql:M registerCompanyRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
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
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            registerCompanyRole( data: {
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
    expect(registerCompanyRole).toBe(newRoleId);
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
      'BadRequest: Token not found.'
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
    expect(companyRoles[0].access).toBeEmptyArray();
    done();
  });

  // Test for add access for company role
  it('/graphql:M addAccessRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            addAccessRole( data: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            addAccessRole( data: {
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
            addAccessRole( data: {
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
    expect(addAccessRole).toBe(true);
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
            addAccessRole( data: {
              roleId: ${newRoleId}
              typesAccessIds: [1, 3]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addAccessRole=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Duplicate data found'
    );
    expect(body.errors[0].path[0]).toBe('addAccessRole');
    done();
  });

  // Test for delete access for company role
  it('/graphql:M deleteAccessRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteAccessRole( data: {
              roleId: ${newRoleId}
              typesAccessIds: [4, 5]
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            deleteAccessRole( data: {
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
            deleteAccessRole( data: {
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
            deleteAccessRole( data: {
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
            addCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${firstAccess}
              }
            ) {
              companyUuid
              userUuid
              roleId
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            addCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${firstAccess}
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
            addCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
                roleId: ${firstAccess}
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
      'BadRequest: Token not found.'
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
    expect(companyMembers[0]).toContainAllKeys(
      ["companyUuid", "createdAt", "isEnabled", "role", "updatedAt", "userUuid"]
    );
    // "langId", "name", "typeAccessId", "role", "langId", "name", "roleMemberId",
    expect(companyMembers[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(companyMembers[0].userUuid).toBe(authorizationUserSecond);
    expect(companyMembers[0].isEnabled).toBe(true);
    expect(companyMembers[0].role.role.roleMemberId).toBe(newRoleId);
    expect(companyMembers[0].role.access[0].name).toBeNonEmptyString();
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
    expect(companyMembers[0]).toContainAllKeys(
      ["companyUuid", "createdAt", "isEnabled", "role", "updatedAt", "userUuid"]
    );
    // "langId", "name", "typeAccessId", "role", "langId", "name", "roleMemberId",
    expect(companyMembers[0].companyUuid).toBe(companyUuidNoSupplier);
    expect(companyMembers[0].userUuid).toBe(authorizationUserSecond);
    expect(companyMembers[0].isEnabled).toBe(true);
    expect(companyMembers[0].role.role.roleMemberId).toBe(newRoleId);
    expect(companyMembers[0].role.access[0].name).toBeNonEmptyString();
    done();
  });

  // delete member company
  it('/graphql:M deleteCompanyMember - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
              }
            ) {
              companyUuid
              userUuid
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            deleteCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
              }
            ) {
              companyUuid
              userUuid
            }
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
            deleteCompanyMember(
              data: {
                companyUuid: "${companyUuidNoSupplier}"
                userUuid: "${authorizationUserSecond}"
              }
            ) {
              companyUuid
              userUuid
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyMember=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompanyMember },
    } = body;
    expect(deleteCompanyMember.companyUuid).toBe(companyUuidNoSupplier);
    expect(deleteCompanyMember.userUuid).toBe(authorizationUserSecond);
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
  it('/graphql:M deleteCompanyRole - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteCompanyRole( data: {
              companyUuid: "${companyUuidNoSupplier}"
              roleId: ${newRoleId}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            deleteCompanyRole( data: {
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
            deleteCompanyRole( data: {
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
    expect(deleteCompanyRole).toBe(1);
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
            deleteCompanyRole( data: {
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
    expect(deleteCompanyRole).toBe(0);
    done();
  });

  // Testing delete company
  it('/graphql:M deleteCompany - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            deleteCompany( companyUuid: "${companyUuidSupplier}") {
                uuid
                shortname
                isSupplier
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
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
            deleteCompany( companyUuid: "${companyUuidSupplier}") {
                uuid
                shortname
                isSupplier
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed delete company'
    );
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
            deleteCompany( companyUuid: "${companyUuidSupplier}") {
                uuid
                shortname
                isSupplier
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompany=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteCompany },
    } = body;
    expect(deleteCompany).toContainAllKeys(["uuid", "shortname", "isSupplier"]);
    expect(deleteCompany.uuid).toBe(companyUuidSupplier);
    expect(deleteCompany.shortname).toBe(shortname);
    expect(deleteCompany.isSupplier).toBe(true);
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
            deleteCompany( companyUuid: "${companyUuidSupplier}") {
                uuid
                shortname
                isSupplier
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed delete company'
    );
    expect(body.errors[0].path[0]).toBe('deleteCompany');
    done();
  });
});
