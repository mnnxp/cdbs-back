const debug = require('debug')('cdbs-back:component.test.js');
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
const name = "Test Fluted Knobs – Tapped (Phenolic)";
const name2 = "Bolt DIN 7964 - M10 x 35 - LC - Sp";
const name3 = "LDH100: Oil humidity sensor; G 3/4; Connector; [oil]: Medium";
const comment = "Phenolic Plastic";
const comment2 = "Technology for Engineering";
const comment3 = "Sensor Technology, Networking and Control Technique for Automation";
const uuid_component_parent = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const uuid_component_parent2 = "e925833e-f8d3-4ecb-bd67-5aa450f9f0ad";
const id_actual_status = 1;
const id_component_type = 1;
const is_delete = 0;
const id_type_access = 1;
const commentchange = "none";
const is_standard = 0;
const is_standard1 = 1;

async function cleanupDb() {
  return global.knex.raw('DELETE FROM component_ref WHERE name in (?,?)', [
    name,
    name2
  ]);
}
describe('component/', () => {
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

  it('/component/add - OK', (done) => {
    agent
      .post('/component/add')
      .send({
        name, comment, uuid_component_parent, id_actual_status,
        id_component_type, id_type_access, is_standard
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/component/add body=%o', body);
        expect(body).toContainAllKeys(
          ["uuid", "name", "comment", "id_actual_status", "is_standard", "created_at"]
        );
        expect(body.uuid).not.toBeNull();
        expect(body.name).toBe(name);
        expect(body.comment).toBe(comment);
        expect(body.id_actual_status).toBe(id_actual_status);
        expect(body.is_standard).toBe(is_standard);
        done();
      });
  });

  it('/component/add - Bad Request', (done) => {
    agent
      .post('/component/add')
      .send({
        name, comment, uuid_component_parent, id_actual_status,
        id_component_type, id_type_access, is_standard
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body, error, text, headers }) => {
        debug(
          '/component/add body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe(
          '\"Key (name, uuid_user)=(Test Fluted Knobs – Tapped (Phenolic), 31ecc6f8-0c09-4a59-a2d5-34b5b833e59b) already exists.\"'
        );
        expect(body).toBe('Key (name, uuid_user)=(Test Fluted Knobs – Tapped (Phenolic), 31ecc6f8-0c09-4a59-a2d5-34b5b833e59b) already exists.');
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent( data: {
                name: "${name2}",
                uuidUser: "${uuid_user}",
                comment: "${comment2}",
                uuidComponentParent: "${uuid_component_parent}",
                idActualStatus: ${id_actual_status},
                idComponentType: ${id_component_type},
                isDelete: ${is_delete},
                idTypeAccess: ${id_type_access},
                commentchange: "${commentchange}",
                isStandard: ${is_standard1}
            }) {
                uuid
                name
                comment
                idActualStatus
                isStandard
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    expect(registerComponent).toContainAllKeys([
      'uuid', 'name', 'comment', 'idActualStatus', 'isStandard', 'createdAt'
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(name2);
    expect(registerComponent.comment).toBe(comment2);
    expect(registerComponent.idActualStatus).toBe(id_actual_status);
    expect(registerComponent.isStandard).toBe(is_standard1);
    done();
  });

  it('/graphql:M register - Key (name)=(Bolt DIN...) already exists.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent( data: {
                name: "${name2}",
                uuidUser: "${uuid_user}",
                comment: "${comment2}",
                uuidComponentParent: "${uuid_component_parent}",
                idActualStatus: ${id_actual_status},
                idComponentType: ${id_component_type},
                isDelete: ${is_delete},
                idTypeAccess: ${id_type_access},
                commentchange: "${commentchange}",
                isStandard: ${is_standard1}
            }) {
                uuid
                name
                comment
                idActualStatus
                isStandard
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      'Key (name, uuid_user)=(Bolt DIN 7964 - M10 x 35 - LC - Sp, 31ecc6f8-0c09-4a59-a2d5-34b5b833e59b) already exists.'
    );
    done();
  });

  it('/graphql:Q List Component - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListComponent {
            components {
                uuid
                name
                uuidUser
                comment
                uuidComponentParent
                idActualStatus
                idComponentType
                isDelete
                idTypeAccess
                commentchange
                isStandard
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql all components=%o', response1.body.data.components);
    expect(response1.body.data.components).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q Components with uuidComponentSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query Component {
            components (uuidComponentSearch: "${uuid_component_parent}") {
                uuid
                name
                uuidUser
                comment
                uuidComponentParent
                idActualStatus
                idComponentType
                isDelete
                idTypeAccess
                commentchange
                isStandard
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter components=%o', response1.body.data.components);
    expect(response1.body.data.components).toBeNonEmptyArray();
    expect(response1.body.data.components[0].uuid).toBe(uuid_component_parent);
    expect(response1.body.data.components.pop().uuid).toBe(uuid_component_parent);
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

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent( data: {
                name: "${name2}",
                uuidUser: "${uuid_user2}",
                comment: "${comment2}",
                uuidComponentParent: "${uuid_component_parent}",
                idActualStatus: ${id_actual_status},
                idComponentType: ${id_component_type},
                isDelete: ${is_delete},
                idTypeAccess: ${id_type_access},
                commentchange: "${commentchange}",
                isStandard: ${is_standard}
            }) {
                uuid
                name
                comment
                idActualStatus
                isStandard
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    expect(registerComponent).toContainAllKeys([
      'uuid', 'name', 'comment', 'idActualStatus', 'isStandard', 'createdAt'
    ]);
    expect(registerComponent.uuid).toBeNonEmptyString();
    expect(registerComponent.name).toBe(name2);
    expect(registerComponent.comment).toBe(comment2);
    expect(registerComponent.idActualStatus).toBe(id_actual_status);
    expect(registerComponent.isStandard).toBe(is_standard);
    done();
  });

  it('/component/add - not supplier.', (done) => {
    agent
      .post('/component/add')
      .send({
        name: name2,  comment,  uuid_component_parent,  id_actual_status,
        id_component_type,  id_type_access,  is_standard: is_standard1
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/component/add body=%o', body);
        expect(body).toBe("You are not supplier.");
        done();
      });
  });

  it('/graphql:M add - not supplier.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponent( data: {
                name: "${name2}",
                uuidUser: "${uuid_user2}",
                comment: "${comment2}",
                uuidComponentParent: "${uuid_component_parent}",
                idActualStatus: ${id_actual_status},
                idComponentType: ${id_component_type},
                isDelete: ${is_delete},
                idTypeAccess: ${id_type_access},
                commentchange: "${commentchange}",
                isStandard: ${is_standard1}
            }) {
                uuid
                name
                comment
                idActualStatus
                isStandard
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql  - not supplier registerComponent=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("You are not supplier.");
    done();
  });
});
