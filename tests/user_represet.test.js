const debug = require('debug')('cdbs-back:user_represet.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

const nickname = "nicknameeee";
const nickname2 = "albane";
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
const uuid_represet = [];

async function cleanupDb() {
  return global.knex.raw('DELETE FROM user_represet_ref WHERE name in (?)', [
    name,
  ]);
}
describe('represets', () => {
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
      .send({ nickname, password })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['nickname', 'is_supplier', 'uuid']);
        expect(body.nickname).toBe(nickname);
        expect(body.is_supplier).toBe(1);
        expect(body.uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/represets - OK', (done) => {
    agent
      .post('/represets')
      .send({
        id_region, id_representation_type, name, address, phone
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/represets body=%o', body);
        expect(body).toContainAllKeys(
          ['uuid', 'uuid_user', 'name', 'address', 'phone']
        );
        expect(body.uuid).not.toBeNull();
        uuid_represet.push(body.uuid);  // <-- for test delete represet
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
            registerUserRepreset( data: {
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
    debug('/graphql registerUserRepreset=%o', body);
    const {
      data: { registerUserRepreset },
    } = body;
    expect(registerUserRepreset).toContainAllKeys(['uuid', 'uuidUser',
      'name', 'address', 'phone']);
    expect(registerUserRepreset.uuid).toBeNonEmptyString();
    uuid_represet.push(registerUserRepreset.uuid);  // <-- for test delete represet not owned user
    expect(registerUserRepreset.uuidUser).toBe(uuid_user);
    expect(registerUserRepreset.name).toBe(name);
    expect(registerUserRepreset.address).toBe(address);
    expect(registerUserRepreset.phone).toBe(phone);
    done();
  });

  it('/graphql:M register - Not correct UUID', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUserRepreset( data: {
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
    debug('/graphql  - Not correct UUID registerUserRepreset=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("Uuid not correct.");
    done();
  });

  it('/graphql:Q List userRepreset - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserRepreset {
            userRepreset {
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
    debug('/graphql all userRepreset=%o', response1.body.data.userRepreset);
    expect(response1.body.data.userRepreset).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q List userRepreset with uuidUserSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserRepreset {
            userRepreset (uuidUserSearch: "${uuid_user}") {
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
    debug('/graphql filter userRepreset=%o', response1.body.data.userRepreset);
    expect(response1.body.data.userRepreset).toBeNonEmptyArray();
    expect(response1.body.data.userRepreset[0].uuidUser).toBe(uuid_user);
    expect(response1.body.data.userRepreset.pop().uuidUser).toBe(uuid_user);
    done();
  });

  it('/represets - Delete bad uuid', (done) => {
    agent
      .delete('/represets/' + 'BadUuid')
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represets body=%o', body);
        expect(body).toBe("Invalid UUID");
        done();
      });
  });

  it('/represets - Delete random uuid', (done) => {
    agent
      .delete('/represets/' + uuid_fail)
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represets body=%o', body);
        expect(body).toBe("The representative not you or not found.");
        done();
      });
  });

  it('/represets - Delete OK', (done) => {
    agent
      .delete('/represets/' + uuid_represet[0])
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/represets body=%o', body);
        expect(body).toContainAllKeys(
          ['uuid', 'uuid_user', 'name', 'address', 'phone']
        );
        expect(body.uuid).toBe(uuid_represet[0]);
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
      .send({ nickname: nickname2, password: password2 })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['nickname', 'is_supplier', 'uuid']);
        expect(body.nickname).toBe(nickname2);
        expect(body.is_supplier).toBe(0);
        expect(body.uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/represets - not supplier.', (done) => {
    agent
      .post('/represets')
      .send({
        id_region, id_representation_type, name, address, phone
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represets body=%o', body);
        expect(body).toBe("You are not supplier.");
        done();
      });
  });

  it('/graphql:M register - not supplier.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUserRepreset( data: {
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
    debug('/graphql  - not supplier registerUserRepreset=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("You are not supplier.");
    done();
  });

  it('/represets - Delete not onwed', (done) => {
    agent
      .delete('/represets/' + uuid_represet[1])
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represets body=%o', body);
        expect(body).toBe("The representative not you or not found.");
        done();
      });
  });
});
