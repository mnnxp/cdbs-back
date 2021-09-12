const debug = require('debug')('cdbs-back:file.test.js');
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
const user_uuid_create = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const user_uuid_create2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
const component_uuid = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const component_uuid2 = "e925833e-f8d3-4ecb-bd67-5aa450f9f0ad";
const modification_uuid = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const parent_file_uuid = "bc1c2151-86d0-4656-9c9d-d016dd584297";
const parent_file_uuid2 = "3706d1a1-80ae-4367-be39-af7091373811";
const hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const hash2 = "ac42cb776fd8096feb871a3ae1bcb6ddfc992358210f9d4d79d072102458d4e8";
const filename = "file_one.3dm";
const filename_test = "Empty_File";
const filename2 = "file_two.3dm";
const id_ext = 2;
const value_ext = ".cdw";
const filesize = 52;
const filesize_test = 16;
const path_file = "/sholder/file/b06a8583-3d01-4739-8761-178ea8d4d27e";
const path_file2 = "/sholder/file/e8ae49d8-39e9-4011-ab4d-734efd7e1f1e";
const path_file_test = './tests/testfile';
const name_file_test = 'testfile';
const path_file_test2 = './tests/second_testfile.cdw';
const name_file_test2 = 'second_testfile.cdw';
const data_file_test = 'tests file data\n';

async function cleanupDb() {
  // TODO: not work deleting for test data in tables: file_ref, file_to_component, file_to_modification
  // return global.knex.raw('DELETE FROM file_ref WHERE filename in (?,?,?)', [
  //   filename,
  //   name_file_test,
  //   name_file_test2
  // ]);
}

describe('files', () => {
  beforeAll(async () => {
    return cleanupDb();
  });
  afterAll(async () => {
    return cleanupDb();
  });

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

  it('/files/users - OK', (done) => {
    agent
      .post('/files/users')
      .type('form')
      .attach('file', path_file_test)
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/files/users body=%o', body);
        expect(body[0].uuid).not.toBeNull();
        expect(body[0].filename).toBe(name_file_test);
        expect(body[0].filesize).toBe(filesize_test);
        expect(body[0].path_file).not.toBeNull();
        done();
      });
  });

  it('/files/users - Not correct', (done) => {
    agent
      .post('/files/users')
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/files/users body=%o', body);
        expect(body).toBe("Data not found.");
        done();
      });
  });

  it('/files/components/{uuid} - OK', (done) => {
    agent
      .post('/files/components/a5953fd9-7393-4f1e-a899-06b5e159dbf1')
      .type('form')
      .attach('file', path_file_test)
      .attach('file', path_file_test2)
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/files/components body=%o', body);
        expect(body[0].uuid).not.toBeNull();
        expect(body[0].filename).toBe(name_file_test);
        expect(body[0].filesize).toBe(filesize_test);
        expect(body[0].path_file).not.toBeNull();
        expect(body[1].uuid).not.toBeNull();
        expect(body[1].filename).toBe(name_file_test2);
        expect(body[1].filesize).not.toBeNull();
        expect(body[1].path_file).not.toBeNull();
        done();
      });
  });

  it('files/components/{uuid} - Not send file', (done) => {
    agent
      .post('/files/components/a5953fd9-7393-4f1e-a899-06b5e159dbf1')
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/files/components/{uuid} body=%o', body);
        expect(body).toBe("Data not found.");
        done();
      });
  });

  it('/files/modifications/{uuid} - OK', (done) => {
    agent
      .post('/files/modifications/aba22d59-4f6c-44a4-9a37-2d38f0e577a8')
      .type('form')
      .attach('file', path_file_test)
      .attach('file', path_file_test2)
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/files/modifications/{uuid} body=%o', body);
        expect(body[0].uuid).not.toBeNull();
        expect(body[0].filename).toBe(name_file_test);
        expect(body[0].filesize).toBe(filesize_test);
        expect(body[0].path_file).not.toBeNull();
        expect(body[1].uuid).not.toBeNull();
        expect(body[1].filename).toBe(name_file_test2);
        expect(body[1].filesize).not.toBeNull();
        expect(body[1].path_file).not.toBeNull();
        done();
      });
  });

  it('files/modifications/{uuid} - Not send file', (done) => {
    agent
      .post('/files/modifications/aba22d59-4f6c-44a4-9a37-2d38f0e577a8')
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/files/modifications/{uuid} body=%o', body);
        expect(body).toBe("Data not found.");
        done();
      });
  });

  it('/graphql:Q List files - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListFile {
            files {
                uuid
                parentFileUuid
                userUuidCreate
                filename
                idExt
                valueExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql all files=%o', response1.body.data.files);
    expect(response1.body.data.files).toBeNonEmptyArray();
    // expect(response1.body.data.files.pop().valueExt).toBe(value_ext);
    done();
  });

  it('/graphql:Q List files with userUuid - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserFile {
            files (userUuid: "${user_uuid_create}") {
                uuid
                parentFileUuid
                userUuidCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter files=%o', response1.body.data.files);
    expect(response1.body.data.files).toBeNonEmptyArray();
    expect(response1.body.data.files[0].userUuidCreate).toBe(user_uuid_create);
    expect(response1.body.data.files.pop().userUuidCreate).toBe(user_uuid_create);
    done();
  });

  it('/graphql:Q List files with componentUuid - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListComponentFile {
            files (componentUuid: "${component_uuid}") {
                uuid
                parentFileUuid
                userUuidCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter files=%o', response1.body.data.files);
    expect(response1.body.data.files).toBeNonEmptyArray();
    // expect(response1.body.data.files[0].uuid).toBe(component_uuid);
    // expect(response1.body.data.files.pop().uuid).toBe(component_uuid);
    done();
  });

  it('/graphql:Q List files with componentUuid - File not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query ListUserFile {
            files (componentUuid: "${uuid_fail}") {
                uuid
                parentFileUuid
                userUuidCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql componentUuid - File not found registerFile=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("File not found.");
    done();
  });

  it('/graphql:Q List files with componentUuidModification - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListModificationFile {
            files (componentUuidModification: "${modification_uuid}") {
                uuid
                parentFileUuid
                userUuidCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql filter files=%o', response1.body.data.files);
    expect(response1.body.data.files).toBeNonEmptyArray();
    // expect(response1.body.data.files[0].uuid).toBe(modification_uuid);
    // expect(response1.body.data.files.pop().uuid).toBe(modification_uuid);
    done();
  });

  it('/graphql:Q List files with componentUuidModification - File not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query ListModificationFile {
            files (componentUuidModification: "${uuid_fail}") {
                uuid
                parentFileUuid
                userUuidCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql componentUuidModification - File not found registerFile=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("File not found.");
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

  it('/files/components/{uuid} - attach to someone else\'s component.', (done) => {
    agent
      .post('/files/components/a5953fd9-7393-4f1e-a899-06b5e159dbf1')
      .type('form')
      .attach('file', path_file_test)
      .attach('file', path_file_test2)
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/files/components/{uuid} body=%o', body);
        expect(body).toBe("Not found data for this uuid.");
        done();
      });
  });

  it('/files/modifications/{uuid} - attach to someone else\'s modification.', (done) => {
    agent
      .post('/files/modifications/aba22d59-4f6c-44a4-9a37-2d38f0e577a8')
      .type('form')
      .attach('file', path_file_test)
      .attach('file', path_file_test2)
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/files/modifications/{uuid} body=%o', body);
        expect(body).toBe("Not found data for this uuid.");
        done();
      });
  });
});
