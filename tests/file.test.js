const debug = require('debug')('cdbs-back:file.test.js');
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
const uuid_user_create = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuid_user_create2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
const uuid_component = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const uuid_component2 = "e925833e-f8d3-4ecb-bd67-5aa450f9f0ad";
const uuid_modification = "aba22d59-4f6c-44a4-9a37-2d38f0e577a8";
const uuid_file_parent = "bc1c2151-86d0-4656-9c9d-d016dd584297";
const uuid_file_parent2 = "3706d1a1-80ae-4367-be39-af7091373811";
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
const path_file_test = '/home/mnnxp/Downloads/tmp/Empty_File';
const name_file_test = 'testfile';
const data_file_test = 'tests file data\n';

async function cleanupDb() {
  return global.knex.raw('DELETE FROM file_ref WHERE filename in (?,?,?)', [
    filename,
    filename2,
    filename_test
  ]);
}
describe('files', () => {
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

  it('/files/users - OK', (done) => {
    agent
      .post('/files/users')
      .append(path_file_test)
      // .attach(name_file_test, path_file_test)
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/files body=%o', body);
        expect(body).toContainAllKeys([
          "uuid", "filename", "filesize", "path_file"
        ]);
        expect(body.uuid).not.toBeNull();
        expect(body.filename).toBe(filename);
        expect(body.filesize).toBe(filesize);
        expect(body.path_file).toBe(path_file);
        done();
      });
  });

  // it('/files - OK', (done) => {
  //   agent
  //     .post('/files')
  //     .set("Content-Disposition", 'form-data; name=""; filename="Empty_File"'')
  //     .set("Content-Type", "multipart/form-data")
  //     // .field("name", "Tomato")
  //     // .field("userId", "5d921d306e96d70a28989127")
  //     .attach(
  //       "productImage",
  //       data_file_test
  //     )
  //     .expect(HttpStatus.OK)
  //     .then(({ body }) => {
  //       debug('/files body=%o', body);
  //       expect(body).toContainAllKeys([
  //         "uuid", "filename", "filesize", "path_file"
  //       ]);
  //       expect(body.uuid).not.toBeNull();
  //       expect(body.filename).toBe(filename_test);
  //       expect(body.filesize).toBe(filesize_test);
  //       expect(body.path_file).not.toBeNull();
  //       done();
  //     });
  // });

  it('/files - Not correct', (done) => {
    agent
      .post('/files')
      // .set("Content-Type", "multipart/form-data")
      // .field("name", "Tomato")
      // .field("userId", "5d921d306e96d70a28989127")
      // .attach(
      //   "productImage",
      //   "./testfile"
      // )
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body }) => {
        debug('/files body=%o', body);
        expect(body).toBe("Data not found. You okay?");
        done();
      });
  });

  // it('/graphql:M register - OK', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .send({
  //       query: `mutation  {
  //           registerFile( data: {
  //               uuidFileParent: "${uuid_file_parent}",
  //               uuidUserCreate: "${uuid_user_create}",
  //               hash: "${hash}",
  //               filename: "${filename}",
  //               idExt: ${id_ext},
  //               filesize: ${filesize},
  //               pathFile: "${path_file}"
  //           }) {
  //               uuid
  //               filename
  //               filesize
  //               pathFile
  //           }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK);
  //   debug('/graphql registerFile=%o', body);
  //   const {
  //     data: { registerFile },
  //   } = body;
  //   expect(registerFile).toContainAllKeys([
  //     "uuid", "filename", "filesize", "pathFile"
  //   ]);
  //   expect(registerFile.uuid).not.toBeNull();
  //   expect(registerFile.filename).toBe(filename);
  //   expect(registerFile.filesize).toBe(filesize);
  //   expect(registerFile.pathFile).toBe(path_file);
  //   done();
  // });

  // it('/graphql:M register - Not correct UUID', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .send({
  //       query: `mutation  {
  //           registerFile( data: {
  //               uuidFileParent: "${uuid_file_parent2}",
  //               uuidUserCreate: "${uuid_user_create2}",
  //               hash: "${hash2}",
  //               filename: "${filename2}",
  //               idExt: ${id_ext},
  //               filesize: ${filesize},
  //               pathFile: "${path_file2}"
  //           }) {
  //               uuid
  //               filename
  //               filesize
  //               pathFile
  //           }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK);
  //   debug('/graphql  - Not correct UUID registerFile=%o', body);
  //   const { errors, data } = body;
  //   expect(data).toBeNull();
  //   expect(errors[0].message).toBe("Uuid not correct.");
  //   done();
  // });

  it('/graphql:Q List files - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListFile {
            files {
                uuid
                uuidFileParent
                uuidUserCreate
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

  it('/graphql:Q List files with uuidUserCreateSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUserFile {
            files (uuidUserCreateSearch: "${uuid_user_create}") {
                uuid
                uuidFileParent
                uuidUserCreate
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
    expect(response1.body.data.files[0].uuidUserCreate).toBe(uuid_user_create);
    expect(response1.body.data.files.pop().uuidUserCreate).toBe(uuid_user_create);
    done();
  });

  it('/graphql:Q List files with uuidComponentSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListComponentFile {
            files (uuidComponentSearch: "${uuid_component}") {
                uuid
                uuidFileParent
                uuidUserCreate
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
    expect(response1.body.data.files[0].uuid).toBe(uuid_file_parent);
    expect(response1.body.data.files.pop().uuid).toBe(uuid_file_parent);
    done();
  });

  it('/graphql:Q List files with uuidComponentSearch - File not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query ListUserFile {
            files (uuidComponentSearch: "${uuid_fail}") {
                uuid
                uuidFileParent
                uuidUserCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql uuidComponentSearch - File not found registerFile=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("File not found.");
    done();
  });

  it('/graphql:Q List files with uuidComponentModificationSearch - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListModificationFile {
            files (uuidComponentModificationSearch: "${uuid_modification}") {
                uuid
                uuidFileParent
                uuidUserCreate
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
    expect(response1.body.data.files[0].uuid).toBe(uuid_file_parent);
    expect(response1.body.data.files.pop().uuid).toBe(uuid_file_parent);
    done();
  });

  it('/graphql:Q List files with uuidComponentModificationSearch - File not found', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query ListModificationFile {
            files (uuidComponentModificationSearch: "${uuid_fail}") {
                uuid
                uuidFileParent
                uuidUserCreate
                filename
                idExt
                createdAt
                filesize
                pathFile
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql uuidComponentModificationSearch - File not found registerFile=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("File not found.");
    done();
  });

  // it('/users/logout - OK', (done) => {
  //   agent.get('/users/logout').expect(HttpStatus.OK, done);
  // });
  //
  // it('/users/login - OK is not supplier', (done) => {
  //   agent
  //     .post('/users/login')
  //     .send({ nickname: nickname2, password: password2 })
  //     .expect(HttpStatus.OK)
  //     .then(({ body, headers }) => {
  //       expect(headers['set-cookie'][0]).toBeNonEmptyString();
  //       expect(body).toContainAllKeys(['nickname', 'is_supplier', 'uuid']);
  //       expect(body.nickname).toBe(nickname2);
  //       expect(body.is_supplier).toBe(0);
  //       expect(body.uuid).toBeNonEmptyString();
  //       done();
  //     });
  // });

  // it('/files - not supplier.', (done) => {
  //   agent
  //     .post('/file')
  //     .send({
  //       uuid_file_parent, hash, filename, id_ext, filesize, path_file
  //     })
  //     .expect(HttpStatus.BAD_REQUEST)
  //     .then(({ body }) => {
  //       debug('/files body=%o', body);
  //       expect(body).toBe("You are not supplier.");
  //       done();
  //     });
  // });

  // it('/graphql:M register - not supplier.', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .send({
  //       query: `mutation  {
  //           registerFile( data: {
  //               uuidFileParent: "${uuid_file_parent2}",
  //               uuidUserCreate: "${uuid_user_create2}",
  //               hash: "${hash2}",
  //               filename: "${filename2}",
  //               idExt: ${id_ext},
  //               filesize: ${filesize},
  //               pathFile: "${path_file2}"
  //           }) {
  //               uuid
  //               filename
  //               filesize
  //               pathFile
  //           }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK);
  //   debug('/graphql  - not supplier registerFile=%o', body);
  //   const { errors, data } = body;
  //   expect(data).toBeNull();
  //   expect(errors[0].message).toBe("You are not supplier.");
  //   done();
  // });
});
