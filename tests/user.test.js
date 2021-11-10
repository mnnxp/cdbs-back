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
const fakeUuid = "9a9221c1-f517-40a0-a06d-fdfa8c17a462";
const baseUserUuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const baseUsername = "usernameeee";
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
var authorizationTokenUserFour = "";
var userUuidFirst = "";
var userUuidSecond = "";
var userUuidThree = "";
var userUuidFour = "";
var uploadFaviconTestUuid = "";

const username = "baromi";
const username2 = "simaco";
const username3 = "threeusername";
const username4 = "username4";
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


var typeAccessId1 = 1;
var typeAccessId2 = 2;
var typeAccessId3 = 3;

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

// for notification
const notificationUpdatePassword = "Updated password";
var notificationId = 0;

const showNotification = `\
id \
notification \
degreeImportance { \
  degreeImportanceId \
  langId \
  degree \
} \
createdAt \
isRead \
`;

const showUserAndRelatedData = ` \
uuid \
firstname \
lastname \
secondname \
username \
description \
position \
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
program { \
  id \
  name \
} \
createdAt \
updatedAt \
certificates { \
  userUuid \
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
  description \
} \
subscribers \
isFollowed \
`;

const userAndRelatedData = ` \
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
  downloadUrl \
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
typeAccess { \
  typeAccessId \
  langId \
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
  description \
} \
subscribers \
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
  downloadUrl \
} \
`;

const userCertificatesQuery = ` \
certificates { \
  userUuid \
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
  description \
} \
`;

async function cleanupTokenDb() {
  return global.knex.raw('DELETE FROM user_token_ref');
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?,?,?,?)', [
    username,
    username2,
    username3,
    username4,
    usernamePut,
  ]);
}

async function cleanupComponentParamDb() {
  return global.knex.raw('DELETE FROM param_to_component WHERE value in (?)', [
    nameComponent2,
  ]);
}

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

describe('users', () => {
  beforeAll(() => {
    cleanupTokenDb();
    cleanupUserDb();
    cleanupComponentParamDb();
    cleanupCompanyDb();
    cleanupStandardDb();
    return;
  });
  afterAll(() => {
    cleanupTokenDb();
    cleanupUserDb();
    cleanupComponentParamDb();
    cleanupCompanyDb();
    cleanupStandardDb();
    return;
  });

  const agent = request.agent(url);

  it('/graphql:Q user - UNAUTHORIZED (by username)', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query {
            user(username: "${baseUsername}") {
              ${showUserAndRelatedData}
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

  it('/graphql:Q user - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query {
            user(userUuid: "${baseUserUuid}") {
              ${showUserAndRelatedData}
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

  it('/graphql:M register - OK public profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "random@random.random",
                username: "${username4}",
                password: "${password}",
                typeAccessId: ${type_access_id_public}
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
    userUuidFour = registerUser.uuid;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
    expect(registerUser.username).toBe(username4);
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

  it('/login - OK to login four user', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username4,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenUserFour = body.bearer;
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
              ${userAndRelatedData}
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

  // update favicon user
  it('/graphql:Q uploadFavicon - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation {
          uploadFavicon (filename: "new favicon.png") {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFavicon=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Token not found.'
    );
    expect(body.errors[0].path[0]).toBe('uploadFavicon');
    done();
  });

  it('/graphql:Q uploadFavicon - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
          uploadFavicon (filename: "new favicon.png") {
            fileUuid
            filename
            uploadUrl
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql uploadFavicon=%o', body);
    const {
      data: { uploadFavicon },
    } = body;
    uploadFaviconTestUuid = uploadFavicon.fileUuid;
    expect(uploadFavicon.fileUuid).toBeNonEmptyString();
    expect(uploadFavicon.filename).toBe("new favicon.png");
    expect(uploadFavicon.uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q selfData - OK check new favicon', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            selfData{
              ${userAndRelatedData}
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
    expect(selfData.imageFile.uuid).toBe(uploadFaviconTestUuid);
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
              ${userAndRelatedData}
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
              ${userAndRelatedData}
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
              ${userAndRelatedData}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.certificates[0].file.download.filename).toBe(goodFilenameCertificateTest);
    expect(selfData.certificates[0].file.download.downloadUrl).toBeNonEmptyString();
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
              ${userAndRelatedData}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.certificates[0].file.download.filename).toBe(goodFilenameCertificateTest);
    expect(selfData.certificates[0].file.download.downloadUrl).toBeNonEmptyString();
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

  it('/graphql:Q notifications - OK not have notification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `query  {
            notifications {
              ${showNotification}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql notifications=%o', body);
    // expect(body).toBe(0);
    const {
      data: { notifications },
    } = body;
    expect(notifications).toBeEmptyArray();
    done();
  });

  it('/graphql:Q notifications - OK update password', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query  {
            notifications {
              ${showNotification}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql notifications=%o', body);
    // expect(body).toBe(0);
    const {
      data: { notifications },
    } = body;
    notificationId = notifications[0].id;
    expect(notifications[0]).toContainAllKeys([
      "id", "notification", "degreeImportance",
      "createdAt", "isRead",
    ]);
    expect(notifications[0].notification).toBe(notificationUpdatePassword);
    expect(notifications[0].degreeImportance.degreeImportanceId).toBe(5);
    expect(notifications[0].degreeImportance.degree).toBe("info");
    expect(notifications[0].isRead).toBe(false);
    done();
  });

  it('/graphql:M readNotifications - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            readNotifications(
              notificationsIds: ${notificationId}
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql readNotifications=%o', body);
    // expect(body).toBe(0);
    const {
      data: { readNotifications },
    } = body;
    expect(readNotifications).toBe(1);
    done();
  });

  it('/graphql:Q notifications - OK read notification', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query  {
            notifications {
              ${showNotification}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql notifications=%o', body);
    // expect(body).toBe(0);
    const {
      data: { notifications },
    } = body;
    notificationId = notifications[0].id;
    expect(notifications[0]).toContainAllKeys([
      "id", "notification", "degreeImportance",
      "createdAt", "isRead",
    ]);
    expect(notifications[0].id).toBe(notificationId);
    expect(notifications[0].notification).toBe(notificationUpdatePassword);
    expect(notifications[0].degreeImportance.degreeImportanceId).toBe(5);
    expect(notifications[0].degreeImportance.degree).toBe("info");
    expect(notifications[0].isRead).toBe(true);
    done();
  });

  it('/graphql:M deleteNotifications - BadRequest someone else notice', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation  {
            deleteNotifications(
              notificationsIds: ${notificationId}
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteNotifications=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteNotifications },
    } = body;
    expect(deleteNotifications).toBe(0);
    done();
  });

  it('/graphql:M deleteNotifications - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation  {
            deleteNotifications(
              notificationsIds: ${notificationId}
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteNotifications=%o', body);
    // expect(body).toBe(0);
    const {
      data: { deleteNotifications },
    } = body;
    expect(deleteNotifications).toBe(1);
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
              ${userAndRelatedData}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql UserCertificate=%o', body);
    // expect(body).toBe(0);
    const {
      data: { selfData },
    } = body;
    expect(selfData.certificates[0].file.download.filename).toBe(goodFilenameCertificateTest);
    expect(selfData.certificates[0].file.download.downloadUrl).toBeNonEmptyString();
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
        query: `mutation {
            addCompanyFav(companyUuid: "${companyUuidBase}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addCompanyFav body=%o', body);
    expect(body.data.addCompanyFav).toBe(true);
    done();
  });

  it('/graphql:M registerComponent - OK not standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `mutation  {
            registerComponent( data: {
                name: "${nameComponent2}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponent},
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
    componentUuidForFav = registerComponent.uuid;
    done();
  });

  it('/graphql:M ComponentFav - BadRequest not access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
            addComponentFav(componentUuid: "${componentUuidBase}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Access denied'
    );
    expect(body.errors[0].path[0]).toBe('addComponentFav');
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
        query: `mutation {
            addComponentFav(componentUuid: "${componentUuidForFav}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addComponentFav body=%o', body);
    expect(body.data.addComponentFav).toBe(true);
    done();
  });

  it('/graphql:M registerCompany - OK Supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `mutation newCompany {
         registerCompany( data: {
            orgname: "${orgname}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${emailCompany}",
            description: "${description}",
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

  it('/graphql:M registerStandard - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
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
    // expect(body).toBe(0);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard.uuid).toBeNonEmptyString();
    expect(registerStandard.name).toBe(nameStandard);
    standardUuidForFav = registerStandard.uuid;
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
        query: `mutation {
            addStandardFav(standardUuid: "${standardUuidForFav}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addStandardFav body=%o', body);
    expect(body.data.addStandardFav).toBe(true);
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
        query: `mutation {
            addUserFav(userUuid: "${userUuidFour}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql addUserFav body=%o', body);
    // expect(body).toBe(0);
    expect(body.data.addUserFav).toBe(true);
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
              ${userAndRelatedData}
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

  it('/graphql:Q users - OK favorite', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            users (favorite: true) {
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
    expect(users[0].uuid).toBe(userUuidFour);
    expect(users[0].username).toBe(username4);
    expect(users.length).toBe(1);
    done();
  });

  it('/graphql:Q users - OK no subscribers', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            users (subscribers: true) {
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
    expect(users).toBeEmptyArray();
    done();
  });

  it('/graphql:Q users - OK 1 subscribers', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFour}`
      )
      .send({
        query: `query {
            users (subscribers: true) {
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
    expect(users.length).toBe(1);
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
        query: `mutation {
            deleteCompanyFav(companyUuid: "${companyUuidBase}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteCompanyFav body=%o', body);
    expect(body.data.deleteCompanyFav).toBe(true);
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
        query: `mutation {
            deleteCompanyFav(companyUuid: "${companyUuidBase}")
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: No data found'
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
        query: `mutation {
            deleteComponentFav(componentUuid: "${componentUuidForFav}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteComponentFav body=%o', body);
    expect(body.data.deleteComponentFav).toBe(true);
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
        query: `mutation {
            deleteComponentFav(componentUuid: "${componentUuidForFav}")
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: No data found'
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
        query: `mutation {
            deleteStandardFav(standardUuid: "${standardUuidForFav}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteStandardFav body=%o', body);
    expect(body.data.deleteStandardFav).toBe(true);
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
        query: `mutation {
            deleteStandardFav(standardUuid: "${standardUuidForFav}")
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', body);
      expect(body.errors[0].message).toBe(
        'BadRequest: No data found'
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
        query: `mutation {
            deleteUserFav(userUuid: "${userUuidFour}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserFav body=%o', body);
    expect(body.data.deleteUserFav).toBe(true);
    done();
  });

  it('/graphql:M UserFav - Ok not found user fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `mutation {
            deleteUserFav(userUuid: "${userUuidFour}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql deleteUserFav body=%o', body);
    expect(body.data.deleteUserFav).toBe(false);
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
              ${userAndRelatedData}
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
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
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
              ${showUserAndRelatedData}
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

  it('/graphql:Q User - BadRequest not set userUuid and username', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user {
              ${showUserAndRelatedData}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Need set userUuid or username'
    );
    expect(body.errors[0].path[0]).toBe('user');
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
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
    done();
  });

  it('/graphql:Q User - Ok (by username)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(username: "${username2}") {
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
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

  // Test get users data
  it('/graphql:Q users - BadRequest not correct arguments', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `query {
            users (
              subscribers: true
              favorite: true
            ){
              ${usersListQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: You cannot request subscribers and favorites in one request'
    );
    expect(body.errors[0].path[0]).toBe('users');
    done();
  });

  it('/graphql:Q users - OK limit and offset', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `query {
            users (
              limit: 2
              offset: 1
            ){
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
    expect(users.length).toBe(2);
    done();
  });

  it('/graphql:Q users - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserThree}`
      )
      .send({
        query: `query {
            users {
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
    // expect(users[0].uuid).toBe(userUuidFour);
    // expect(users[0].username).toBe(username4);
    // expect(users[1].uuid).toBe(userUuidSecond);
    // expect(users[1].username).toBe(username);
    // expect(users.length).toBe(4);
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
            typeAccessId: ${typeAccessId2},
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
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
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
              ${showUserAndRelatedData}
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
                typeAccessId: ${typeAccessId2}
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

  it('/graphql:Q User - Ok get private profile from access component (by username)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(username: "${username2}") {
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
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
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
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
                typeAccessId: ${typeAccessId2}
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

  it('/graphql:Q User - BadRequest have access to one component but private profile (by username)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(username: "${username3}") {
              ${showUserAndRelatedData}
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

  it('/graphql:Q User - BadRequest fakeUsername (by username)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(username: "fakeusername") {
              ${showUserAndRelatedData}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'BadRequest: Data not found'
    );
    expect(body.errors[0].path[0]).toBe('user');
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
              ${showUserAndRelatedData}
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

  it('/graphql:Q User - BadRequest fakeUuid (failed check access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${fakeUuid}") {
              ${showUserAndRelatedData}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    expect(body.data).toBeNull();
    expect(body.errors[0].message).toBe(
      'Internal Server Error'
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

  // change type access user profile
  it('/graphql:M changeTypeAccessUser - Ok change access type profile to public', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation {
            changeTypeAccessUser(
              newTypeAccess: ${type_access_id_public}
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: changeTypeAccessUser
    } = body;
    expect(changeTypeAccessUser.changeTypeAccessUser).toBe(true);
    done();
  });

  it('/graphql:Q User - Ok get public profile (by username)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(username: "${username2}") {
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
    done();
  });

  it('/graphql:Q User - Ok get public profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidSecond}") {
              ${showUserAndRelatedData}
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
    expect(user.subscribers).toBe(0);
    expect(user.isFollowed).toBe(false);
    done();
  });

  it('/graphql:M changeTypeAccessUser - Ok return private access type profile', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation {
            changeTypeAccessUser(
              newTypeAccess: ${type_access_id_private}
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: changeTypeAccessUser
    } = body;
    expect(changeTypeAccessUser.changeTypeAccessUser).toBe(true);
    done();
  });

  it('/graphql:M changeTypeAccessUser - Ok this access already has', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserSecond}`
      )
      .send({
        query: `mutation {
            changeTypeAccessUser(
              newTypeAccess: ${type_access_id_private}
            )
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    // expect(body).toBe(0);
    const {
      data: changeTypeAccessUser
    } = body;
    expect(changeTypeAccessUser.changeTypeAccessUser).toBe(false);
    done();
  });

  it('/graphql:Q User - BadRequest private profile (by username)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenUserFirst}`
      )
      .send({
        query: `query {
            user(username: "${username2}") {
              ${showUserAndRelatedData}
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
              ${showUserAndRelatedData}
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
