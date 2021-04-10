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
const paramname_index = 2;
const paramname = "Selector";
const param_test_name = "testnameparametr";
const param_test_name2 = "testnameparametr2";

async function cleanupDb() {
  return global.knex.raw('DELETE FROM param_ref WHERE paramname in (?,?)', [
    param_test_name,
    param_test_name2,
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

  it('/param/add - OK', (done) => {
    agent
      .post('/param/add')
      .send({
        paramname: param_test_name
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/param/add body=%o', body);
        expect(body).toContainAllKeys([
          "id", "paramname"
        ]);
        expect(body.id).not.toBeNull();
        expect(body.paramname).toBe(param_test_name);
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParam( data: {
                paramname: "${param_test_name2}",
            }) {
              id
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerParam=%o', body);
    const {
      data: { registerParam },
    } = body;
    expect(registerParam).toContainAllKeys([
      "id", "paramname"
    ]);
    expect(registerParam.id).not.toBeNull();
    expect(registerParam.paramname).toBe(param_test_name2);
    done();
  });

  it('/graphql:M register - param name is already', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParam( data: {
                paramname: "${param_test_name2}"
            }) {
              id
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql  - param name is already =%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("This param name is already there.");
    done();
  });

  it('/graphql:Q List param - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListParam {
            param {
                id
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql all param=%o', response1.body.data.param);
    expect(response1.body.data.param).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q List param with idParamSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserParam {
            param (idParamSearch: ${paramname_index}) {
                id
                paramname
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter param=%o', response1.body.data.param);
    expect(response1.body.data.param).toBeNonEmptyArray();
    expect(response1.body.data.param[0].id).toBe(paramname_index);
    expect(response1.body.data.param[0].paramname).toBe(paramname);
    done();
  });

  it('/user/logout - OK', (done) => {
    agent.get('/user/logout').expect(HttpStatus.OK, done);
  });

  it('/param/add - Unauthorized', (done) => {
    agent
      .post('/param/add')
      .send({
        paramname: param_test_name
      })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body }) => {
        debug('/param/add body=%o', body);
        expect(body).toBe("Unauthorized");
        done();
      });
  });

  it('/graphql:M register - Unauthorized', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerParam( data: {
                paramname: "${param_test_name2}"
            }) {
              id
              paramname
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql  - not supplier registerParam=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("Unauthorized");
    done();
  });
});
