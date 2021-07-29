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
const uuidUser = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuidUser2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";

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
const timeZone = 3;
const uuidImageFile = "3706d1a1-80ae-4367-be39-af7091373811";
const idRegionCompany = 5;
const idTypeOrg = 2;
const uuidCompanyBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";
var uuidCompanyNoSupplier = "";
var uuidCompanySupplier = "";

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname IN (?,?)', [
    orgname,
    orgname2,
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

describe('users', () => {
  beforeAll(async () => {
    cleanupCompanyDb();
    cleanupTokenDb();
    cleanupUserDb();
    return;
  });
  afterAll(async () => {
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
            userRegister( data: {
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
                timeZone: 1,
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
      data: { userRegister },
    } = body;
    expect(userRegister).toContainAllKeys(['uuid', 'idProgram', 'username']);
    expect(userRegister.uuid).toBeNonEmptyString();
    expect(userRegister.idProgram).toBe(1);
    expect(userRegister.username).toBe(username);
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

  it('/graphql:M companyRegister - OK Supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation newCompany {
         companyRegister( data: {
            orgname: "${orgname}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: ${timeZone},
            uuidImageFile: "${uuidImageFile}",
            idRegion: ${idRegionCompany},
            idTypeOrg: ${idTypeOrg}
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
      data: { companyRegister },
    } = body;
    expect(companyRegister.uuid).toBeNonEmptyString();
    expect(companyRegister.shortname).toBe(shortname);
    expect(companyRegister.isSupplier).toBe(false);
    uuidCompanySupplier = companyRegister.uuid;
    done();
    // change supplier status on 1
    await global.knex.raw('UPDATE company_ref SET is_supplier=? WHERE shortname=?', [
      't',
      shortname,
    ]);
  });

  it('/graphql:Q companies - OK Select supplier company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query companies {
        	companies (uuidCompany: "${uuidCompanySupplier}"){
            uuid
            orgname
            shortname
            inn
            phone
            email
            description
            address
            siteUrl
            timeZone
            uuidUser
            uuidImageFile
            idRegion
            idTypeOrg
            isSupplier
            isEmailVerified
            isEnabled
            isDelete
            createdAt
            updatedAt
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companies=%o', body);
    const {
      data: { companies },
    } = body;
    expect(companies[0].uuid).toBe(uuidCompanySupplier);
    expect(companies[0].orgname).toBe(orgname);
    expect(companies[0].isSupplier).toBe(true);
    done();
  });

  it('/graphql:M companyRegister - OK NoSupplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation newCompany {
         companyRegister( data: {
            orgname: "${orgname2}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: ${timeZone},
            uuidImageFile: "${uuidImageFile}",
            idRegion: ${idRegionCompany},
            idTypeOrg: ${idTypeOrg}
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
      data: { companyRegister },
    } = body;
    expect(companyRegister.uuid).toBeNonEmptyString();
    expect(companyRegister.shortname).toBe(shortname);
    expect(companyRegister.isSupplier).toBe(false);
    uuidCompanyNoSupplier = companyRegister.uuid;
    done();
  });

  it('/graphql:Q companies - OK Select no supplier company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query companies {
        	companies (uuidCompany: "${uuidCompanyNoSupplier}"){
            uuid
            orgname
            shortname
            inn
            phone
            email
            description
            address
            siteUrl
            timeZone
            uuidUser
            uuidImageFile
            idRegion
            idTypeOrg
            isSupplier
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql companies=%o', body);
    const {
      data: { companies },
    } = body;
    expect(companies[0].uuid).toBe(uuidCompanyNoSupplier);
    expect(companies[0].orgname).toBe(orgname2);
    expect(companies[0].isSupplier).toBe(false);
    done();
  });
});
