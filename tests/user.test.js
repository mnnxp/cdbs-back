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
const uuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const email = "testemail@mail.ru";
const psw_hash = "test_psw_hash";
const psw_salt = "test_psw_salt";
const firstname = "test_firstname";
const lastname = "test_lastname";
const secondname = "test_secondname";
const username = "baromi";
const username2 = "simaco";
const password = "password";
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

const companyUuidBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";

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
  pathFile \
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
    pathFile \
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
  pathFile \
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
        query: `query ListUsers {
            user(userUuid: "${userUuidFirst}") {
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
                email: "testemail@mail.ru",
                firstname: "test_firstname",
                lastname: "test_lastname",
                secondname: "test_secondname",
                username: "${username}",
                password: "password",
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
    debug('/graphql users=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
    expect(registerUser.username).toBe(username);
    userUuidFirst = registerUser.uuid;
    done();
  });

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
                username: "${username2}",
                password: "password",
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
    debug('/graphql users=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
    expect(registerUser.username).toBe(username2);
    userUuidSecond = registerUser.uuid;
    done();
  });

  it('/graphql:M register - already exists', async (done) => {
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
                password: "password",
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
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      'BadRequest: Key (username)=(simaco) already exists.'
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
          '/users/login body=%o text=%o error=%o headers=%o ',
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
          '/users/login body=%o text=%o error=%o headers=%o ',
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

  it('/graphql:Q user - Ok', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListUsers {
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

  it('/graphql:M user - Ok add company fav', async (done) => {
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

  it('/graphql:Q user - Ok add company fav', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query ListUsers {
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
    expect(body.data.user.favComponentsCount).toBe(0);
    expect(body.data.user.favStandardsCount).toBe(0);
    expect(body.data.user.favUsersCount).toBe(0);
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
