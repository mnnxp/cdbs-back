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
const is_email_verified = false;
const is_enabled = true;
const is_delete = false;
var authorizationTokenFirst = "";
var authorizationTokenSecond = "";
var userUuidFirst = "";
var userUuidSecond = "";

const username = "baromi";
const username2 = "simaco";
const password = "password";

// for update user
const emailNew = "testemail@mail.ru.new";
const firstnameNew = "test_firstname_new";
const lastnameNew = "test_lastname_new";
const secondnameNew = "test_secondname_new";
const usernameNew = "username_new";
const passwordNew = "password_new";
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

const descriptionCertificateTest = "test desctiption for certificate";
const badFilenameCertificateTest = "name* file/ certificate.pdf";
const goodFilenameCertificateTest = "name file certificate.pdf";

const companyUuidBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";
const componentUuidBase = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const standardUuidBase = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const userUuidBase = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";

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

async function cleanupTokenDb() {
  return global.knex.raw('DELETE FROM user_token_ref');
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?)', [
    username,
    username2,
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
        authorizationTokenFirst = body.bearer;
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
        // authorizationTokenSecond = body.bearer;
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
    debug('/graphql authorizationTokenFirst=%o', authorizationTokenFirst);
    debug('/graphql authorizationTokenSecond=%o', authorizationTokenSecond);
    debug('/graphql getToken.bearer=%o', response1.body.data.getToken.bearer);
    expect(response1.body.data.getToken.bearer).toBeNonEmptyString();
    authorizationTokenSecond = response1.body.data.getToken.bearer;

    const response2 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenSecond}`
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
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query deleteTokenQuery {
          deleteToken(token: "${authorizationTokenSecond}")
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', response1.body);
    const {
      data: { deleteToken },
    } = response1.body;
    expect(deleteToken).toBe('removed 1 token.');
    done();
  });

  it('/graphql:Q decodeToken - UNAUTHORIZED old token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
          `Bearer ${authorizationTokenSecond}`
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
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query deleteTokenQuery {
          deleteToken(token: "${authorizationTokenFirst}")
        }`,
      })
      .expect(HttpStatus.OK)
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'Unauthorized'
      );
      done();
  });

  it('/graphql:Q User - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(body.data.user.uuid).toBe(userUuidFirst);
    expect(body.data.user.username).toBe(username);
    expect(body.data.user.favCompaniesCount).toBe(0);
    expect(body.data.user.favComponentsCount).toBe(0);
    expect(body.data.user.favStandardsCount).toBe(0);
    expect(body.data.user.favUsersCount).toBe(0);
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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

  it('/graphql:Q User - OK check update data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidFirst}") {
              ${userFullDataQuery}
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql user=%o', body);
    // expect(body).toBe(0);
    const {
      data: { user },
    } = body;
    expect(user.email).toBe(emailPut);
    expect(user.firstname).toBe(firstnamePut);
    expect(user.lastname).toBe(lastnamePut);
    expect(user.secondname).toBe(secondnamePut);
    expect(user.username).toBe(usernamePut);
    expect(user.phone).toBe(phonePut);
    expect(user.description).toBe(descriptionPut);
    expect(user.address).toBe(addressPut);
    expect(user.position).toBe(positionPut);
    expect(user.timeZone).toBe(timeZonePut);
    expect(user.region.regionId).toBe(regionIdPut);
    expect(user.program.id).toBe(programIdPut);
    done();
  });

  it('/graphql:M putUserUpdate - OK return data', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
  it('/graphql:Q UserCertificate - BadRequest not token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query {
            user(userUuid: "${userUuidFirst}") {
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
    expect(body.errors[0].path[0]).toBe('user');
    done();
  });

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
        `Bearer ${authorizationTokenFirst}`
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
      .expect(HttpStatus.OK)
    debug('/graphql UserCertificate=%o', body);
    expect(body.data.uploadUserCertificate.fileUuid).toBeNonEmptyString();
    expect(body.data.uploadUserCertificate.filename).toBe(goodFilenameCertificateTest);
    expect(body.data.uploadUserCertificate.uploadUrl).toBeNonEmptyString();
    done();
  });

  it('/graphql:Q UserCertificate - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query {
            user(userUuid: "${userUuidFirst}") {
              certificates { \
                userUuid \
                file { \
                  uuid \
                  filename \
                  filesize \
                } \
                description \
              } \
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql UserCertificate=%o', body);
    expect(body.data.user.certificates[0].file.filename).toBe(goodFilenameCertificateTest);
    expect(body.data.user.certificates[0].description).toBe(descriptionCertificateTest);
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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

  it('/graphql:Q User - Ok fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(body.data.user.uuid).toBe(userUuidFirst);
    expect(body.data.user.username).toBe(username);
    expect(body.data.user.favCompaniesCount).toBe(1);
    expect(body.data.user.favComponentsCount).toBe(1);
    expect(body.data.user.favStandardsCount).toBe(1);
    expect(body.data.user.favUsersCount).toBe(1);
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenFirst}`
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

  it('/graphql:Q User - Ok no fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    expect(body.data.user.uuid).toBe(userUuidFirst);
    expect(body.data.user.username).toBe(username);
    expect(body.data.user.favCompaniesCount).toBe(0);
    expect(body.data.user.favComponentsCount).toBe(0);
    expect(body.data.user.favStandardsCount).toBe(0);
    expect(body.data.user.favUsersCount).toBe(0);
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

  it('/graphql:Q users - OK select uuidsUsers', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    debug('/graphql users=%o', response1.body);
    expect(response1.body.data.users).toBeNonEmptyArray();
    expect(response1.body.data.users[0].uuid).toBe(userUuidFirst);
    expect(response1.body.data.users[0].username).toBe(username);
    expect(response1.body.data.users[1].uuid).toBe(userUuidSecond);
    expect(response1.body.data.users[1].username).toBe(username2);
    done();
  });

  it('/graphql:Q getToken UNAUTHORIZED removed token', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
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
      'BadRequest: Your token is invalid.'
    );
    expect(response1.body.errors[0].path[0]).toBe('getToken');
    done();
  });

  it('/graphql:Q updateToken', async (done) => {
    const response2 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
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
    authorizationTokenSecond = response2.body.data.updateToken.bearer;

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
      `Bearer ${authorizationTokenFirst}`
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
        `Bearer ${authorizationTokenSecond}`
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
        `Bearer ${authorizationTokenSecond}`
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
      `Bearer ${authorizationTokenSecond}`
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
        `Bearer ${authorizationTokenSecond}`
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
        'BadRequest: Your token is invalid.'
      );
      done();
  });

  it('/graphql:Q showTokens - UNAUTHORIZED', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
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
});
