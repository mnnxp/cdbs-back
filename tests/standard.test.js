const debug = require('debug')('cdbs-back:standard.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

// data for user
const username = "baromi";
const username2 = "simaco";
const password = "password";

const uuidFail = "aba22d59-4f6c-24a4-9a37-2d38f0e577a8";
const uuidUser = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const uuidUser2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
var uuidUserSecond = "";

// data for standard
const uuidStandardParent = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const classifierStandard = "GOST-2012-Test";
const nameStandard = "GOST 2012 Test standard";
const descriptionStandard = "Test GOST standard";
const specifiedTolerance = "C";
const technicalCommittee = "GOST";
const publicationAt = "2021-07-31T00:00:00";
const idTypeAccess3 = 3;
const idTypeAccess1 = 1;
const idStandardStatus = 1;
const idRegion = 5;
var uuidStandardFirst = "";
var uuidStandardSecond = "";

const standardFullDataQuery = ` \
uuid \
uuidStandardParent \
classifier \
name \
description \
specifiedTolerance \
technicalCommittee \
publicationAt \
imageFile { \
  uuid \
  filename \
  filesize \
  pathFile \
} \
ownerUser { \
  uuid \
  username \
  imageFile { \
    uuid \
    filename \
    filesize \
    pathFile \
  } \
} \
ownerCompany { \
  uuid \
  shortname \
  inn \
  description \
  imageFile { \
    uuid \
    filename \
    filesize \
    pathFile \
  } \
  region { \
    idRegion \
    idLang \
    region \
  } \
  companyType { \
    idCompanyType \
    idLang \
    name \
    shortname \
  } \
  isSupplier \
  isFollowed \
  updatedAt \
} \
idTypeAccess \
standardStatus { \
  idStandardStatus \
  idLang \
  name \
} \
region { \
  idRegion \
  idLang \
  region \
} \
isDelete \
createdAt \
updatedAt \
standardFiles { \
  uuid \
  uuidFileParent \
  uuidUser \
  filename \
  contentType \
  idExt \
  filesize \
  pathFile \
  createdAt \
  updatedAt \
} \
standardSpecs { \
  spec { \
    idSpec \
    idLang \
    spec \
  } \
	uuidStandard \
} \
standardKeywords { \
  id \
	keyword \
} \
subscribers \
isFollowed \
updatedAt \
isFollowed \
`;

const standardsListQuery = ` \
uuid \
classifier \
name \
description \
specifiedTolerance \
publicationAt \
ownerCompany { \
  uuid \
  shortname \
  inn \
  description \
  imageFile { \
    uuid \
    filename \
    filesize \
    pathFile \
  } \
  region { \
    idRegion \
    idLang \
    region \
  } \
  companyType { \
    idCompanyType \
    idLang \
    name \
    shortname \
  } \
  isSupplier \
  isFollowed \
  updatedAt \
} \
standardStatus { \
  idStandardStatus \
  idLang \
  name \
} \
updatedAt \
isFollowed  \
`;

// data for company
const orgname = "orgname supplier of the test";
const orgname2 = "orgnametest not supplier of the test";
const shortname = "shortnametest";
const inn = "5555555";
const phoneCompany = "7777777777";
const email = "testcompany@testemail.ru";
const description = "test company";
const addressCompany = "China";
const siteUrl = "example.test";
const timeZone = "Europe/Moscow";
const uuidImageFile = "3706d1a1-80ae-4367-be39-af7091373811";
const idRegionCompany = 5;
const idCompanyType = 2;
const uuidCompanyBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";
var uuidCompanyNoSupplier = "";
var uuidCompanySupplier = "";

// data for represent
const idRegionRepresentation = 15;
const idRepresentationType = 1;
const nameRepresentationFirst = "test first additional office";
const nameRepresentationSecond = "test second additional office";
const addressRepresentation = "Fake str, Fantom";
const phoneRepresentation = "+743874487556";
const uuidFake = "2cd385e1-8f7e-4908-8235-dfe42938b888";
const uuidRepresentArray = [];
var uuidCompanyFirst = "";
var uuidRepresentFirst = "";
var uuidRepresentDelete = "";

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname in (?,?);', [
    orgname,
    orgname2,
  ]);
}

async function cleanupStandardDb() {
  return global.knex.raw('DELETE FROM standard_ref WHERE name in (?)', [
    nameStandard,
  ]);
}

async function cleanupCompanyRepresentDb() {
  return global.knex.raw('DELETE FROM company_represent_ref WHERE name in (?,?)', [
    nameRepresentationFirst,
    nameRepresentationSecond,
  ]);
}

async function cleanupTokenDb() {
  return global.knex.raw('DELETE FROM user_token_ref');
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username in (?,?)', [
    username,
    username2,
  ]);
}

describe('company', () => {
  beforeAll(() => {
    cleanupCompanyRepresentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
    cleanupTokenDb();
    cleanupUserDb();
    return;
  });
  afterAll(() => {
    cleanupCompanyRepresentDb();
    cleanupStandardDb();
    cleanupCompanyDb();
    cleanupTokenDb();
    cleanupUserDb();
    return;
  });

  const agent = request.agent(url);

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "testemail@mail.ru",
                firstname: "test_firstname",
                lastname: "test_lastname",
                secondname: "test_secondname",
                username: "${username}",
                password: "${password}",
                phone: "test_phone",
                description: "test_description",
                address: "test_address",
                position: "test_position",
                timeZone: "Europe/Moscow",
                uuidImageFile: "bc1c2151-86d0-4656-9c9d-d016dd584297",
                idRegion: 1,
                idProgram: 1,
            }) {
                uuid
                idProgram
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'idProgram', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.idProgram).toBe(1);
    expect(registerUser.username).toBe(username);
    done();
  });

  it('/login - OK', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/login body=%o', body);
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenFirst = body.bearer;
        done();
      });
  });

  it('/graphql:M register second - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser( data: {
                email: "testemail@mail.ru",
                firstname: "test_firstname",
                lastname: "test_lastname",
                secondname: "test_secondname",
                username: "${username2}",
                password: "${password}",
                phone: "test_phone",
                description: "test_description",
                address: "test_address",
                position: "test_position",
                timeZone: "Europe/Moscow",
                uuidImageFile: "bc1c2151-86d0-4656-9c9d-d016dd584297",
                idRegion: 1,
                idProgram: 5,
            }) {
                uuid
                idProgram
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    expect(registerUser).toContainAllKeys(['uuid', 'idProgram', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.idProgram).toBe(5);
    expect(registerUser.username).toBe(username2);
    uuidUserSecond = registerUser.uuid;
    done();
  });

  it('/login second - OK', (done) => {
    agent
      .post('/login')
      .send({ "user": {
            "username": username2,
            "password": password,
          }
        })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/login body=%o', body);
        expect(body.bearer).toBeNonEmptyString();
        authorizationTokenSecond = body.bearer;
        done();
      });
  });

  it('/graphql:M registerCompany - OK Supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation newCompany {
         registerCompany( data: {
            orgname: "${orgname}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: "${timeZone}",
            idRegion: ${idRegionCompany},
            idCompanyType: ${idCompanyType}
          }) {
            uuid
            shortname
            isSupplier
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany.uuid).toBeNonEmptyString();
    expect(registerCompany.shortname).toBe(shortname);
    expect(registerCompany.isSupplier).toBe(false);
    uuidCompanySupplier = registerCompany.uuid;
    done();
    // change supplier status on 1
    await global.knex.raw('UPDATE company_ref SET is_supplier=? WHERE orgname=?', [
      't',
      orgname,
    ]);
  });

  it('/graphql:M registerCompany - OK NoSupplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation newCompany {
         registerCompany( data: {
            orgname: "${orgname2}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: "${timeZone}",
            idRegion: ${idRegionCompany},
            idCompanyType: ${idCompanyType}
          }) {
            uuid
            shortname
            isSupplier
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany.uuid).toBeNonEmptyString();
    expect(registerCompany.shortname).toBe(shortname);
    expect(registerCompany.isSupplier).toBe(false);
    uuidCompanyNoSupplier = registerCompany.uuid;
    done();
  });

  it('/graphql:M registerStandard - BadRequest without token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            uuidStandardParent: "${uuidStandardParent}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            uuidCompany: "${uuidCompanySupplier}",
            idTypeAccess: ${idTypeAccess3},
            idStandardStatus: ${idStandardStatus},
            idRegion: ${idRegion}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            idStandardStatus
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found.");
    done();
  });

  it('/graphql:M registerStandard - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            uuidStandardParent: "${uuidStandardParent}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            uuidCompany: "${uuidCompanySupplier}",
            idTypeAccess: ${idTypeAccess3},
            idStandardStatus: ${idStandardStatus},
            idRegion: ${idRegion}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            idStandardStatus
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard.uuid).toBeNonEmptyString();
    expect(registerStandard.name).toBe(nameStandard);
    uuidStandardFirst = registerStandard.uuid;
    done();
  });

  it('/graphql:M registerStandard - OK not set parent', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            uuidCompany: "${uuidCompanySupplier}",
            idTypeAccess: ${idTypeAccess1},
            idStandardStatus: ${idStandardStatus},
            idRegion: ${idRegion}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            idStandardStatus
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard.uuid).toBeNonEmptyString();
    expect(registerStandard.name).toBe(nameStandard);
    done();
  });

  it('/graphql:M registerStandard - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            uuidStandardParent: "${uuidStandardParent}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            uuidCompany: "${uuidCompanySupplier}",
            idTypeAccess: ${idTypeAccess1},
            idStandardStatus: ${idStandardStatus},
            idRegion: ${idRegion}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            idStandardStatus
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerStandard=%o', body);
    const {
      data: { registerStandard },
    } = body;
    expect(registerStandard.uuid).toBeNonEmptyString();
    expect(registerStandard.name).toBe(nameStandard);
    uuidStandardSecond = registerStandard.uuid;
    done();
  });

  it('/graphql:M registerStandard - BadRequest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            uuidStandardParent: "${uuidStandardParent}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            uuidCompany: "${uuidCompanySupplier}",
            idTypeAccess: ${idTypeAccess3},
            idStandardStatus: ${idStandardStatus},
            idRegion: ${idRegion}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            idStandardStatus
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: You not have access.");
    done();
  });

  it('/graphql:M registerStandard - BadRequest not supplier', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation standardQuery {
          registerStandard( data: {
            uuidStandardParent: "${uuidStandardParent}",
            classifier: "${classifierStandard}",
            name: "${nameStandard}",
            description: "${descriptionStandard}",
            specifiedTolerance: "${specifiedTolerance}",
            technicalCommittee: "${technicalCommittee}",
            publicationAt: "${publicationAt}",
            uuidCompany: "${uuidCompanyNoSupplier}",
            idTypeAccess: ${idTypeAccess3},
            idStandardStatus: ${idStandardStatus},
            idRegion: ${idRegion}
          }) {
            uuid
            classifier
            name
            specifiedTolerance
            technicalCommittee
            publicationAt
            idStandardStatus
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: The company is not supplier.");
    done();
  });

  it('/graphql:Q standard - BadRequest without token', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `query selectStandardQuery{
          standards (standardsUuids: "${uuidStandardSecond}") {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: Token not found.");
    done();
  });

  // it('/graphql:Q standard - OK Select with fake uuid', async (done) => {
  //   const { body } = await agent
  //     .post('/graphql')
  //     .set(
  //       'Authorization',
  //       `Bearer ${authorizationTokenFirst}`
  //     )
  //     .send({
  //       query: `query selectStandardQuery{
  //         standard (standardUuid: "${uuidFake}") {
  //           ${standardFullDataQuery}
  //         }
  //       }`,
  //     })
  //     .expect(HttpStatus.OK)
  //   debug('/graphql - body=%o', body);
  //   const { errors, data } = body;
  //   expect(data).toBeNull();
  //   expect(errors[0].message).toBe("BadRequest: You not have access.");
  //   done();
  // });

  it('/graphql:Q standard - OK ShowUserShort', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `query selectStandardQuery{
          standard (standardUuid: "${uuidStandardFirst}") {
            ${standardFullDataQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standard },
    } = body;
    expect(standard.uuid).toBe(uuidStandardFirst);
    expect(standard.classifier).toBe(classifierStandard);
    done();
  });

  it('/graphql:Q standard - OK Select with uuid (public access)', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (standardsUuids: ["${uuidStandardFirst}"]) {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standards },
    } = body;
    expect(standards[0].uuid).toBe(uuidStandardFirst);
    expect(standards[0].classifier).toBe(classifierStandard);
    done();
  });

  it('/graphql:Q standard - BadReuest no access', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (standardsUuids: "${uuidStandardSecond}") {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql - body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe("BadRequest: You not have access.");
    done();
  });

  it('/graphql:Q standard - OK Select with uuid (private access)', async (done) => {
    // add access to the object for the user
    await global.knex.raw('INSERT INTO user_access_to_standard (uuid_standard, uuid_user, id_type_access, is_enabled, is_delete, created_at, updated_at) VALUES (?, ?, 1, true, false, now(), now());', [
      uuidStandardSecond,
      uuidUserSecond,
    ]);
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `query selectStandardQuery{
          standards (standardsUuids: "${uuidStandardSecond}") {
            ${standardsListQuery}
          }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql body=%o', body);
    const {
      data: { standards },
    } = body;
    expect(standards[0].uuid).toBe(uuidStandardSecond);
    expect(standards[0].classifier).toBe(classifierStandard);
    done();
  });
});
