const debug = require('debug')('cdbs-back:user.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

const loginData = [ { "user": {
      "username": "baromi",
      "password": "password"
    }
  }
];
const baseUserUuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const email = "testemail@mail.ru";
const firstname = "test_firstname";
const lastname = "test_lastname";
const secondname = "test_secondname";
const phone = "test_phone";
const description = "test_description";
const address = "test_address";
const position = "test_position";
const time_zone = "Europe/Moscow";
const image_file_uuid = "test_image_file_uuid";
const region_id = 1;
const program_id = 1;
const type_access_id_private = 1;
const type_access_id_public = 3;
const is_email_verified = false;
const is_enabled = true;
const is_delete = false;
var authorizationTokenUserFirst = "";
var authorizationTokenUserFirstUpdate = "";
var authorizationTokenUserSecond = "";
var authorizationTokenUserThree = "";
var userUuidFirst = "";
var userUuidSecond = "";
var userUuidThree = "";

const username = "baromi";
const username2 = "simaco";
const username3 = "threeusername";
const password = "password";
const passwordBad = "pbad";
const passwordGood = "1passwordG00D!";

// for update user
const emailNew = "testemail@mail.ru.new";
const firstnameNew = "test_firstname_new";
const lastnameNew = "test_lastname_new";
const secondnameNew = "test_secondname_new";
const usernameNew = "username_new";
const phoneNew = "test_phone_new";
const descriptionNew = "test_description_new";
const addressNew = "test_address_new";
const positionNew = "test_position_new";
const timeZoneNew = "Europe/Minks";
const regionIdNew = 2;
const programIdNew = 2;

// for update user
const emailPut = "testemail@mail.ru.put";
const firstnamePut = "test_firstname_put";
const lastnamePut = "test_lastname_put";
const secondnamePut = "test_secondname_put";
const usernamePut = "usernewnameput";
const phonePut = "test_phone_put";
const descriptionPut = "test_description_put";
const addressPut = "test_address_put";
const positionPut = "test_position_put";
const timeZonePut = "Europe/Moscow";
const regionIdPut = 2;
const programIdPut = 2;


var firstAccess = 1;
var secondAccess = 2;

var langId = 1;
var nameRole = "test role";
var newRoleId = 0;
var nameRole2 = "test role2";
var newRoleId2 = 0;

// data for company
const orgname = "orgname supplier of the test";
const orgname2 = "orgnametest not supplier of the test";
const shortname = "shortnametest";
const inn = "5555555";
const phoneCompany = "7777777777";
const emailCompany = "testcompany@testemail.ru";
const descriptionCompany = "test company";
const addressCompany = "China";
const siteUrlCompany = "example.test";
const timeZoneCompany = "Europe/Moscow";
const imageFileUuid = "3706d1a1-80ae-4367-be39-af7091373811";
const regionIdCompany = 5;
const companyTypeId = 2;
var companyUuidNoSupplier = "";
var companyUuidSupplier = "";

const specIdsOk = [10,30,55];
const specIdsDup = [10,22,30,44,55];
const specIdsDel = [10,55];
const idErr = 0;

const descriptionCertificateTest = "test desctiption for certificate";
const descriptionCertificateUpdateTest = "test of the test description";
const badFilenameCertificateTest = "name* file/ certificate.pdf";
const goodFilenameCertificateTest = "name file certificate.pdf";

var fileCertificateTestUuid = "";

const companyUuidBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";
const componentUuidBase = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const standardUuidBase = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const userUuidBase = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";

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
const licenseIdOk = 1;
const licenseIdErr = 2;

const userFullDataQuery = ` \
uuid \
email \
firstname \
lastname \
secondname \
username \
phone \
description \
address \
position \
timeZone \
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
program { \
  id \
  name \
} \
isEmailVerified \
isEnabled \
isDelete \
createdAt \
updatedAt \
certificates { \
  userUuid \
  file { \
    uuid \
    filename \
    filesize \
  } \
  description \
} \
subscribers \
isFollowed \
companiesCount \
componentsCount \
standardsCount \
favCompaniesCount \
favComponentsCount \
favStandardsCount \
favUsersCount \
`;

const usersListQuery = ` \
uuid \
username \
imageFile { \
  uuid \
  filename \
  filesize \
} \
`;

const userCertificatesQuery = ` \
certificates { \
  userUuid \
  file { \
    uuid \
    filename \
    filesize \
  } \
  description \
} \
`;

async function cleanupTokenDb() {
  return global.knex.raw('DELETE FROM user_token_ref');
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?,?)', [
    username,
    username2,
    username3,
  ]);
}

describe('users', () => {
  beforeAll(() => {
    cleanupTokenDb();
    cleanupUserDb();
    return;
  });
  afterAll(() => {
    cleanupTokenDb();
    cleanupUserDb();
    return;
  });

  const agent = request.agent(url);

  it('/graphql:Q users - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query {
            user(userUuid: "${baseUserUuid}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(response1.body.errors[0].path[0]).toBe('user');
    done();
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "${email}",
                username: "${username}",
                password: "${password}",
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql users=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerUser },
    } = body;
    userUuidFirst = registerUser.uuid;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
    expect(registerUser.username).toBe(username);
    done();
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "${emailNew}",
                username: "${username2}",
                password: "${password}"
                firstname: "${firstnameNew}"
                lastname: "${lastnameNew}"
                secondname: "${secondnameNew}"
                phone: "${phoneNew}"
                description: "${descriptionNew}"
                address: "${addressNew}"
                position: "${positionNew}"
                timeZone: "${timeZoneNew}"
                regionId: ${regionIdNew}
                programId: ${programIdNew}
                typeAccessId: ${type_access_id_private}
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql users=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerUser },
    } = body;
    userUuidSecond = registerUser.uuid;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(programIdNew);
    expect(registerUser.username).toBe(username2);
    done();
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "${email}",
                username: "${username3}",
                password: "${password}",
                typeAccessId: ${type_access_id_private}
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql users=%o', body);
    // expect(body).toBe(0);
    const {
      data: { registerUser },
    } = body;
    userUuidThree = registerUser.uuid;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
    expect(registerUser.username).toBe(username3);
    done();
  });

  it('/graphql:M register - already exists', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "${email}",
                username: "${username}",
                password: "${password}",
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      'BadRequest: Failed create new user'
    );
    done();
  });

  it('/login - UNAUTHORIZED with invalid username', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": 'invalidusername',
            "password": password,
          }
        })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text, error, headers }) => {
        debug(
          '/login body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });

  it('/login - UNAUTHORIZED with invalid password', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username,
            "password": 'invalid password',
          }
        })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text, error, headers }) => {
        debug(
          '/login body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });

  it('/login - OK to login first time', (done) => {
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
        authorizationTokenUserFirst = body.bearer;
        done();
      });
  });

  it('/login - OK to login second time', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username2,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenUserSecond = body.bearer;
        done();
      });
  });

  it('/login - OK to login three user', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username3,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenUserThree = body.bearer;
        done();
      });
  });

  it('/graphql:Q decodeToken - UNAUTHORIZED without token', async (done) => {
    const response3 = await agent
      .post('/graphql')
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
        		iss
        		iat
        		exp
        		sub
        		username
          }
      }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data).toBeNull();
    expect(response3.body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(response3.body.errors[0].path[0]).toBe('decodeToken');
    done();
  });

  it('/graphql:Q decodeToken - OK', async (done) => {
    const response3 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
        		iss
        		iat
        		exp
        		sub
        		username
          }
      }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data.decodeToken.username).toBe(username);
    done();
  });

  it('/graphql:Q getToken - BadRequest too fast release of tokens', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query tokenQuery {
         getToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response1.body);
    const response2 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query tokenQuery {
         getToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response2.body);
    expect(response2.body.errors[0].message).toBe(
      'BadRequest: Please, try again later.'
    );
    done();
  });

  it('/graphql:Q getToken', async (done) => {
    await new Promise(r => setTimeout(r, 1100));
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query tokenQuery {
         getToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    debug('/graphql authorizationTokenUserFirst=%o', authorizationTokenUserFirst);
    debug('/graphql authorizationTokenUserFirstUpdate=%o', authorizationTokenUserFirstUpdate);
    debug('/graphql getToken.bearer=%o', response1.body.data.getToken.bearer);
    expect(response1.body.data.getToken.bearer).toBeNonEmptyString();
    authorizationTokenUserFirstUpdate = response1.body.data.getToken.bearer;

    const response2 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
        		iss
        		iat
        		exp
        		sub
        		username
          }
      }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response2.body);
    expect(response2.body.data.decodeToken.username).toBe(username);
    done();
  });

  it('/graphql:Q showTokens - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query showTokensQuery {
          showTokens {
            userUuid
            token
            createdAt
            expirationAt
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    const {
      data: { showTokens },
    } = response1.body;
    expect(showTokens[0]).toContainAllKeys(['userUuid', 'token', 'createdAt', 'expirationAt']);
    done();
  });

  it('/graphql:Q deleteToken - OK target token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query deleteTokenQuery {
          deleteToken(token: "${authorizationTokenUserFirstUpdate}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    const {
      data: { deleteToken },
    } = response1.body;
    expect(deleteToken).toBe(true);
    done();
  });

  it('/graphql:Q decodeToken - UNAUTHORIZED old token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
          `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
        		iss
        		iat
        		exp
        		sub
        		username
          }
      }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'Unauthorized'
      );
      done();
  });

  it('/graphql:Q deleteToken - UNAUTHORIZED old token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query deleteTokenQuery {
          deleteToken(token: "${authorizationTokenUserFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'Unauthorized'
      );
      done();
  });

  it('/graphql:Q selfData - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    let {
      data: { selfData }
    } = body;
    expect(selfData.uuid).toBe(userUuidFirst);
    expect(selfData.username).toBe(username);
    expect(selfData.favCompaniesCount).toBe(0);
    expect(selfData.favComponentsCount).toBe(0);
    expect(selfData.favStandardsCount).toBe(0);
    expect(selfData.favUsersCount).toBe(0);
    done();
  });

  // Testing user data update
  it('/graphql:M putUserUpdate - BadRequest no token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            putUserUpdate(
              data: {
                email: "${emailPut}"
                firstname: "${firstnamePut}"
                lastname: "${lastnamePut}"
                secondname: "${secondnamePut}"
                username: "${usernamePut}"
                phone: "${phonePut}"
                description: "${descriptionPut}"
                address: "${addressPut}"
                position: "${positionPut}"
                timeZone: "${timeZonePut}"
                regionId: ${regionIdPut}
                programId: ${programIdPut}
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
    expect(body.errors[0].path[0]).toBe('putUserUpdate');
    done();
  });

  it('/graphql:M putUserUpdate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            putUserUpdate(
              data: {
                email: "${emailPut}"
                firstname: "${firstnamePut}"
                lastname: "${lastnamePut}"
                secondname: "${secondnamePut}"
                username: "${usernamePut}"
                phone: "${phonePut}"
                description: "${descriptionPut}"
                address: "${addressPut}"
                position: "${positionPut}"
                timeZone: "${timeZonePut}"
                regionId: ${regionIdPut}
                programId: ${programIdPut}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putUserUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putUserUpdate },
    } = body;
    expect(putUserUpdate).toBe(13);
    done();
  });

  it('/graphql:M putUserUpdate - BadRequest data has already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            putUserUpdate(
              data: {
                email: "${emailPut}"
                firstname: "${firstnamePut}"
                lastname: "${lastnamePut}"
                secondname: "${secondnamePut}"
                username: "${usernamePut}"
                phone: "${phonePut}"
                description: "${descriptionPut}"
                address: "${addressPut}"
                position: "${positionPut}"
                timeZone: "${timeZonePut}"
                regionId: ${regionIdPut}
                programId: ${programIdPut}
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
    expect(body.errors[0].path[0]).toBe('putUserUpdate');
    done();
  });

  it('/graphql:Q selfData - OK check update data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql user=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.email).toBe(emailPut);
    expect(selfData.firstname).toBe(firstnamePut);
    expect(selfData.lastname).toBe(lastnamePut);
    expect(selfData.secondname).toBe(secondnamePut);
    expect(selfData.username).toBe(usernamePut);
    expect(selfData.phone).toBe(phonePut);
    expect(selfData.description).toBe(descriptionPut);
    expect(selfData.address).toBe(addressPut);
    expect(selfData.position).toBe(positionPut);
    expect(selfData.timeZone).toBe(timeZonePut);
    expect(selfData.region.regionId).toBe(regionIdPut);
    expect(selfData.program.id).toBe(programIdPut);
    done();
  });

  it('/graphql:M putUserUpdate - OK return data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            putUserUpdate(
              data: {
                email: "${emailPut}"
                firstname: "${firstnamePut}"
                lastname: "${lastnamePut}"
                secondname: "${secondnamePut}"
                username: "${username}"
                phone: "${phonePut}"
                description: "${descriptionPut}"
                address: "${addressPut}"
                position: "${positionPut}"
                timeZone: "${timeZonePut}"
                regionId: ${regionIdPut}
                programId: ${programIdPut}
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putUserUpdate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putUserUpdate },
    } = body;
    expect(putUserUpdate).toBe(2);
    done();
  });

  // Testing user certificates
  it('/graphql:M UserCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          uploadUserCertificate(certData: {
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
    debug('/graphql UserCertificate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('uploadUserCertificate');
    done();
  });

  it('/graphql:M UserCertificate - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          uploadUserCertificate(certData: {
            description: "${descriptionCertificateTest}"
        		filename: "${badFilenameCertificateTest}"
          }) {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql UserCertificate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { uploadUserCertificate },
    } = body;
    fileCertificateTestUuid = uploadUserCertificate.fileUuid;
    expect(uploadUserCertificate.fileUuid).toBeNonEmptyString();
    expect(uploadUserCertificate.filename).toBe(goodFilenameCertificateTest);
    expect(uploadUserCertificate.uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:M UserCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql UserCertificate=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('selfData');
    done();
  });

  it('/graphql:Q selfData - Ok check add certificate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.certificates[0].file.filename).toBe(goodFilenameCertificateTest);
    expect(selfData.certificates[0].description).toBe(descriptionCertificateTest);
    done();
  });

  // Test update user certificate description
  it('/graphql:M updateUserCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          updateUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('updateUserCertificate');
    done();
  });

  it('/graphql:M updateUserCertificate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          updateUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateUpdateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { updateUserCertificate },
    } = body;
    expect(updateUserCertificate).toBe(true);
    done();
  });

  it('/graphql:Q selfData - Ok check update description', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.certificates[0].file.filename).toBe(goodFilenameCertificateTest);
    expect(selfData.certificates[0].description).toBe(descriptionCertificateUpdateTest);
    done();
  });

  it('/graphql:M putUpdatePassword - OK duplicate password', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            putUpdatePassword(
              data: {
                oldPassword: "${password}",
                newPassword: "${password}",
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putUpdatePassword=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putUpdatePassword },
    } = body;
    expect(putUpdatePassword).toBe(false);
    done();
  });

  it('/graphql:M putUpdatePassword - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            putUpdatePassword(
              data: {
                oldPassword: "${password}",
                newPassword: "${passwordGood}",
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql putUpdatePassword=%o', body);
    // expect(body).toBe(0);
    const {
      data: { putUpdatePassword },
    } = body;
    expect(putUpdatePassword).toBe(true);
    done();
  });

  it('/graphql:M putUpdatePassword - BadRequest not valid old password', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            putUpdatePassword(
              data: {
                oldPassword: "${password}",
                newPassword: "${passwordBad}",
              }
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Password is not correct.'
    );
    expect(body.errors[0].path[0]).toBe('putUpdatePassword');
    done();
  });

  it('/graphql:M updateUserCertificate - OK return old description', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          updateUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { updateUserCertificate },
    } = body;
    expect(updateUserCertificate).toBe(true);
    done();
  });

  it('/graphql:M updateUserCertificate - OK duplicate data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          updateUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
            description: "${descriptionCertificateTest}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { updateUserCertificate },
    } = body;
    expect(updateUserCertificate).toBe(false);
    done();
  });

  it('/graphql:Q selfData - Ok check update certificate', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql UserCertificate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.certificates[0].file.filename).toBe(goodFilenameCertificateTest);
    expect(selfData.certificates[0].description).toBe(descriptionCertificateTest);
    done();
  });

  // Test delete user certificate
  it('/graphql:M deleteUserCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          deleteUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserCertificate');
    done();
  });

  it('/graphql:M deleteUserCertificate - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          deleteUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { deleteUserCertificate },
    } = body;
    expect(deleteUserCertificate).toBe(true);
    done();
  });

  it('/graphql:M deleteUserCertificate - BadRequest not found data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          deleteUserCertificate(data: {
            fileUuid: "${fileCertificateTestUuid}"
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Failed remove certificate data'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserCertificate');
    done();
  });

  it('/graphql:M CompanyFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation addCompanyFavM {
            addCompanyFav(companyUuid: "${companyUuidBase}") {
              companyUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyFav body=%o', body);
    expect(body.data.addCompanyFav.companyUuid).toBe(companyUuidBase);
    expect(body.data.addCompanyFav.userUuid).toBe(userUuidFirst);
    expect(body.data.addCompanyFav.isEnabled).toBe(true);
    done();
  });

  it('/graphql:M ComponentFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation addComponentFavM {
            addComponentFav(componentUuid: "${componentUuidBase}") {
              componentUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentFav body=%o', body);
    expect(body.data.addComponentFav.componentUuid).toBe(componentUuidBase);
    expect(body.data.addComponentFav.userUuid).toBe(userUuidFirst);
    expect(body.data.addComponentFav.isEnabled).toBe(true);
    done();
  });

  it('/graphql:M StandardFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation addStandardFavM {
            addStandardFav(standardUuid: "${standardUuidBase}") {
              standardUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardFav body=%o', body);
    expect(body.data.addStandardFav.standardUuid).toBe(standardUuidBase);
    expect(body.data.addStandardFav.userUuid).toBe(userUuidFirst);
    expect(body.data.addStandardFav.isEnabled).toBe(true);
    done();
  });

  it('/graphql:M UserFav - Ok add', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation addUserFavM {
            addUserFav(userUuid: "${userUuidBase}") {
              userFavoriteUuid
              userFollowerUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addUserFav body=%o', body);
    expect(body.data.addUserFav.userFavoriteUuid).toBe(userUuidBase);
    expect(body.data.addUserFav.userFollowerUuid).toBe(userUuidFirst);
    expect(body.data.addUserFav.isEnabled).toBe(true);
    done();
  });

  it('/graphql:Q selfData - Ok fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { selfData }
    } = body;
    expect(selfData.uuid).toBe(userUuidFirst);
    expect(selfData.username).toBe(username);
    expect(selfData.favCompaniesCount).toBe(1);
    expect(selfData.favComponentsCount).toBe(1);
    expect(selfData.favStandardsCount).toBe(1);
    expect(selfData.favUsersCount).toBe(1);
    done();
  });

  it('/graphql:M CompanyFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteCompanyFavM {
            deleteCompanyFav(companyUuid: "${companyUuidBase}") {
              companyUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyFav body=%o', body);
    expect(body.data.deleteCompanyFav.companyUuid).toBe(companyUuidBase);
    expect(body.data.deleteCompanyFav.userUuid).toBe(userUuidFirst);
    expect(body.data.deleteCompanyFav.isEnabled).toBe(false);
    done();
  });

  it('/graphql:M CompanyFav - BadRequest not found company fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteCompanyFavM {
            deleteCompanyFav(companyUuid: "${companyUuidBase}") {
              companyUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: Company not found in favotite list'
      );
      done();
  });

  it('/graphql:M ComponentFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteComponentFavM {
            deleteComponentFav(componentUuid: "${componentUuidBase}") {
              componentUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFav body=%o', body);
    expect(body.data.deleteComponentFav.componentUuid).toBe(componentUuidBase);
    expect(body.data.deleteComponentFav.userUuid).toBe(userUuidFirst);
    expect(body.data.deleteComponentFav.isEnabled).toBe(false);
    done();
  });

  it('/graphql:M ComponentFav - BadRequest not found component fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteComponentFavM {
            deleteComponentFav(componentUuid: "${componentUuidBase}") {
              componentUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: Component not found in favotite list'
      );
      done();
  });

  it('/graphql:M StandardFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteStandardFavM {
            deleteStandardFav(standardUuid: "${standardUuidBase}") {
              standardUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardFav body=%o', body);
    expect(body.data.deleteStandardFav.standardUuid).toBe(standardUuidBase);
    expect(body.data.deleteStandardFav.userUuid).toBe(userUuidFirst);
    expect(body.data.deleteStandardFav.isEnabled).toBe(false);
    done();
  });

  it('/graphql:M StandardFav - BadRequest not found standard fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteStandardFavM {
            deleteStandardFav(standardUuid: "${standardUuidBase}") {
              standardUuid
              userUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: Standard not found in favotite list'
      );
      done();
  });

  it('/graphql:M UserFav - Ok delete', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteUserFavM {
            deleteUserFav(userUuid: "${userUuidBase}") {
              userFavoriteUuid
              userFollowerUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserFav body=%o', body);
    expect(body.data.deleteUserFav.userFavoriteUuid).toBe(userUuidBase);
    expect(body.data.deleteUserFav.userFollowerUuid).toBe(userUuidFirst);
    expect(body.data.deleteUserFav.isEnabled).toBe(false);
    done();
  });

  it('/graphql:M UserFav - BadRequest not found user fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation deleteUserFavM {
            deleteUserFav(userUuid: "${userUuidBase}") {
              userFavoriteUuid
              userFollowerUuid
              isEnabled
              createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: User not found in favotite list'
      );
      done();
  });

  it('/graphql:Q selfData - Ok no fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { selfData }
    } = body;
    expect(selfData.uuid).toBe(userUuidFirst);
    expect(selfData.username).toBe(username);
    expect(selfData.favCompaniesCount).toBe(0);
    expect(selfData.favComponentsCount).toBe(0);
    expect(selfData.favStandardsCount).toBe(0);
    expect(selfData.favUsersCount).toBe(0);
    done();
  });

  it('/graphql:Q users - BadRequest not token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUsers {
            users(usersUuids: [
              "${userUuidFirst}",
              "${userUuidSecond}"
            ]) {
              ${usersListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql users=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(response1.body.errors[0].path[0]).toBe('users');
    done();
  });

  // get user member one company
  it('/graphql:Q User - Ok get public profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidFirst}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { user }
    } = body;
    expect(user.uuid).toBe(userUuidFirst);
    expect(user.username).toBe(username);
    expect(user.favCompaniesCount).toBe(0);
    expect(user.favComponentsCount).toBe(0);
    expect(user.favStandardsCount).toBe(0);
    expect(user.favUsersCount).toBe(0);
    done();
  });

  it('/graphql:Q User - BadRequest private profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidSecond}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('user');
    done();
  });

  it('/graphql:M registerCompany - OK Supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation newCompany {
         registerCompany( data: {
            orgname: "${orgname}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${emailCompany}",
            description: "${descriptionCompany}",
            address: "${addressCompany}"
            siteUrl: "${siteUrlCompany}",
            timeZone: "${timeZoneCompany}",
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

  it('/graphql:M registerCompanyRole - OK create company role', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            registerCompanyRole( data: {
              companyUuid: "${companyUuidSupplier}"
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

  it('/graphql:M addCompanyMember - OK add company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            addCompanyMember(
              data: {
                companyUuid: "${companyUuidSupplier}"
                userUuid: "${userUuidSecond}"
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
    expect(addCompanyMember.companyUuid).toBe(companyUuidSupplier);
    expect(addCompanyMember.userUuid).toBe(userUuidSecond);
    expect(addCompanyMember.roleId).toBe(newRoleId);
    done();
  });

  it('/graphql:Q User - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidSecond}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { user }
    } = body;
    expect(user.uuid).toBe(userUuidSecond);
    expect(user.username).toBe(username2);
    expect(user.favCompaniesCount).toBe(0);
    expect(user.favComponentsCount).toBe(0);
    expect(user.favStandardsCount).toBe(0);
    expect(user.favUsersCount).toBe(0);
    done();
  });

  it('/graphql:Q users - BadRequest privates profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `query ListUsers {
            users(usersUuids: [
              "${userUuidFirst}",
              "${userUuidSecond}"
            ]) {
              ${usersListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('users');
    done();
  });

  it('/graphql:M addCompanyMember - OK add three user in member  company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            addCompanyMember(
              data: {
                companyUuid: "${companyUuidSupplier}"
                userUuid: "${userUuidThree}"
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
    expect(addCompanyMember.companyUuid).toBe(companyUuidSupplier);
    expect(addCompanyMember.userUuid).toBe(userUuidThree);
    expect(addCompanyMember.roleId).toBe(newRoleId);
    done();
  });

  it('/graphql:Q users - OK select uuidsUsers all of one company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `query ListUsers {
            users(usersUuids: [
              "${userUuidFirst}",
              "${userUuidSecond}"
            ]) {
              ${usersListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql users=%o', body);
    const {
      data: { users }
    } = body;
    // expect(body).toBe(0);
    expect(users).toBeNonEmptyArray();
    expect(users[0].uuid).toBe(userUuidFirst);
    expect(users[0].username).toBe(username);
    expect(users[1].uuid).toBe(userUuidSecond);
    expect(users[1].username).toBe(username2);
    done();
  });

  it('/graphql:M deleteCompanyMember - OK del company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyMember(
              data: {
                companyUuid: "${companyUuidSupplier}"
                userUuid: "${userUuidSecond}"
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
    expect(deleteCompanyMember.companyUuid).toBe(companyUuidSupplier);
    expect(deleteCompanyMember.userUuid).toBe(userUuidSecond);
    done();
  });

  it('/graphql:M deleteCompanyMember - OK del company member', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            deleteCompanyMember(
              data: {
                companyUuid: "${companyUuidSupplier}"
                userUuid: "${userUuidThree}"
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
    expect(deleteCompanyMember.companyUuid).toBe(companyUuidSupplier);
    expect(deleteCompanyMember.userUuid).toBe(userUuidThree);
    done();
  });

  // get access from access standard
  it('/graphql:M registerStandard - OK not set parent', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            companyUuid: "${companyUuidSupplier}",
            typeAccessId: ${secondAccess},
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
    // expect(body).toBe(0);
    const {
      data: { registerStandard },
    } = body;
    standardUuidFirst = registerStandard.uuid;
    expect(registerStandard.uuid).toBeNonEmptyString();
    expect(registerStandard.name).toBe(nameStandard);
    done();
  });

  it('/graphql:M setUserAccessStandard - OK add access for second user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessStandard(
              data: {
                standardUuid: "${standardUuidFirst}"
                userUuid: "${userUuidSecond}"
                typeAccessId: ${secondAccess}
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

  it('/graphql:Q User - Ok get private profile from access standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidSecond}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { user }
    } = body;
    expect(user.uuid).toBe(userUuidSecond);
    expect(user.username).toBe(username2);
    expect(user.favCompaniesCount).toBe(0);
    expect(user.favComponentsCount).toBe(0);
    expect(user.favStandardsCount).toBe(0);
    expect(user.favUsersCount).toBe(0);
    done();
  });

  it('/graphql:M setUserAccessStandard - OK add access for three user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            setUserAccessStandard(
              data: {
                standardUuid: "${standardUuidFirst}"
                userUuid: "${userUuidThree}"
                typeAccessId: ${secondAccess}
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

  it('/graphql:Q User - BadRequest have access to one standard but private profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidThree}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('user');
    done();
  });

  it('/graphql:M deleteCompany - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
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

  // get access user from access component
  it('/graphql:M registerComponent - OK not standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
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

  it('/graphql:M setUserAccessComponent - OK add access for first user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation  {
            setUserAccessComponent(
              data: {
                componentUuid: "${componentUuidNoStandard}"
                userUuid: "${userUuidFirst}"
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

  it('/graphql:Q User - Ok get private profile from access component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidSecond}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { user }
    } = body;
    expect(user.uuid).toBe(userUuidSecond);
    expect(user.username).toBe(username2);
    expect(user.favCompaniesCount).toBe(0);
    expect(user.favComponentsCount).toBe(0);
    expect(user.favStandardsCount).toBe(0);
    expect(user.favUsersCount).toBe(0);
    done();
  });

  it('/graphql:M setUserAccessComponent - OK add access for three user', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation  {
            setUserAccessComponent(
              data: {
                componentUuid: "${componentUuidNoStandard}"
                userUuid: "${userUuidThree}"
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

  it('/graphql:Q User - BadRequest have access to one component but private profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidThree}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('user');
    done();
  });

  it('/graphql:M deleteComponent - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation  {
            deleteComponent( componentUuid: "${componentUuidNoStandard}") {
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
    expect(deleteComponent.name).toBe(nameComponent2);
    expect(deleteComponent.description).toBe(descriptionComponent);
    done();
  });

  it('/graphql:Q getToken UNAUTHORIZED removed token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query tokenQuery {
         getToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'Unauthorized'
    );
    expect(response1.body.errors[0].path[0]).toBe('getToken');
    done();
  });

  it('/graphql:Q updateToken', async (done) => {
    const response2 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query tokenQuery {
         updateToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response2.body);
    expect(response2.body.data.updateToken.bearer).toBeNonEmptyString();
    authorizationTokenUserFirstUpdate = response2.body.data.updateToken.bearer;

    const response3 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${response2.body.data.updateToken.bearer}`
      )
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
        		iss
        		iat
        		exp
        		sub
        		username
          }
      }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data.decodeToken.username).toBe(username);
    done();
  });

  it('/graphql:Q myself - UNAUTHORIZED old token', async (done) => {
    const response1 = await agent
    .post('/graphql')
    .set(
      'Authorization',
      `Bearer ${authorizationTokenUserFirst}`
    )
    .send({
      query: `query myselfQuery {
        myself {
          uuid
          programId
          username
        }
      }`,
    })
    .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'Unauthorized'
    );
    expect(response1.body.errors[0].path[0]).toBe('myself');
    done();
  });

  it('/graphql:Q myself - Ok', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query myselfQuery {
          myself {
            uuid
            programId
            username
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    const {
      data: { myself },
    } = response1.body;
    expect(myself).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(myself.username).toBe(username);
    done();
  });

  it('/graphql:Q logout - Ok', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query logoutQuery {
          logout
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql data=%o', response1.body.data);
    expect(response1.body.data.logout).toBe('Good Luck');
    done();
  });

  it('/graphql:Q myself - UNAUTHORIZED', async (done) => {
    const response1 = await agent
    .post('/graphql')
    .set(
      'Authorization',
      `Bearer ${authorizationTokenUserFirstUpdate}`
    )
    .send({
      query: `query myselfQuery {
        myself {
          uuid
          programId
          username
        }
      }`,
    })
    .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data).toBeNull();
    expect(response1.body.errors[0].message).toBe(
      'Unauthorized'
    );
    expect(response1.body.errors[0].path[0]).toBe('myself');
    done();
  });

  it('/graphql:Q updateToken UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query tokenQuery {
         updateToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'Unauthorized'
      );
      done();
  });

  it('/graphql:Q showTokens - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirstUpdate}`
      )
      .send({
        query: `query showTokensQuery {
          showTokens {
            userUuid
            token
            createdAt
            expirationAt
          }
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'Unauthorized'
      );
      done();
  });

  it('/graphql:M deleteUserData - UNAUTHORIZED not valid token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
          deleteUserData(
            password: "${passwordGood}"
          )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'Unauthorized'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserData');
    done();
  });

  it('/login - OK to login first time', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username,
            "password": passwordGood,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/login headers=%o', headers);
        expect(body.bearer).toBeNonEmptyString();
        // write new token (update old)
        authorizationTokenUserFirst = body.bearer;
        done();
      });
  });

  it('/graphql:M deleteUserData - BadRequest not valid password', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
          deleteUserData(
            password: "${password}"
          )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Password is not correct.'
    );
    expect(body.errors[0].path[0]).toBe('deleteUserData');
    done();
  });

  it('/graphql:M deleteUserData - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            deleteUserData(
              password: "${passwordGood}"
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserData=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteUserData },
    } = body;
    expect(deleteUserData).toBe(true);
    done();
  });

  it('/login - UNAUTHORIZED not found user', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username,
            "password": passwordGood,
          }
        })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text, error, headers }) => {
        debug(
          '/login body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });
});
