const debug = require('debug')('cdbs-back:user_represent.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

const username = "usernameeee";
const username2 = "albane";
const password = "password";
const password2 = "password1";
const uuid_fail = "aba22d59-4f6c-24a4-9a37-2d38f0e577a8";
const uuid_user = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuid_user2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
const id_region = 15;
const name = "test additional office";
const address = "Fake str, Fantom";
const phone = "+743874487556";
const id_representation_type = 1;
const uuid_represent = [];

async function cleanupDb() {
  return global.knex.raw('DELETE FROM user_represent_ref WHERE name in (?)', [
    name,
  ]);
}
describe('represents', () => {
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

  it('/users/login - OK is supplier', (done) => {
    agent
      .post('/users/login')
      .send({ username, password })
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

  it('/represents - OK', (done) => {
    agent
      .post('/represents')
      .send({
        id_region, id_representation_type, name, address, phone
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/represents body=%o', body);
        expect(body).toContainAllKeys(
          ['uuid', 'uuid_user', 'name', 'address', 'phone']
        );
        expect(body.uuid).not.toBeNull();
        uuid_represent.push(body.uuid);  // <-- for test delete represent
        expect(body.uuid_user).toBe(uuid_user);
        expect(body.name).toBe(name);
        expect(body.address).toBe(address);
        expect(body.phone).toBe(phone);
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUserRepresent( data: {
                uuidUser: "${uuid_user}",
                name: "${name}",
                address: "${address}",
                phone: "${phone}",
                idRegion: ${id_region},
                idRepresentationType: ${id_representation_type}
            }) {
                uuid
                uuidUser
                name
                address
                phone
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerUserRepresent=%o', body);
    const {
      data: { registerUserRepresent },
    } = body;
    expect(registerUserRepresent).toContainAllKeys(['uuid', 'uuidUser',
      'name', 'address', 'phone']);
    expect(registerUserRepresent.uuid).toBeNonEmptyString();
    uuid_represent.push(registerUserRepresent.uuid);  // <-- for test delete represent not owned user
    expect(registerUserRepresent.uuidUser).toBe(uuid_user);
    expect(registerUserRepresent.name).toBe(name);
    expect(registerUserRepresent.address).toBe(address);
    expect(registerUserRepresent.phone).toBe(phone);
    done();
  });

  it('/graphql:M register - Not correct UUID', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUserRepresent( data: {
                uuidUser: "${uuid_user2}",
                name: "${name}",
                address: "${address}",
                phone: "${phone}",
                idRegion: ${id_region},
                idRepresentationType: ${id_representation_type}
            }) {
                uuid
                uuidUser
                name
                address
                phone
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql  - Not correct UUID registerUserRepresent=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("Uuid not correct.");
    done();
  });

  it('/graphql:Q List userRepresent - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserRepresent {
            userRepresent {
                uuid
                uuidUser
                name
                phone
                idRegion
                idRepresentationType
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql all userRepresent=%o', response1.body.data.userRepresent);
    expect(response1.body.data.userRepresent).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q List userRepresent with uuidUser - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserRepresent {
            userRepresent (uuidUser: "${uuid_user}") {
                uuid
                uuidUser
                idRegion
                name
                phone
                idRepresentationType
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter userRepresent=%o', response1.body.data.userRepresent);
    expect(response1.body.data.userRepresent).toBeNonEmptyArray();
    expect(response1.body.data.userRepresent[0].uuidUser).toBe(uuid_user);
    expect(response1.body.data.userRepresent.pop().uuidUser).toBe(uuid_user);
    done();
  });

  it('/represents - Delete bad uuid', (done) => {
    agent
      .delete('/represents/' + 'BadUuid')
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represents body=%o', body);
        expect(body).toBe("Invalid UUID");
        done();
      });
  });

  it('/represents - Delete random uuid', (done) => {
    agent
      .delete('/represents/' + uuid_fail)
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represents body=%o', body);
        expect(body).toBe("The representative not you or not found.");
        done();
      });
  });

  it('/represents - Delete OK', (done) => {
    agent
      .delete('/represents/' + uuid_represent[0])
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/represents body=%o', body);
        expect(body).toContainAllKeys(
          ['uuid', 'uuid_user', 'name', 'address', 'phone']
        );
        expect(body.uuid).toBe(uuid_represent[0]);
        expect(body.uuid_user).toBe(uuid_user);
        expect(body.name).toBe(name);
        expect(body.address).toBe(address);
        expect(body.phone).toBe(phone);
        done();
      });
  });

  it('/users/logout - OK', (done) => {
    agent.get('/users/logout').expect(HttpStatus.OK, done);
  });

  it('/users/login - OK is not supplier', (done) => {
    agent
      .post('/users/login')
      .send({ username: username2, password: password2 })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['username', 'is_supplier', 'uuid']);
        expect(body.username).toBe(username2);
        expect(body.is_supplier).toBe(0);
        expect(body.uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/represents - not supplier.', (done) => {
    agent
      .post('/represents')
      .send({
        id_region, id_representation_type, name, address, phone
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represents body=%o', body);
        expect(body).toBe("You are not supplier.");
        done();
      });
  });

  it('/graphql:M register - not supplier.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUserRepresent( data: {
                uuidUser: "${uuid_user2}",
                name: "${name}",
                address: "${address}",
                phone: "${phone}",
                idRegion: ${id_region},
                idRepresentationType: ${id_representation_type}
            }) {
                uuid
                uuidUser
                name
                address
                phone
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql  - not supplier registerUserRepresent=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("You are not supplier.");
    done();
  });

  it('/represents - Delete not onwed', (done) => {
    agent
      .delete('/represents/' + uuid_represent[1])
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represents body=%o', body);
        expect(body).toBe("The representative not you or not found.");
        done();
      });
  });
});
