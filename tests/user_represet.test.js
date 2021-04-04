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
// const uuid = "";
const uuid_user = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuid_user2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
const id_region = 15;
const name = "test additional office";
const address = "Fake str, Fantom";
const phone = "+743874487556";
const id_representation_type = 1;

async function cleanupDb() {
  return global.knex.raw('DELETE FROM user_represet_ref WHERE name in (?)', [
    name,
  ]);
}
describe('represet/', () => {
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

  it('/user/login - OK is supplier', (done) => {
    agent
      .post('/user/login')
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

  it('/represet/register - OK', (done) => {
    agent
      .post('/represet/register')
      .send({
        id_region, id_representation_type, name, address, phone
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/represet/register body=%o', body);
        expect(body).toContainAllKeys(
          ['uuid', 'uuid_user', 'name', 'address', 'phone']
        );
        expect(body.uuid).not.toBeNull();
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
    done();
  });

  it('/user/logout - OK', (done) => {
    agent.get('/user/logout').expect(HttpStatus.OK, done);
  });

  it('/user/login - OK is not supplier', (done) => {
    agent
      .post('/user/login')
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

  it('/represet/register - not supplier.', (done) => {
    agent
      .post('/represet/register')
      .send({
        id_region, id_representation_type, name, address, phone
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/represet/register body=%o', body);
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
});
