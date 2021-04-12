const debug = require('debug')('cdbs-back:param.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

const nickname = "nicknameeee";
const password = "password";
const uuid_user_create = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuid_component = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const uuid_modification = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const paramname_index_fail = 100;
const paramname_index = 5;
const paramname = "Selector";
const param_test_value = "testvalueparametr";
const param_test_value2 = "testvalueparametr2";

async function cleanupDb() {
  return global.knex.raw('DELETE FROM param_to_modification WHERE value in (?,?)', [
    param_test_value,
    param_test_value2,
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

  it('/param/modification - OK', (done) => {
    agent
      .post('/param/modification')
      .send({
        uuid: uuid_modification,
        id_param: paramname_index,
        value: param_test_value
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/param/modification body=%o', body);
        expect(body).toContainAllKeys([
          "id", "uuid", "id_param", "value"
        ]);
        expect(body.id).not.toBeNull();
        expect(body.value).toBe(param_test_value);
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamModification( data: {
                uuid: "${uuid_modification}",
                idParam: ${paramname_index},
                value: "${param_test_value2}"
            }) {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerParamModification=%o', body);
    const {
      data: { registerParamModification },
    } = body;
    expect(registerParamModification).toContainAllKeys([
      "id", "uuid", "idParam", "value"
    ]);
    expect(registerParamModification.id).not.toBeNull();
    expect(registerParamModification.value).toBe(param_test_value2);
    done();
  });

  it('/graphql:M register - param is already has', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamModification( data: {
                uuid: "${uuid_modification}",
                idParam: ${paramname_index},
                value: "${param_test_value2}"
            }) {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - param is already has =%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("This param name is already with the component.");
    done();
  });

  it('/graphql:Q List param - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListParam {
            paramModification {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql all param=%o', response1.body.data.paramModification);
    expect(response1.body.data.paramModification).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q List param filter idParamSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListParamModification {
            paramModification (idParamSearch: ${paramname_index}) {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter idParamSearch =%o', response1.body.data.paramModification);
    expect(response1.body.data.paramModification).toBeNonEmptyArray();
    expect(response1.body.data.paramModification.pop().id).not.toBeNull();
    expect(response1.body.data.paramModification.pop().idParam).toBe(paramname_index);
    done();
  });

  it('/graphql:Q List param filter uuidModificationSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListParamModification {
            paramModification (uuidModificationSearch: "${uuid_modification}") {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter uuidModificationSearch =%o', response1.body.data.paramModification);
    expect(response1.body.data.paramModification).toBeNonEmptyArray();
    expect(response1.body.data.paramModification.pop().id).not.toBeNull();
    expect(response1.body.data.paramModification.pop().uuid).toBe(uuid_modification);
    done();
  });

  it('/graphql:Q List param filter idParam and uuidModificationSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListParamModification {
            paramModification (
              idParamSearch: ${paramname_index},
              uuidModificationSearch: "${uuid_modification}"
            ) {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter idParam and uuidModificationSearch =%o', response1.body.data.paramModification);
    expect(response1.body.data.paramModification).toBeNonEmptyArray();
    expect(response1.body.data.paramModification.pop().id).not.toBeNull();
    expect(response1.body.data.paramModification.pop().idParam).toBe(paramname_index);
    expect(response1.body.data.paramModification.pop().uuid).toBe(uuid_modification);
    done();
  });

  it('/user/logout - OK', (done) => {
    agent.get('/user/logout').expect(HttpStatus.OK, done);
  });

  it('/param/modification - Unauthorized', (done) => {
    agent
      .post('/param/modification')
      .send({
        uuid: uuid_modification,
        id_param: paramname_index,
        value: param_test_value
      })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body }) => {
        debug('/param/modification body=%o', body);
        expect(body).toBe("Unauthorized");
        done();
      });
  });

  it('/graphql:M register - Unauthorized', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParamModification( data: {
                uuid: "${uuid_modification}",
                idParam: ${paramname_index},
                value: "${param_test_value2}"
            }) {
                id
                uuid
                idParam
                value
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - not supplier registerParamModification=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("Unauthorized");
    done();
  });
});
