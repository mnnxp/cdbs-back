const debug = require('debug')('cdbs-back:user.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

const firstname = "testfirstname";
const lastname = "testlastname";
const secondname = "testsecondname";
const loginData = [ { "user": {
      "username": "baromi",
      "password": "password"
    }
  }
];
const username = "baromi";
const username2 = "simaco";
const email = "email@baromi.com";
const password = "password";
const id_type_user = 2;
const id_type_user1 = 1;
const is_supplier0 = 0;
const is_supplier = 1;
const orgname = "Inver SAS";
const shortname = "ISAS";
const inn = "778855443322";
const phone = "+499885522441";
const id_name_cad = 2;
const comment = "This is fake.";
const address = "UK, Central str.";
const time_zone = 6;
const position = "superposition";
const site_url = "dot.com.net";
const uuid_file_info_icon = "bc1c2151-86d0-4656-9c9d-d016dd584297";
const id_region = 51;

async function cleanupDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username in (?,?)', [
    username,
    username2,
  ]);
}
describe('users', () => {
  beforeAll(async () => {
    return cleanupDb();
  });
  afterAll(async () => {
    return cleanupDb();
  });

  // const app = express();
  // app.use(cookieParser());
  //
  // app.get('/', function(req, res) {
  //     res.cookie('cookie', 'hey');
  //     res.send();
  // });
  //
  // app.get('/return', function(req, res) {
  //     if (req.cookies.cookie) res.send(req.cookies.cookie);
  //     else res.send(':(')
  // });

  const agent = request.agent(url);

  it('/users/me - UNAUTHORIZED before register', (done) => {
    agent
      .get('/users/me')
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text }) => {
        debug('/users/me body=%o text=%s', body, text);
        expect(body).toBe('Unauthorized');
        expect(text).toBe('"Unauthorized"');
        done();
      });
  });

  it('/users - OK', (done) => {
    agent
      .post('/users')
      .send({
        firstname, lastname, secondname, username,
        email, password, id_type_user, is_supplier, orgname, shortname,
        inn, phone, id_name_cad, comment, address, time_zone, position,
        site_url, uuid_file_info_icon, id_region
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/users body=%o', body);
        expect(body).toContainAllKeys(['uuid', 'is_supplier', 'username']);
        expect(body.uuid).not.toBeNull();
        expect(body.is_supplier).toBe(1);
        expect(body.username).toBe(username);
        done();
      });
  });

  it('/users - Bad Request', (done) => {
    agent
      .post('/users')
      .send({
        firstname, lastname, secondname, username,
        email, password, id_type_user, is_supplier, orgname, shortname,
        inn, phone, id_name_cad, comment, address, time_zone, position,
        site_url, uuid_file_info_icon, id_region
       })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body, error, text, headers }) => {
        debug(
          '/users body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe(
          '\"Key (username)=(baromi) already exists.\"'
        );
        expect(body).toBe('Key (username)=(baromi) already exists.');
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                firstname: "testfirstname",
                lastname: "testlastname",
                secondname: "testsecondname",
                username: "${username2}",
                email: "testemail@testemail.ru",
                password: "password",
                idTypeUser: 1,
                isSupplier: 1,
                orgname: "testorgname",
                shortname: "testshortname",
                inn: "testinn",
                phone: "testphone",
                idNameCad: 5,
                comment: "testcomment",
                address: "testaddress",
                timeZone: 3,
                position: "testaddress",
                siteUrl: "testsiteUrl",
                uuidFileInfoIcon: "bc1c2151-86d0-4656-9c9d-d016dd584297",
                idRegion: 13
            }) {
                uuid
                isSupplier
                username
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql users=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'isSupplier', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.isSupplier).toBe(1);
    expect(registerUser.username).toBe(username2);
    done();
  });

  it('/graphql:M register - Key (username)=(simaco) already exists.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                firstname: "testfirstname",
                lastname: "testlastname",
                secondname: "testsecondname",
                username: "${username2}",
                email: "testemail@testemail.ru",
                password: "password",
                idTypeUser: 1,
                isSupplier: 1,
                orgname: "testorgname",
                shortname: "testshortname",
                inn: "testinn",
                phone: "testphone",
                idNameCad: 5,
                comment: "testcomment",
                address: "testaddress",
                timeZone: 3,
                position: "testaddress",
                siteUrl: "testsiteUrl",
                uuidFileInfoIcon: "bc1c2151-86d0-4656-9c9d-d016dd584297",
                idRegion: 13
            }) {
                uuid
                isSupplier
                username
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      'Key (username)=(simaco) already exists.'
    );
    done();
  });

  it('/users/me - UNAUTHORIZED before login', (done) => {
    agent
      .get('/users/me')
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text }) => {
        debug('/users/me body=%o text=%s', body, text);
        expect(text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });

  it('/users/login - UNAUTHORIZED with invalid username', (done) => {
    agent
      .post('/users/login')
      // .send({ username: 'invalidusername', password })
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

  it('/users/login - UNAUTHORIZED with invalid password', (done) => {
    agent
      .post('/users/login')
      // .send({ username, password: 'invalid password' })
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

  it('/users/login - OK to login first time', (done) => {
    agent
      .post('/users/login')
      // .send({ username, password })
      .send({ "user": {
            "username": username,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/users/login headers=%o', headers);
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['username', 'is_supplier', 'uuid']);
        expect(body.username).toBe(username);
        expect(body.is_supplier).toBe(1);
        expect(body.uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/users/login - OK to login second time', (done) => {
    agent
      .post('/users/login')
      // .send({ username, password })
      .send({ "user": {
            "username": username,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['username', 'is_supplier', 'uuid']);
        expect(body.username).toBe(username);
        expect(body.is_supplier).toBe(1);
        expect(body.uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/graphql:Q users - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUsers {
            users {
                uuid
                email
                emailVerified
                idTypeUser
                isSupplier
                firstname
                lastname
                secondname
                username
                orgname
                shortname
                inn
                phone
                idNameCad
                comment
                address
                timeZone
                position
                siteUrl
                uuidFileInfoIcon
                idRegion
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql users=%o', response1.body.data.users);
    expect(response1.body.data.users).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q decodeToken - NO_ACCESS', async (done) => {
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
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data).toBeNull();
    expect(response3.body.errors[0].message).toBe(
      'Unauthorized'
    );
    expect(response3.body.errors[0].path[0]).toBe('decodeToken');
    expect(response3.body.errors[0].extensions.type).toBe('NO_ACCESS');
    done();
  });

  it('/graphql:Q generateToken', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query tokenQuery {
         generateToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.generateToken.bearer).toBeNonEmptyString();

    const response3 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${response1.body.data.generateToken.bearer}`
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
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data.decodeToken.username).toBe(username);
    expect(response3.body.data.decodeToken.iss).toBe('0.0.0.0');

    done();
  });

  it('/users/me - OK', (done) => {
    agent
      .get('/users/me')
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/users/me body=%o', body);
        debug('/users/me headers=%o', headers);
        expect(body).toContainAllKeys(['uuid', 'is_supplier', 'username']);
        expect(body.uuid).not.toBeNull();
        expect(body.is_supplier).toBe(1);
        expect(body.username).toBe(username);
        done();
      });
  });

  it('/users/logout - OK', (done) => {
    agent.get('/users/logout').expect(HttpStatus.OK, done);
  });

  it('/users/me - UNAUTHORIZED', (done) => {
    agent.get('/users/me').expect(HttpStatus.UNAUTHORIZED, done);
  });

  it('/graphql:Q users - You need to have is_supplier 1, but have is_supplier 0', async (done) => {
    await global.knex.raw('UPDATE user_ref SET is_supplier=? WHERE username=?', [
      0,
      username,
    ]);
    {
      const { body, headers } = await agent
        .post('/users/login')
        .send({ username, password })
        .expect(HttpStatus.OK);
      expect(headers['set-cookie'][0]).toBeNonEmptyString();
      expect(body).toContainAllKeys(['username', 'is_supplier', 'uuid']);
      expect(body.username).toBe(username);
      expect(body.is_supplier).toBe(0);
      expect(body.uuid).toBeNonEmptyString();
    }
    {
      const response1 = await agent
        .post('/graphql')
        .send({
          query: `query ListUsers {
            users {
                uuid
                email
                username
                createdAt
            }
        }`,
        })
        .expect(HttpStatus.OK);
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'You are not supplier.'
      );
    }
    done();
  });
});
