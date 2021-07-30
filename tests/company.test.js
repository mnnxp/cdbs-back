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

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname IN (?,?)', [
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
  return global.knex.raw('DELETE FROM user_tokens_ref');
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?)', [
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
                timeZone: 1,
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
      data: { registerCompany },
    } = body;
    expect(registerCompany.uuid).toBeNonEmptyString();
    expect(registerCompany.shortname).toBe(shortname);
    expect(registerCompany.isSupplier).toBe(false);
    uuidCompanyNoSupplier = registerCompany.uuid;
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

  it('/graphql:Q companies - UNAUTHORIZED List all', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query companies {
        	companies {
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
        	companies {
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
    debug('/graphql body=%o', body);
    const {
      data: { companies },
    } = body;
    expect(companies).toBeNonEmptyArray();
    expect(companies[0].orgname).toBeNonEmptyString();
    expect(companies[1].orgname).toBeNonEmptyString();
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
            uuidCompany: "${uuidCompanySupplier}",
            name: "${nameRepresentationFirst}",
            address: "${addressRepresentation}",
            phone: "${phoneRepresentation}",
            idRegion: ${idRegionRepresentation},
            idRepresentationType: ${idRepresentationType}
          }) {
            uuid
            uuidCompany
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
      ["address", "phone", "uuid", "uuidCompany"]
    );
    expect(registerCompanyRepresent.uuid).toBeNonEmptyString();
    expect(registerCompanyRepresent.uuidCompany).toBe(uuidCompanySupplier);
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
                uuidCompany: "${uuidCompanyNoSupplier}",
                name: "${nameRepresentationFirst}",
                address: "${addressRepresentation}",
                phone: "${phoneRepresentation}",
                idRegion: ${idRegionRepresentation},
                idRepresentationType: ${idRepresentationType}
            }) {
                uuid
                uuidCompany
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
            companyRepresents {
                uuid
                uuidCompany
                name
                phone
                idRegion
                idRepresentationType
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

  it('/graphql:Q List companyRepresents - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListcompanyRepresents {
            companyRepresents {
                uuid
                uuidCompany
                name
                phone
                idRegion
                idRepresentationType
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql all body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeNonEmptyArray();
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
            uuidCompany: "${uuidCompanySupplier}",
            uuidCompanyRepresent: "${uuidRepresentFirst}"
          ){
            uuid
            uuidCompany
            name
            address
            phone
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: You not have access.'
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
            uuidCompany: "${uuidFake}",
            uuidCompanyRepresent: "${uuidRepresentFirst}"
          ){
            uuid
            uuidCompany
            name
            address
            phone
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: You not have access.'
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
            uuidCompany: "${uuidCompanySupplier}",
            uuidCompanyRepresent: "${uuidFake}"
          ){
            uuid
            uuidCompany
            name
            address
            phone
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

  it('/graphql:Q Select companyRepresents with uuidCompany - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query ListcompanyRepresents {
            companyRepresents (uuidCompany: "${uuidCompanySupplier}") {
                uuid
                uuidCompany
                idRegion
                name
                phone
                idRepresentationType
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeNonEmptyArray();
    expect(response1.body.data.companyRepresents[0].uuidCompany).toBe(uuidCompanySupplier);
    expect(response1.body.data.companyRepresents.pop().uuidCompany).toBe(uuidCompanySupplier);
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
            uuidCompany: "${uuidCompanySupplier}",
            uuidCompanyRepresent: "${uuidRepresentFirst}"
          ){
            uuid
            uuidCompany
            name
            address
            phone
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyRepresent body=%o', body);
    const {
      data: { deleteCompanyRepresent },
    } = body;
    expect(deleteCompanyRepresent).toContainAllKeys(
      ["address", "name", "phone", "uuid", "uuidCompany"]
    );
    expect(deleteCompanyRepresent.uuid).toBe(uuidRepresentFirst);
    expect(deleteCompanyRepresent.uuidCompany).toBe(uuidCompanySupplier);
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
            companyRepresents (uuidCompany: "${uuidCompanySupplier}") {
                uuid
                uuidCompany
                idRegion
                name
                phone
                idRepresentationType
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql filter body=%o', response1.body);
    expect(response1.body.data.companyRepresents).toBeEmptyArray();
    done();
  });

  // Test for add memeber to company

});
