const debug = require('debug')('cdbs-back:component_modification.test.js');
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
// const uuid = "";
const uuid_user = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuid_user2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
const uuid_component = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const uuid_component2 = "e925833e-f8d3-4ecb-bd67-5aa450f9f0ad";
const modification_name = "Head style U - Type A";
const modification_name2 = "Head style U - Type B";
const modification_name3 = "Head style U - Type C";
const id_name_cad = 2;
const comment = "bottom modification";
const uuid_modification_parent = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const id_actual_status = 1;

async function cleanupDb() {
  return global.knex.raw('DELETE FROM component_modification_list WHERE modification_name in (?,?,?)', [
    modification_name,
    modification_name2,
    modification_name3
  ]);
}
describe('modifications', () => {
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

  it('/modifications - OK', (done) => {
    agent
      .post('/modifications')
      .send({
        uuid_component, modification_name, id_name_cad, comment,
        uuid_modification_parent, id_actual_status
      })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/modifications body=%o', body);
        expect(body).toContainAllKeys(
          ["uuid", "uuid_component", "modification_name", "id_name_cad",
          "comment", "uuid_modification_parent", "id_actual_status", "created_at"]
        );
        expect(body.uuid).not.toBeNull();
        expect(body.uuid_component).toBe(uuid_component);
        expect(body.modification_name).toBe(modification_name);
        expect(body.uuid_modification_parent).toBe(uuid_modification_parent);
        expect(body.id_actual_status).toBe(id_actual_status);
        done();
      });
  });

  it('/modifications - Bad Request', (done) => {
    agent
      .post('/modifications')
      .send({
        uuid_component, modification_name, id_name_cad, comment,
        uuid_modification_parent, id_actual_status
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body, error, text, headers }) => {
        debug(
          '/modifications body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe(
          '\"Key (uuid_component, modification_name, uuid_modification_parent)=(a5953fd9-7393-4f1e-a899-06b5e159dbf1, Head style U - Type A, aba22d59-4f6c-44a4-9a37-2d38f0e577a8) already exists.\"'
        );
        expect(body).toBe('Key (uuid_component, modification_name, uuid_modification_parent)=(a5953fd9-7393-4f1e-a899-06b5e159dbf1, Head style U - Type A, aba22d59-4f6c-44a4-9a37-2d38f0e577a8) already exists.');
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponentModification( data: {
                uuidComponent: "${uuid_component}",
                modificationName: "${modification_name3}",
                idNameCad: ${id_name_cad},
                comment: "${comment}",
                uuidModificationParent: "${uuid_modification_parent}",
                idActualStatus: ${id_actual_status}
            }) {
                uuid
                uuidComponent
                modificationName
                idNameCad
                comment
                uuidModificationParent
                idActualStatus
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerComponentModification=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "uuidComponent", "modificationName", "idNameCad",
      "comment", "uuidModificationParent", "idActualStatus", "createdAt"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.uuidComponent).toBe(uuid_component);
    expect(registerComponentModification.modificationName).toBe(modification_name3);
    expect(registerComponentModification.idNameCad).toBe(id_name_cad);
    expect(registerComponentModification.comment).toBe(comment);
    expect(registerComponentModification.uuidModificationParent).toBe(uuid_modification_parent);
    expect(registerComponentModification.idActualStatus).toBe(id_actual_status);
    done();
  });

  it('/graphql:M register - Key already exists.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponentModification( data: {
                uuidComponent: "${uuid_component}",
                modificationName: "${modification_name3}",
                idNameCad: ${id_name_cad},
                comment: "${comment}",
                uuidModificationParent: "${uuid_modification_parent}",
                idActualStatus: ${id_actual_status}
            }) {
                uuid
                uuidComponent
                modificationName
                idNameCad
                comment
                uuidModificationParent
                idActualStatus
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      'Key (uuid_component, modification_name, uuid_modification_parent)=(a5953fd9-7393-4f1e-a899-06b5e159dbf1, Head style U - Type C, aba22d59-4f6c-44a4-9a37-2d38f0e577a8) already exists.'
    );
    done();
  });

  it('/graphql:Q List ComponentModification - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListComponentModification {
            componentModification {
                uuid
                uuidComponent
                modificationName
                createdAt
                idNameCad
                comment
                uuidModificationParent
                commentchange
                idActualStatus
                isDelete
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql all componentModification=%o', response1.body.data.componentModification);
    expect(response1.body.data.componentModification).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q componentModification with uuidComponent - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query componentModification {
            componentModification (uuidComponent: "${uuid_component}") {
                uuid
                uuidComponent
                modificationName
                createdAt
                idNameCad
                comment
                uuidModificationParent
                commentchange
                idActualStatus
                isDelete
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter componentModification=%o', response1.body.data.componentModification);
    expect(response1.body.data.componentModification).toBeNonEmptyArray();
    expect(response1.body.data.componentModification[0].uuidComponent).toBe(uuid_component);
    expect(response1.body.data.componentModification.pop().uuidComponent).toBe(uuid_component);
    done();
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

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponentModification( data: {
                uuidComponent: "${uuid_component2}",
                modificationName: "${modification_name2}",
                idNameCad: ${id_name_cad},
                comment: "${comment}",
                uuidModificationParent: "${uuid_modification_parent}",
                idActualStatus: ${id_actual_status}
            }) {
                uuid
                uuidComponent
                modificationName
                idNameCad
                comment
                uuidModificationParent
                idActualStatus
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql registerComponentModification=%o', body);
    const {
      data: { registerComponentModification },
    } = body;
    expect(registerComponentModification).toContainAllKeys(
      ["uuid", "uuidComponent", "modificationName", "idNameCad",
      "comment", "uuidModificationParent", "idActualStatus", "createdAt"]
    );
    expect(registerComponentModification.uuid).not.toBeNull();
    expect(registerComponentModification.uuidComponent).toBe(uuid_component2);
    expect(registerComponentModification.modificationName).toBe(modification_name2);
    expect(registerComponentModification.idNameCad).toBe(id_name_cad);
    expect(registerComponentModification.comment).toBe(comment);
    expect(registerComponentModification.uuidModificationParent).toBe(uuid_modification_parent);
    expect(registerComponentModification.idActualStatus).toBe(id_actual_status);
    done();
  });

  it('/modifications - modification not yours component', (done) => {
    agent
      .post('/modifications')
      .send({
        uuid_component, modification_name, id_name_cad, comment,
        uuid_modification_parent, id_actual_status
      })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/modifications body=%o', body);
        expect(body).toBe("Not found this component of you.");
        done();
      });
  });

  it('/graphql:M add - modification not yours component', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerComponentModification( data: {
                uuidComponent: "${uuid_component}",
                modificationName: "${modification_name3}",
                idNameCad: ${id_name_cad},
                comment: "${comment}",
                uuidModificationParent: "${uuid_modification_parent}",
                idActualStatus: ${id_actual_status}
            }) {
                uuid
                uuidComponent
                modificationName
                idNameCad
                comment
                uuidModificationParent
                idActualStatus
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql - Not yours component registerComponentModification=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("Not found this component of you.");
    done();
  });
});
