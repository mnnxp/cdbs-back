const HttpStatus = require('http-status-codes');
const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;
const api = `${url}/graphql`;
const fetch = require('node-fetch');
const debug = require('debug')('cdbs-back:service.test.js');
const request = require('supertest');

jest.setTimeout(1300);

const regexUuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/; // UUID
const ToObject = Object.freeze({
  COMPANY: 'COMPANY',
  COMPONENT: 'COMPONENT',
  SERVICE: 'SERVICE',
  BADOPTION: 'BADOPTION',
});
var testDiscussionCommentData = {
  objectDiscussion: {
    objectUuid: null,
    toObject: ToObject.COMPANY,
  },
  discussionUuid: null,
  parentCommentUuid: null,
  messageContent: 'Test comment message',
};
var companyUuidSupplier = "";
var componentUuidNoStandard = "";
var serviceUuidFirst = "";
var discussionCompanyUuidFirt = '';
var discussionComponentUuidFirt = '';
var discussionServiceUuidFirt = '';
var rootDiscussCommentCompanyUuid = '';
var rootDiscussCommentServiceUuid = '';
var rootDiscussCommentComponentUuid = '';
var testPreventDuplicateCommentUuid = '';
var test1RepliesCommentFirstUuid = '';
var test0RepliesCommentSecondUuid = '';
var test7RepliesCommentThreeUuid = '';
var testRepliesForEachCommentUuid = [];

// data for user
const username = "baromi";
const username2 = "simaco";
const password = "password";
const uuidFail = "aba22d59-4f6c-24a4-9a37-2d38f0e577a8";
const userUuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b";
const userUuid2 = "68b8281a-d19c-4d4b-88eb-6fd4a2afde1b";
var authorizationUserFirst = '';
var authorizationUserSecond = '';
var authorizationTokenFirst = '';
var authorizationTokenSecond = '';
var invalidAuthorizationToken = 'oO9mu7KdINx2D5ZY';

const sqlInjections = [
  `Robert' /* */; DROP TABLE user_token_ref; --`,
  `Robert' UNION SELECT * FROM user_ref LIMIT 1; --`,
  `Robert' OR 1 = 1; --`,
  `Robert' AND 1 = 1; --`,
  `Robert' /* */; DROP TABLE Students; --`,
  `Robert' (SELECT * FROM Users WHERE id = 1); --`,
  `Robert' (SELECT concat('Hello, ', 'World')); --`,
  `Robert' %_with\nnewline\rreturn\ttab\0null; --`,
];

// data for service
const nameService = "GOST 2012 Test service";
const descriptionService = "Test GOST service";
const serviceStatusId = 1;
const regionId = 5;
const nameService2 = "GOST 2012 Test service 2222";
const descriptionService2 = "Test GOST service 2222";
const serviceStatusId2 =  3;
const regionId2 = 5;


const discussionFieldResponse = ` \
uuid \
title \
isPinned \
lastActivityAt \
createdAt \
repliesCount \
`;

const authorFieldResponse = ` \
author { \
  uuid \
  username \
  imageFile { \
    uuid \
    filename \
    filesize \
    downloadUrl \
  } \
} \
`;

const discussionCommentFieldResponse = ` \
uuid \
discussionUuid \
parentCommentUuid \
${authorFieldResponse} \
messageContent \
isEdited \
isHidden \
createdAt \
updatedAt \
repliesCount \
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
const imageFileUuid = "3706d1a1-80ae-4367-be39-af7091373811";
const regionIdCompany = 5;
const companyTypeId = 2;
const companyUuidBase = "2cd385e1-8f7e-4908-8235-dfe42938b46d";

// data for represent
const regionIdRepresentation = 15;
const representationTypeId = 1;
const nameRepresentationFirst = "test first additional office";
const nameRepresentationSecond = "test second additional office";
const addressRepresentation = "Fake str, Fantom";
const phoneRepresentation = "+743874487556";
const uuidFake = "2cd385e1-8f7e-4908-8235-dfe42938b888";
const uuidRepresentArray = [];
var companyUuidFirst = "";
var uuidRepresentFirst = "";
var uuidRepresentDelete = "";

// role company member
var langId = 1;
var nameRole = "test role";
var newRoleId = 0;

// service type access
const typeAccessId3 = 3;
const typeAccessId2 = 2;
const typeAccessId1 = 1;
const typesAccessIds23 = [2,3];

// service keywords
const keywordIdsOk = [1,3,5];
const keywordIdsDup = [1,2,3,4,5];
const keywordNames = ["asd2","asd3","asd4"];
const keywordNamesDup = ["asd2","asd3","asd4","asd5","asd6"];
const keywordNamesBad = ["asd11","asd12345678","asd12"];

// service specs
const specIdsOk = [10,30,55];
const specIdsDup = [10,22,30,44,55];
const specIdsDel = [10,55];
const idErr = 0;

// service files
const filename1 = "file-test-name 1.pdf";
const filename2 = "file-test-name 2.pdf";
const filename3 = "file-test-name 3.pdf";
const filename4 = "file-test-name 4.pdf";
const filename5 = "file-test-name 5.pdf";

const descriptionServiceFileTest = "test desctiption for service";
const filenameServiceFileTest = "second name file for service.pdf";
const badFilenameServiceFileTest = "name* file/ service.pdf";
const goodFilenameServiceFileTest = "name file service.pdf";
const badFilenameServiceFaviconTest = "no image file.pdf";
const goodFilenameServiceFaviconTest = "image file.png";

var fileServiceFileTestUuid = "";
var fileServiceFileTestUuid2 = "";
var seconRevFileFileTestUuid = "";
var seconRevFileFileTestUuid2 = "";
var threeRevFileFileTestUuid2 = "";
var fourthRevFileFileTestUuid2 = "";
var fifthRevFileFileTestUuid2 = "";
var sixthRevFileFileTestUuid2 = "";
var seventhRevFileFileTestUuid2 = "";

// data for standard
const parentStandardUuid = "303ec2aa-2066-42e3-93fb-de4fb9344bcb";
const nameStandard = "GOST 2012 Test standard";
const descriptionStandard = "Test GOST standard";
const publicationAt = "2021-07-31T00:00:00";
const standardStatusId = 1;
var standardUuidFirst = "";
var standardUuidSecond = "";

const descriptionSupplier = "description for supplier component";
const descriptionSupplierNew = "new description supplier";

// data for component
const componentNamePut = "componentNamePutUpdate";
const descriptionNamePut = "descriptionNamePutUpdate";
const componentTypeIdPut = 2;
const actualStatusIdPut = 1;
var componentActualStatusesId1 = "";
var componentActualStatusesId2 = "";
var componentTypesId1 = "";

const parentComponentUuid = "a5953fd9-7393-4f1e-a899-06b5e159dbf1";
const nameComponent = "M Series Geared Motor";
const nameComponent2 = "X Custom Geared Motor";
const descriptionComponent = "graphqlcomment for component";
const typeAccessIdComponent = 3;
const typeAccessIdComponentPrivate = 1;
const componentTypeId = 1;
const actualStatusIdComponent = 1;
const isBaseComponent = true;
const isBaseComponent0 = false;
const subscribersCount = 1;
const licenseIdOk = 1;
const licenseIdErr = 2;
const filename0 = "file-test-name 0.pdf";
const modificationNameM1 = "1/16/20-SS-V";
const descriptionM1 = "Dest 1/ss-v";
const actualStatusIdM1 = 1;
const modificationNameM2 = "2/16/20-TT-V";
const descriptionM2 = "Dest 2/tt-v";
const actualStatusIdM2 = 2;
const modificationNameM3 = "3/16/20-D-V";
const descriptionM3 = "Dest 3/d-v";
const actualStatusIdM3 = 3;
const modificationNameM4 = "4/16/20-D-V";
const descriptionM4 = "Dest 4/d-v";
const actualStatusIdM4 = 4;
const badFilenameComponentFaviconTest = "no image file.pdf";
const goodFilenameComponentFaviconTest = "image file.png";

async function cleanupCompanyDb() {
  return global.knex.raw('DELETE FROM company_ref WHERE orgname IN (?,?)', [
    orgname,
    orgname2,
  ]);
}

async function cleanupDiscussiondDb() {
  return global.knex.raw('DELETE FROM discussion_ref WHERE discussion_ref.uuid IN (SELECT dr.uuid FROM discussion_ref AS dr LEFT JOIN discus_to_company AS dtc ON dr.uuid  = dtc.discussion_uuid LEFT JOIN discus_to_component AS dtc2 ON dr.uuid = dtc2.discussion_uuid LEFT JOIN discus_to_service AS dts ON dr.uuid  = dts.discussion_uuid WHERE dtc.discussion_uuid IS NULL AND dtc2.discussion_uuid IS NULL AND dts.discussion_uuid IS NULL)');
}

async function cleanupComponentDb() {
  return global.knex.raw('DELETE FROM component_ref WHERE name in (?,?)', [
    nameComponent,
    nameComponent2,
  ]);
}

async function cleanupServiceDb() {
  return global.knex.raw('DELETE FROM service_ref WHERE name in (?)', [
    nameService,
  ]);
}

async function cleanupCompanyRepresentDb() {
  return global.knex.raw('DELETE FROM company_represent_ref WHERE name in (?,?)', [
    nameRepresentationFirst,
    nameRepresentationSecond,
  ]);
}

async function cleanupUserDb() {
  return global.knex.raw('DELETE FROM user_ref WHERE username in (?,?)', [
    username,
    username2,
  ]);
}

describe('discussion', () => {
  beforeAll(() => {
    cleanupComponentDb();
    cleanupCompanyRepresentDb();
    cleanupServiceDb();
    cleanupCompanyDb();
    cleanupUserDb();
    cleanupDiscussiondDb();
    return;
  });
  afterAll(() => {
    cleanupComponentDb();
    cleanupCompanyRepresentDb();
    cleanupServiceDb();
    cleanupCompanyDb();
    cleanupUserDb();
    cleanupDiscussiondDb();
    return;
  });

  const agent = request.agent(url);

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            registerUser(args: {
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
                regionId: 1,
                programId: 1,
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    authorizationUserFirst = registerUser.uuid;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(1);
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
        debug('/login headers=%o', headers);
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
            registerUser(args: {
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
                regionId: 1,
                programId: 5,
            }) {
                uuid
                programId
                username
            }
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerUser=%o', body);
    const {
      data: { registerUser },
    } = body;
    authorizationUserSecond = registerUser.uuid;
    expect(registerUser).toContainAllKeys(['uuid', 'programId', 'username']);
    expect(registerUser.uuid).toBeNonEmptyString();
    expect(registerUser.programId).toBe(5);
    expect(registerUser.username).toBe(username2);
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
        debug('/login headers=%o', headers);
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
         registerCompany(args: {
            orgname: "${orgname2}",
            shortname: "${shortname}",
            inn: "${inn}",
            phone: "${phoneCompany}",
            email: "${email}",
            description: "${description}",
            address: "${addressCompany}"
            siteUrl: "${siteUrl}",
            timeZone: "${timeZone}",
            regionId: ${regionIdCompany},
            companyTypeId: ${companyTypeId},
            typeAccessId: ${typeAccessId3}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerCompany=%o', body);
    const {
      data: { registerCompany },
    } = body;
    expect(registerCompany).toBeNonEmptyString();
    companyUuidSupplier = registerCompany;
    // change supplier status on 1
    await global.knex.raw('UPDATE company_ref SET is_supplier=? WHERE uuid=?', [
      't',
      companyUuidSupplier,
    ]);
    done();
  });

  it('/graphql:M serviceRequest - OK user is owner company', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenFirst}`
      )
      .send({
        query: `mutation serviceQuery {
          serviceRequest(args: {
            name: "${nameService}",
            description: "${descriptionService}",
            companyUuid: "${companyUuidSupplier}",
            regionId: ${regionId}
          })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql serviceRequest=%o', body);
    // expect(body).toBe(0);
    const {
      data: { serviceRequest },
    } = body;
    expect(serviceRequest).toBeNonEmptyString();
    serviceUuidFirst = serviceRequest;
    done();
  });

  it('/graphql:M registerComponent - OK not standard', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${authorizationTokenSecond}`
      )
      .send({
        query: `mutation  {
            registerComponent(args: {
                name: "${nameComponent2}",
                description: "${descriptionComponent}",
                typeAccessId: ${typeAccessIdComponentPrivate},
                componentTypeId: ${typeAccessId1},
                actualStatusId: ${actualStatusIdComponent},
                isBase: ${isBaseComponent0}
            })
        }`,
      })
      .expect(HttpStatus.OK)
    debug('/graphql registerComponent=%o', body);
    const {
      data: { registerComponent },
    } = body;
    componentUuidNoStandard = registerComponent;
    expect(registerComponent).toBeNonEmptyString();
    done();
  });

  // Test 1: Successful registration of a comment for a company
  it('should register a new discussion comment to company successfully', async () => {
    const validDiscussionCommentData = {...testDiscussionCommentData };
    validDiscussionCommentData.objectDiscussion.objectUuid = companyUuidSupplier;
    validDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPANY;
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: validDiscussionCommentData },
      }),
    });

    const jsonData = await response.json();
    expect(jsonData.data.registerDiscussionComment).toMatch(regexUuid);
    rootDiscussCommentCompanyUuid = jsonData.data.registerDiscussionComment;
  });

  // Test 2: Successful request for discussions without sorting specified in the request
  it('should return discussions without sorting', async () => {
    const validArgs = {
      objectUuid: companyUuidSupplier,
      toObject: ToObject.COMPANY,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: null,
          paginate: null,
        },
      }),
    });
    // const jsonData = await response.json();
    const { data: { discussions } } = await response.json();
    // expect(jsonData).toBe(0);
    expect(discussions).toBeDefined(); // Make sure discussions are defined
    expect(discussions).toBeInstanceOf(Array); // Make sure the discussions are an array
    expect(discussions.length).toBe(1); // Make sure the discussions are an array of a single discussion
    discussionCompanyUuidFirt = discussions[0].uuid; // Save the UUID of the first discussion
  });

  // Test 3: Successful registration of a comment for a service
  it('should register a new discussion comment to service successfully', async () => {
    const validDiscussionCommentData = {...testDiscussionCommentData };
    validDiscussionCommentData.objectDiscussion.objectUuid = serviceUuidFirst;
    validDiscussionCommentData.objectDiscussion.toObject = ToObject.SERVICE;
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: validDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.data.registerDiscussionComment).toMatch(regexUuid);
    rootDiscussCommentServiceUuid = jsonData.data.registerDiscussionComment;
  });

  // Test 4: Successful registration of a comment for a component
  it('should register a new discussion comment to component successfully', async () => {
    const validDiscussionCommentData = {...testDiscussionCommentData };
    validDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    validDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: validDiscussionCommentData },
      }),
    });

    const jsonData = await response.json();
    expect(jsonData.data.registerDiscussionComment).toMatch(regexUuid);
    rootDiscussCommentComponentUuid = jsonData.data.registerDiscussionComment;
  });

  // Test 5: Successful registration of a comment for a component (the comment is not duplicated, since the previous one is without parentCommentUuid)
  it('should register a new discussion comment to component successfully (with parentCommentUuid)', async () => {
    const validDiscussionCommentData = {...testDiscussionCommentData };
    validDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    validDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;
    validDiscussionCommentData.parentCommentUuid = rootDiscussCommentComponentUuid;
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: validDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.data.registerDiscussionComment).toMatch(regexUuid);
  });

  // Test 6: Authorization error without authorization token
  it('should return an error for unauthorized access (without token)', async () => {
    const invalidDiscussionCommentData = {...testDiscussionCommentData };
    invalidDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    invalidDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;
    invalidDiscussionCommentData.messageContent = 'a'.repeat(20);
    const response = await fetch(api, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: invalidDiscussionCommentData },
      }),
    });

    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Token not found');
  });

  // Test 7: Authorization error (invalid token)
  it('should return an error for unauthorized access', async () => {
    const invalidDiscussionCommentData = {...testDiscussionCommentData };
    invalidDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    invalidDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;
    invalidDiscussionCommentData.messageContent = 'a'.repeat(20);
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${invalidAuthorizationToken}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: invalidDiscussionCommentData },
      }),
    });

    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('Unauthorized');
  });

  // Test 8: Invalid input - no objectDiscussion specified
  it('should return an error objectDiscussion is not set', async () => {
    const intestDiscussionCommentData = {...testDiscussionCommentData };

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: intestDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.errors[0].message).toBe('BadRequest: Access denied');
  });

  // Test 9: Invalid input - no objectUuid specified for objectDiscussion
  it('should return an error objectUuid of objectDiscussion is not set', async () => {
    const intestDiscussionCommentData = {...testDiscussionCommentData };
    intestDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPANY;

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: intestDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.errors[0].message).toBe('Internal Server Error');
  });

  // Test 10: Invalid input - no toObject specified for objectDiscussion
  it('should return an error toObject of objectDiscussion is not set', async () => {
    const intestDiscussionCommentData = {...testDiscussionCommentData };
    intestDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: intestDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.errors[0].message).toBe('Internal Server Error');
  });

  // Test 11: Invalid input - invalid toObject specified for objectDiscussion
  it('should return an error objectDiscussion is set bad option', async () => {
    const intestDiscussionCommentData = {...testDiscussionCommentData };
    intestDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    intestDiscussionCommentData.objectDiscussion.toObject = ToObject.BADOPTION;

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: intestDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.errors[0].message).toContain('Invalid value for argument');
    expect(jsonData.errors[0].message).toContain('does not contain the value');
    expect(jsonData.errors[0].message).toContain(ToObject.BADOPTION);
  });

  // Test 12: Invalid input - empty message specified
  it('should return an error for invalid input data - message is empty', async () => {
    const intestDiscussionCommentData = {...testDiscussionCommentData };
    intestDiscussionCommentData.objectDiscussion.objectUuid = companyUuidSupplier;
    intestDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPANY;
    intestDiscussionCommentData.messageContent = ""; // Невалидное поле (пустая строка)

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: intestDiscussionCommentData },
      }),
    });

    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Data not found');
  });

  // Test 13: Limit duplicate comments (with parent_comment_uuid)
  it('should prevent duplicate comments with parent_comment_uuid', async () => {
    const duplicateCommentData = {...testDiscussionCommentData };
    duplicateCommentData.objectDiscussion.objectUuid = companyUuidSupplier;
    duplicateCommentData.objectDiscussion.toObject = ToObject.COMPANY;
    duplicateCommentData.parentCommentUuid = rootDiscussCommentCompanyUuid;
    // duplicateCommentData.messageContent = 'duplicate check 1145';

    // Первый комментарий регистрируется успешно
    const firstResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: duplicateCommentData },
      }),
    });
    const firstJsonData = await firstResponse.json();
    expect(firstJsonData.data.registerDiscussionComment).toMatch(regexUuid);
    testPreventDuplicateCommentUuid = firstJsonData.data.registerDiscussionComment;

    // Второй комментарий с тем же parent_comment_uuid и messageContent не регистрируется
    const secondResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: duplicateCommentData },
      }),
    });
    const secondJsonData = await secondResponse.json();
    // expect(secondJsonData).toBe(0);
    expect(secondJsonData.errors[0].message).toBe('BadRequest: Found duplicate data');
  });

  // Test 14: Check for creation of first comment in duplicate test
  it('should return one comment on the search criterion with the given filterByParentUuid', async () => {
    const validArgs = {
      discussionUuid: discussionCompanyUuidFirt,
      filterByParentUuid: rootDiscussCommentCompanyUuid,
      // filterByUuids: [testDupFirst],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: null,
          paginate: null,
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussionComments } } = await response.json();
    expect(discussionComments).toBeDefined();
    expect(discussionComments).toBeInstanceOf(Array);
    expect(discussionComments.length).toBe(1);  // Make sure the brought back the only discussion are an array
    expect(discussionComments[0].discussionUuid).toBe(discussionCompanyUuidFirt); // Make sure to return the right discussion
    expect(discussionComments[0].parentCommentUuid).toBe(rootDiscussCommentCompanyUuid); // Make sure to return the right comment message
    expect(discussionComments[0].messageContent).toBe(testDiscussionCommentData.messageContent); // Make sure to return the right message content
    expect(discussionComments[0].uuid).toBe(testPreventDuplicateCommentUuid); // Make sure to return the right discussion
  });

  // Test 15: Limit duplicate comments (without parent_comment_uuid)
  it('should prevent duplicate root comments', async () => {
    const duplicateRootCommentData = {...testDiscussionCommentData };
    duplicateRootCommentData.objectDiscussion.objectUuid = companyUuidSupplier;
    duplicateRootCommentData.objectDiscussion.toObject = ToObject.COMPANY;
    duplicateRootCommentData.messageContent = 'уникальный провал'; // Корневой комментарий

    // 6.1: Первый комментарий регистрируется успешно
    const firstResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: duplicateRootCommentData },
      }),
    });
    const firstJsonData = await firstResponse.json();
    expect(firstJsonData.data.registerDiscussionComment).toMatch(regexUuid);

    // 6.2: Второй комментарий с тем же messageContent не регистрируется
    const secondResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: duplicateRootCommentData },
      }),
    });
    const secondJsonData = await secondResponse.json();
    expect(secondJsonData.errors[0].message).toBe('BadRequest: Found duplicate data');
  });

  // Test 16: Limit message_content length (Latin)
  it('should prevent message_content exceeding 4000 bytes (latin characters)', async () => {
    const longMessageData = {...testDiscussionCommentData };
    longMessageData.objectDiscussion.objectUuid = companyUuidSupplier;
    longMessageData.objectDiscussion.toObject = ToObject.COMPANY;
    longMessageData.messageContent = 'a'.repeat(4001); // Строка из 4001 латинской буквы 'a'

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: longMessageData },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Text must be less than 4000 bit (~2000 symbols)');
  });

  // Test 17: Limit message_content length (non-Latin characters)
  it('should prevent message_content exceeding 4000 bytes (non-latin characters)', async () => {
    const longMessageData = {...testDiscussionCommentData };
    longMessageData.objectDiscussion.objectUuid = companyUuidSupplier;
    longMessageData.objectDiscussion.toObject = ToObject.COMPANY;
    longMessageData.messageContent = 'я'.repeat(4001); // Строка из 4001 символов не из латиницы (например, китайских иероглифов)

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: longMessageData },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Text must be less than 4000 bit (~2000 symbols)');
  });

  // Test 18: Specified discussionUuid not related to parent_comment_uuid
  it('should return an error when discussionUuid is not associated with parent_comment_uuid', async () => {
    const invalidDiscussionCommentData = {...testDiscussionCommentData };
    invalidDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    invalidDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;
    invalidDiscussionCommentData.parentCommentUuid = rootDiscussCommentServiceUuid;

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: invalidDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Failed match arguments');
  });

  // Test 19: No access to commented object (no access to this component)
  it('should return an error when user lacks access to the commentable object', async () => {
    const restrictedDiscussionCommentData = {...testDiscussionCommentData };
    restrictedDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    restrictedDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: restrictedDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Access denied');
  });

  // Test 20: No access to commented object (no access to this component)
  it('should return an error when user lacks access to the commentable object (with parentCommentUuid)', async () => {
    const restrictedDiscussionCommentData = {...testDiscussionCommentData };
    restrictedDiscussionCommentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    restrictedDiscussionCommentData.objectDiscussion.toObject = ToObject.COMPONENT;
    restrictedDiscussionCommentData.parentCommentUuid = rootDiscussCommentComponentUuid;

    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: restrictedDiscussionCommentData },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Access denied');
  });

  // Test 21: Successful discussions request with default sorting
  it('should return discussions with default sorting', async () => {
    const validArgs = {
      objectUuid: companyUuidSupplier,
      toObject: ToObject.COMPANY,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default Page Navigation
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure discussions are defined
    expect(discussions).toBeInstanceOf(Array); // Make sure the discussions are an array
    expect(discussions.length).toBe(1); // Make sure the discussions are an array of a single discussion
    // discussionCompanyUuidFirt = discussions[0].uuid; // Save the UUID of the first discussion
  });

  // Test 22: Query discussions with sorting by title in descending order
  it('should return discussions sorted by title in descending order', async () => {
    const validArgs = {
      objectUuid: componentUuidNoStandard,
      toObject: ToObject.COMPONENT,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'title', asDesc: true }, // Sorting by title in descending order
          paginate: { currentPage: 1, perPage: 5 }, // Default Page Navigation
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure discussions are defined
    expect(discussions).toBeInstanceOf(Array); // Make sure the discussions are an array
    expect(discussions.length).toBe(1); // Make sure the discussions are an array of a single discussion
    discussionComponentUuidFirt = discussions[0].uuid; // Save the UUID of the first discussion
  });

  // Test 23: Query discussions without access to the component (Access denied)
  it('should return an error when user lacks access to the object discussion', async () => {
    const validArgs = {
      objectUuid: componentUuidNoStandard,
      toObject: ToObject.COMPONENT,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'title', asDesc: true }, // Sorting by title in descending order
          paginate: { currentPage: 1, perPage: 5 }, // Default Page Navigation
        },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.errors[0].message).toBe('BadRequest: Access denied');
  });

  // Test 24: Query discussions with pagination (first page, 10 items per page)
  it('should return discussions with pagination (first page, 10 items per page) - result 1', async () => {
    const validArgs = {
      objectUuid: serviceUuidFirst,
      toObject: ToObject.SERVICE,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 10 }, // Second page, 10 items per page
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure discussions are defined
    expect(discussions).toBeInstanceOf(Array); // Make sure the discussions are an array
    expect(discussions.length).toBe(1); // Make sure the discussions are an array of a single discussion
    // expect(discussions.length).toBeLessThanOrEqual(10);
    discussionServiceUuidFirt = discussions[0].uuid; // Save the UUID of the first discussion
  });

  // Test 25: Query discussions with pagination, but pagination does not work for a single discussion
  it('should return the only discussion associated with the object (sorting and pagination are ignored)', async () => {
    const validArgs = {
      objectUuid: serviceUuidFirst,
      toObject: ToObject.SERVICE,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 2, perPage: 10 }, // Second page, 10 items per page
        },
      }),
    });
    // const jsonData = await response.json();
    const { data: { discussions } } = await response.json();
    // expect(jsonData).toBe(0);
    expect(discussions).toBeDefined(); // Make sure discussions are defined
    expect(discussions).toBeInstanceOf(Array); // Make sure the discussions are an array
    expect(discussions.length).toBe(1); // Make sure the brought back the only discussion are an array
    expect(discussions[0].uuid).toBe(discussionServiceUuidFirt); // Make sure to return the right discussion
    // expect(discussions.length).toBeLessThanOrEqual(10);
  });

  // Test 26: Error requesting discussions with invalid objectUuid
  it('should return an error when objectUuid is invalid', async () => {
    const invalidArgs = {
      objectUuid: uuidFail, // Invalid UUID
      toObject: ToObject.COMPANY,
      // filterByUuids: [],
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
            }
          }
        `,
        variables: {
          args: invalidArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default Page Navigation
        },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    expect(jsonData.errors).toBeDefined(); // Make sure the error is defined
    expect(jsonData.errors[0].message).toBe('Internal Server Error'); // Make sure the error is related to a non-valid objectUuid
  });

  // Test 27: Creating comments with replies for each object type (toObject)
  it('should create comments with replies for each toObject type', async () => {
    const toObjectTypes = {
      COMPANY: companyUuidSupplier, // objectUuid for 'COMPANY'
      SERVICE: serviceUuidFirst, // objectUuid for 'SERVICE'
      COMPONENT: componentUuidNoStandard, // objectUuid for 'COMPONENT'
    };
    var authorizationToken = authorizationTokenFirst;
    var topIndex = 1;

    // Create comments for each object type
    for (const [toObjectType, toObjectUuid] of Object.entries(toObjectTypes)) {
      if (toObjectType == ToObject.COMPONENT) {
        authorizationToken = authorizationTokenSecond;
      }
      // Create 5 comments for the current object type
      const comments = await Promise.all(
        Array(5)
        .fill(0)
        .map(async (_, index) => {
            const commentData = {
              objectDiscussion: {
                objectUuid: toObjectUuid,
                toObject: toObjectType,
              },
              discussionUuid: null,
              parentCommentUuid: null,
              messageContent: `Comment ${index + 1} for ${toObjectType}`,
            };
            const response = await fetch(api, {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${authorizationToken}`,
              },
              body: JSON.stringify({
                query: `
                  mutation CreateDiscussionComment($args: IptDiscussionCommentData!) {
                    registerDiscussionComment(args: $args)
                  }
                `,
                variables: { args: commentData },
              }),
            });
            const jsonData = await response.json();
            if (jsonData.data == null) {
              // expect(jsonData).toBe(0);
              var bugLog = commentData;
              expect(jsonData).toBe(`
                topIndex: ${topIndex},
                toObjectUuid: ${toObjectUuid},
                toObjectType: ${toObjectType},
                bugLog.discussionUuid: ${bugLog.discussionUuid},
                bugLog.messageContent: ${bugLog.messageContent},
                bugLog.objectDiscussion.objectUuid: ${bugLog.objectDiscussion.objectUuid},
                bugLog.objectDiscussion.toObject: ${bugLog.objectDiscussion.toObject},
                bugLog.parentCommentUuid: ${bugLog.parentCommentUuid},
              `);
            }
            return jsonData.data.registerDiscussionComment;
          })
      );

      // Creating replies to 3 comments
      const commentsWithReplies = comments.slice(0, 3);
      for (const commentUuid of commentsWithReplies) {
        // Creating 4 replies to current comments
        await Promise.all(
          Array(4)
          .fill(0)
          .map(async (_, index) => {
              topIndex += 1;
              const replyData = {
                objectDiscussion: {
                  objectUuid: toObjectUuid,
                  toObject: toObjectType,
                },
                discussionUuid: null,
                parentCommentUuid: commentUuid,
                messageContent: `Reply ${index + 1}/${topIndex} to Comment for ${toObjectType}, parentCommentUuid ${commentUuid}`,
              };
              const response = await fetch(api, {
                method: 'POST',
                headers: {
                  'Content-Type': 'application/json',
                  Authorization: `Bearer ${authorizationToken}`,
                },
                body: JSON.stringify({
                  query: `
                    mutation CreateDiscussionComment($args: IptDiscussionCommentData!) {
                      registerDiscussionComment(args: $args)
                    }
                  `,
                  variables: { args: replyData },
                }),
              });
              const jsonData = await response.json();
              if (jsonData.data == null) {
                // expect(jsonData).toBe(0);
                var bugLog = replyData;
                expect(jsonData).toBe(`
                  topIndex: ${topIndex},
                  toObjectUuid: ${toObjectUuid},
                  toObjectType: ${toObjectType},
                  bugLog.discussionUuid: ${bugLog.discussionUuid},
                  bugLog.messageContent: ${bugLog.messageContent},
                  bugLog.objectDiscussion.objectUuid: ${bugLog.objectDiscussion.objectUuid},
                  bugLog.objectDiscussion.toObject: ${bugLog.objectDiscussion.toObject},
                  bugLog.parentCommentUuid: ${bugLog.parentCommentUuid},
                `);
              }
              return jsonData.data.registerDiscussionComment;
            })
        );

        // Creating half-answer responses (2 responses)
        const replies = await Promise.all(
          Array(4)
          .fill(0)
          .map(async (_, index) => {
              topIndex += 1;
              const replyUuid = commentsWithReplies[index];
              const replyData = {
                objectDiscussion: {
                  objectUuid: toObjectUuid,
                  toObject: toObjectType,
                },
                discussionUuid: null,
                parentCommentUuid: replyUuid,
                messageContent: `Reply to Reply ${index + 1}/${topIndex} for ${toObjectType}, parentCommentUuid ${replyUuid}`,
              };
              const response = await fetch(api, {
                method: 'POST',
                headers: {
                  'Content-Type': 'application/json',
                  Authorization: `Bearer ${authorizationToken}`,
                },
                body: JSON.stringify({
                  query: `
                    mutation CreateDiscussionComment($args: IptDiscussionCommentData!) {
                      registerDiscussionComment(args: $args)
                    }
                  `,
                  variables: { args: replyData },
                }),
              });
              const jsonData = await response.json();
              if (jsonData.data == null) {
                // expect(jsonData).toBe(0);
                var bugLog = replyData;
                expect(jsonData).toBe(`
                  topIndex: ${topIndex},
                  commentsWithReplies: ${commentsWithReplies},
                  toObjectUuid: ${toObjectUuid},
                  toObjectType: ${toObjectType},
                  bugLog.discussionUuid: ${bugLog.discussionUuid},
                  bugLog.messageContent: ${bugLog.messageContent},
                  bugLog.objectDiscussion.objectUuid: ${bugLog.objectDiscussion.objectUuid},
                  bugLog.objectDiscussion.toObject: ${bugLog.objectDiscussion.toObject},
                  bugLog.parentCommentUuid: ${bugLog.parentCommentUuid},
                `);
              }
              return jsonData.data.registerDiscussionComment;
            })
          .slice(0, 2) // Half-answer responses (2 responses)
        );

        // Create replies to 1 reply from previous threads
        const lastReplyUuid = replies[0];
        // const lastReplyUuid = replies[0];
        const lastReplyData = {
          objectDiscussion: {
            objectUuid: toObjectUuid,
            toObject: toObjectType,
          },
          discussionUuid: null,
          parentCommentUuid: lastReplyUuid,
          messageContent: `Last Reply to Reply for ${toObjectType}, topIndex ${topIndex}`,
        };
        const lastResponse = await fetch(api, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${authorizationToken}`,
          },
          body: JSON.stringify({
            query: `
              mutation CreateDiscussionComment($args: IptDiscussionCommentData!) {
                registerDiscussionComment(args: $args)
              }
            `,
            variables: { args: lastReplyData },
          }),
        });
        const lastJsonData = await lastResponse.json();
        if (lastJsonData.data == null) {
          // expect(lastJsonData).toBe(0);
          var bugLog = lastReplyData;
          expect(lastJsonData).toBe(`
            toObjectUuid: ${toObjectUuid},
            toObjectType: ${toObjectType},
            bugLog.discussionUuid: ${bugLog.discussionUuid},
            bugLog.messageContent: ${bugLog.messageContent},
            bugLog.objectDiscussion.objectUuid: ${bugLog.objectDiscussion.objectUuid},
            bugLog.objectDiscussion.toObject: ${bugLog.objectDiscussion.toObject},
            bugLog.parentCommentUuid: ${bugLog.parentCommentUuid},
          `);
        }
        expect(lastJsonData.data.registerDiscussionComment).toMatch(regexUuid);
        // testRepliesForEachCommentUuid = lastJsonData.data.registerDiscussionComment
      }
    }

    // Everything will go well if we have reached this point.
    expect(true).toBe(true);
  });

  // Test 28: Successful request for discussionComments without filtering and sorting
  it('should return discussion comments without filtering and sorting', async () => {
    const validArgs = {
      discussionUuid: discussionCompanyUuidFirt,
      filterByParentUuid: null, // Without filtering by parent UUID
      filterByUuids: [], // Without filtering by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    const {
      data: { discussionComments },
    } = await response.json();
    // expect(data).toBe(0);
    expect(discussionComments).toBeDefined(); // Make sure comments are defined
    expect(discussionComments).toBeInstanceOf(Array); // Make sure comments are an array
    expect(discussionComments.length).toBe(5);
    expect(discussionComments[0].author.username).toBe("baromi");
    expect(discussionComments[0].messageContent).toBe("Test comment message");
    expect(discussionComments[0].repliesCount).toBe(1);
    test1RepliesCommentFirstUuid = discussionComments[0].uuid;
    expect(discussionComments[1].messageContent).toBe("уникальный провал");
    expect(discussionComments[1].discussionUuid).toBe(discussionCompanyUuidFirt);
    expect(discussionComments[1].parentCommentUuid).toBe(discussionComments[1].uuid);
    expect(discussionComments[1].repliesCount).toBe(0);
    test0RepliesCommentSecondUuid = discussionComments[1].uuid;
    expect(discussionComments[2].messageContent).toContain("for COMPANY");
    expect(discussionComments[3].messageContent).toContain("for COMPANY");
    test7RepliesCommentThreeUuid = discussionComments[3].uuid;
    expect(discussionComments[4].messageContent).toContain("for COMPANY");
  });

  // Test 29: Requesting discussionComments with filtering by parent UUID
  it('should return discussion comments filtered by parent UUID', async () => {
    const validArgs = {
      discussionUuid: discussionCompanyUuidFirt,
      filterByParentUuid: test1RepliesCommentFirstUuid, // Filter by parent UUID
      filterByUuids: [], // Without filtering by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussionComments } } = await response.json();
    expect(discussionComments).toBeDefined(); // Make sure comments are defined
    expect(discussionComments).toBeInstanceOf(Array); // Make sure comments are an array
    expect(discussionComments.length).toBe(1);
    expect(discussionComments.every(comment => comment.uuid === validArgs.filterByParentUuid)).toBe(false);
    expect(discussionComments.every(comment => comment.parentCommentUuid === validArgs.filterByParentUuid)).toBe(true);
  });

  // Test 30: Requesting discussionComments with filtering by UUID
  it('should return discussion comments filtered by UUID', async () => {
    const validArgs = {
      discussionUuid: discussionCompanyUuidFirt,
      filterByParentUuid: null, // Without filtering by parent UUID
      filterByUuids: [test1RepliesCommentFirstUuid, test0RepliesCommentSecondUuid], // Filter by UUID
    };
    const checkMessageContent = ["Test comment message", "уникальный провал"];
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
              replies {\
                ${discussionCommentFieldResponse}
              } \
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussionComments } } = await response.json();
    expect(discussionComments).toBeDefined(); // Make sure comments are defined
    expect(discussionComments).toBeInstanceOf(Array); // Make sure comments are an array
    expect(discussionComments.every(comment => validArgs.filterByUuids.includes(comment.uuid))).toBe(true);
    expect(discussionComments.every(comment => checkMessageContent.includes(comment.messageContent))).toBe(true);
    expect(discussionComments.length).toBe(2);
  });

  // Test 31: Query discussionComments sorted by updatedAt in descending order
  it('should return discussion comments sorted by updatedAt in descending order', async () => {
    const validArgs = {
      discussionUuid: discussionCompanyUuidFirt,
      filterByParentUuid: null, // Without filtering by parent UUID
      filterByUuids: [], // Without filtering by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'updatedAt', asDesc: true }, // Sort by updatedAt in order of removal
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussionComments } } = await response.json();
    expect(discussionComments).toBeDefined(); // Make sure comments are defined
    expect(discussionComments).toBeInstanceOf(Array); // Make sure comments are an array
    const sortedComments = discussionComments.slice().sort((a, b) => b.updatedAt.localeCompare(a.updatedAt));
    expect(JSON.stringify(discussionComments)).toBe(JSON.stringify(sortedComments));
    expect(discussionComments.length).toBe(5);
  });

  // Test 32: Query discussionComments with pagination (second page, 3 items per page)
  it('should return discussion comments with pagination (second page, 3 items per page)', async () => {
    const validArgs = {
      discussionUuid: discussionComponentUuidFirt,
      filterByParentUuid: null, // Without filtering by parent UUID
      filterByUuids: [], // Without filtering by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 2, perPage: 3 }, // Second page, 3 items per page
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussionComments } } = await response.json();
    expect(discussionComments).toBeDefined(); // Make sure comments are defined
    expect(discussionComments).toBeInstanceOf(Array); // Make sure comments are an array
    expect(discussionComments.length).toBeLessThanOrEqual(3); // Make sure there are no more than 3 elements
  });

  // Test 33: Query discussionComments with pagination (first page, 8 items per page)
  it('should return discussion comments with pagination (first page, 8 items per page)', async () => {
    const validArgs = {
      discussionUuid: discussionComponentUuidFirt,
      filterByParentUuid: null, // Without filtering by parent UUID
      filterByUuids: [], // Without filtering by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 8 }, // Second page, 8 items per page
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussionComments } } = await response.json();
    expect(discussionComments).toBeDefined(); // Make sure comments are defined
    expect(discussionComments).toBeInstanceOf(Array); // Make sure comments are an array
    expect(discussionComments.length).toBeLessThanOrEqual(8); // Make sure there are no more than 8 elements
  });

  // Test 34: Error querying discussionComments with invalid discussionUuid
  it('should return an error when discussionUuid is invalid', async () => {
    const invalidArgs = {
      discussionUuid: uuidFail, // Invalid UUID
      filterByParentUuid: null, // Without filtering by parent UUID
      filterByUuids: [], // Without filtering by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query DiscussionComments($args: IptDiscussionCommentsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussionComments(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionCommentFieldResponse}
            }
          }
        `,
        variables: {
          args: invalidArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors).toBeDefined(); // Let's make sure that the definition error
    expect(jsonData.errors[0].message).toContain('BadRequest: Failed check data'); // Убедитесь, что ошибка связана с невалидным discussionUuid
  });

  // Test 35: Successful query discussions without filtering and sorting
  it('should return discussions without filtering and sorting', async () => {
    const validArgs = {
      objectUuid: componentUuidNoStandard, // Replace your object UUID
      toObject: ToObject.COMPONENT, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [], // Without UUID filter
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure the discussion remains
    expect(discussions).toBeInstanceOf(Array); // Make sure discussions are an array
    expect(discussions[0].comments.length).toBe(5);
    expect(discussions.length).toBe(1);
  });

  // Test 36: Error when requesting discussions without access access denied
  it('should return an error when access denied', async () => {
    const invalidArgs = {
      objectUuid: componentUuidNoStandard, // Replace your object UUID
      toObject: ToObject.COMPONENT, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [], // Without UUID filter
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: invalidArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors).toBeDefined(); // Let's make sure that the definition error
    expect(jsonData.errors[0].message).toContain('BadRequest: Access denied');
  });

  // Test 37: Requesting discussions with filtering by UUID
  it('should return discussions filtered by UUID', async () => {
    const validArgs = {
      objectUuid: componentUuidNoStandard, // Replace your object UUID
      toObject: ToObject.COMPONENT, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [discussionComponentUuidFirt], // Filter by UUID
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure the discussion remains
    expect(discussions).toBeInstanceOf(Array); // Make sure discussions are an array
    expect(discussions.every(discussion => discussion.uuid === validArgs.filterByUuids[0])).toBe(true);
    expect(discussions[0].comments.length).toBe(5);
    expect(discussions.length).toBe(1);
  });

  // Test 38: Error when requesting discussions without access access denied
  it('should return an error when access denied', async () => {
    const invalidArgs = {
      objectUuid: serviceUuidFirst, // Replace your object UUID
      toObject: ToObject.SERVICE, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [], // Without UUID filter
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: invalidArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors).toBeDefined(); // Let's make sure that the definition error
    expect(jsonData.errors[0].message).toContain('BadRequest: Access denied');
  });

  // Test 39: Requesting discussions sorted by title in descending order
  it('should return discussions sorted by title in descending order', async () => {
    const validArgs = {
      objectUuid: serviceUuidFirst, // Replace your object UUID
      toObject: ToObject.SERVICE, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [discussionServiceUuidFirt], // Without UUID filter
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'title', asDesc: true }, // Sort by title in deletion order
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure the discussion remains
    expect(discussions).toBeInstanceOf(Array); // Make sure discussions are an array
    const sortedDiscussions = discussions.slice().sort((a, b) => b.title.localeCompare(a.title));
    expect(JSON.stringify(discussions)).toBe(JSON.stringify(sortedDiscussions));
  });

  // Test 40: Query discussions with paginated navigation (second page, 10 items per page)
  it('should return discussions with pagination (second page, 10 items per page)', async () => {
    const validArgs = {
      objectUuid: componentUuidNoStandard, // Replace your object UUID
      toObject: ToObject.COMPONENT, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [], // Without UUID filter
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 2, perPage: 10 }, // Second page, 10 elements per page
        },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = await response.json();
    expect(discussions).toBeDefined(); // Make sure the discussion remains
    expect(discussions).toBeInstanceOf(Array); // Make sure discussions are an array
    expect(discussions.length).toBeLessThanOrEqual(10); // Make sure there are no more than 10 elements
  });

  // Test 41: Error querying discussions with invalid objectUuid
  it('should return an error when objectUuid is invalid', async () => {
    const invalidArgs = {
      objectUuid: uuidFail, // Invalid UUID
      toObject: ToObject.COMPONENT, // Select one of the options: COMPANY, COMPONENT, SERVICE
      filterByUuids: [], // Without UUID filter
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              ${discussionFieldResponse}
              comments(filterByUuids: [], sort: { byField: "createdAt", asDesc: false }, paginate: { currentPage: 1, perPage: 5 }) {
                ${discussionCommentFieldResponse}
              }
            }
          }
        `,
        variables: {
          args: invalidArgs,
          sort: { byField: 'createdAt', asDesc: false }, // Default sorting
          paginate: { currentPage: 1, perPage: 5 }, // Default pagination
        },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors).toBeDefined(); // Let's make sure that the definition error
    expect(jsonData.errors[0].message).toContain('Internal Server Error'); // Make sure the error is related to invalid objectUuid
  });

  // Test 42: Granting access to the component to the user
  it('should return confirm that the component has been granted access to the user', async () => {
    const validArgs = {
      componentUuid: componentUuidNoStandard,
      userUuid: authorizationUserFirst,
      typeAccessId: 3,
    };
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation SetUserAccessComponent($args: IptUserAccessComponentData!) {
            setUserAccessComponent(args: $args)
          }
        `,
        variables: { args: validArgs, },
      }),
    });
    // const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { setUserAccessComponent } } = await response.json();
    expect(setUserAccessComponent).toBe(true);
  });

  // Test 43: Creating multiple comments with 5 levels of replies
  it('should create multiple comments with replies up to 5 levels', async () => {
    const objectUuid = componentUuidNoStandard; // UUID of the object to create comments
    const toObject = ToObject.COMPONENT; // Type of object to create comments
    testRepliesForEachCommentUuid = []; // Crear array to store UUID of created comments

    // Level 1: Create a parent comment
    const parentCommentData = {
      objectDiscussion: {
        objectUuid,
        toObject,
      },
      discussionUuid: null,
      parentCommentUuid: null,
      messageContent: 'Родительский комментарий',
    };
    const parentCommentResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: parentCommentData },
      }),
    });
    const parentCommentJsonData = await parentCommentResponse.json();
    // expect(parentCommentJsonData).toBe(0);
    const parentCommentUuid = parentCommentJsonData.data.registerDiscussionComment;
    testRepliesForEachCommentUuid.push(parentCommentUuid);

    // Level 2: Create a reply to a parent comment
    const replyToParentData = {
      objectDiscussion: {
        objectUuid,
        toObject,
      },
      discussionUuid: null,
      parentCommentUuid,
      messageContent: 'Ответ на родительский комментарий',
    };
    const replyToParentResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: replyToParentData },
      }),
    });
    const replyToParentJsonData = await replyToParentResponse.json();
    const replyToParentUuid = replyToParentJsonData.data.registerDiscussionComment;
    testRepliesForEachCommentUuid.push(replyToParentUuid);

    // Level 3: Create a reply to a parent comment reply
    const replyToReplyData = {
      objectDiscussion: {
        objectUuid,
        toObject,
      },
      discussionUuid: null,
      parentCommentUuid: replyToParentUuid,
      messageContent: 'Ответ на ответ родительского комментария',
    };
    const replyToReplyResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: replyToReplyData },
      }),
    });
    const replyToReplyJsonData = await replyToReplyResponse.json();
    const replyToReplyUuid = replyToReplyJsonData.data.registerDiscussionComment;
    testRepliesForEachCommentUuid.push(replyToReplyUuid);

    // Level 4: Create a reply to a parent comment manager reply
    const replyToReplyToReplyData = {
      objectDiscussion: {
        objectUuid,
        toObject,
      },
      discussionUuid: null,
      parentCommentUuid: replyToReplyUuid,
      messageContent: 'Ответ на ответ ответа родительского комментария',
    };
    const replyToReplyToReplyResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: replyToReplyToReplyData },
      }),
    });
    const replyToReplyToReplyJsonData = await replyToReplyToReplyResponse.json();
    const replyToReplyToReplyUuid = replyToReplyToReplyJsonData.data.registerDiscussionComment;
    testRepliesForEachCommentUuid.push(replyToReplyToReplyUuid);

    // Level 5: Create a reply to a comment author reply reply
    const replyToReplyToReplyToReplyData = {
      objectDiscussion: {
        objectUuid,
        toObject,
      },
      discussionUuid: null,
      parentCommentUuid: replyToReplyToReplyUuid,
      messageContent: 'Ответ на ответ ответа ответа родительского комментария',
    };
    const replyToReplyToReplyToReplyResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation RegisterDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: replyToReplyToReplyToReplyData },
      }),
    });
    const replyToReplyToReplyToReplyJsonData = await replyToReplyToReplyToReplyResponse.json();
    const replyToReplyToReplyToReplyUuid = replyToReplyToReplyToReplyJsonData.data.registerDiscussionComment;
    testRepliesForEachCommentUuid.push(replyToReplyToReplyToReplyUuid);

    // Проверка успешного создания всех комментариев
    expect(testRepliesForEachCommentUuid.length).toBe(5);
    for (const uuid of testRepliesForEachCommentUuid) {
      expect(uuid).toBeDefined();
    }
  });

  // Test 44: Editing someone else's comment (access denied)
  it('should return an error when user lacks access to edit a comment', async () => {
    // UUID of the comment to edit (take the last created one)
    const commentUuid = testRepliesForEachCommentUuid[testRepliesForEachCommentUuid.length - 1];
    const updatedMessage = 'Обновленное содержимое комментария'; // New comment content
    // Data for editing comments
    const editCommentData = {
      commentUuid,
      updatedMessage,
    };

    // Submitting a request to edit comment
    const editCommentResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation EditComment($data: IptEditCommentData!) {
            editComment(data: $data)
          }
        `,
        variables: { data: editCommentData },
      }),
    });
    const jsonData = await editCommentResponse.json();
    expect(jsonData.errors).toBeDefined(); // Let's make sure that the definition error
    expect(jsonData.errors[0].message).toContain('BadRequest: Access denied');
  });

  // Test 45: Editing a comment
  it('should edit a comment', async () => {
    // UUID of the comment to edit (take the last created one)
    const commentUuid = testRepliesForEachCommentUuid[testRepliesForEachCommentUuid.length - 1];
    const updatedMessage = 'Обновленное содержимое комментария'; // New comment content
    // Data for editing comments
    const editCommentData = {
      commentUuid,
      updatedMessage,
    };

    // Submitting a request to edit comment
    const editCommentResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation EditComment($data: IptEditCommentData!) {
            editComment(data: $data)
          }
        `,
        variables: { data: editCommentData },
      }),
    });

    // Checking the quality of comment editing
    const editCommentJsonData = await editCommentResponse.json();
    expect(editCommentJsonData.data.editComment).toBe(true);
  });

  // Test 46: Editing a comment with special characters
  it('should edit a comment with special characters in message', async () => {
    // UUID of the comment to edit (take the last created one)
    const commentUuid = testRepliesForEachCommentUuid[testRepliesForEachCommentUuid.length - 1];
    // New comment content with SQL injection prevention testing
    const updatedMessage = sqlInjections[1];
    // Data for editing comments
    const editCommentData = {
      commentUuid,
      updatedMessage,
    };

    // Submitting a request to edit comment
    const editCommentResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation EditComment($data: IptEditCommentData!) {
            editComment(data: $data)
          }
        `,
        variables: { data: editCommentData },
      }),
    });

    // Checking the quality of comment editing
    const editCommentJsonData = await editCommentResponse.json();
    expect(editCommentJsonData.data.editComment).toBe(true);
  });

  // Test 47: Error requesting discussions with invalid message content invalid - byte 0x00
  it('should return an error for invalid byte sequence for encoding "UTF8": 0x00 (edit message)', async () => {
    // UUID of the comment to edit (take the last created one)
    const commentUuid = testRepliesForEachCommentUuid[testRepliesForEachCommentUuid.length - 1];
    // New comment content with SQL injection prevention testing
    const updatedMessage = sqlInjections[sqlInjections.length - 1];
    // Data for editing comments
    const editCommentData = {
      commentUuid,
      updatedMessage,
    };

    // Submitting a request to edit comment
    const editCommentResponse = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation EditComment($data: IptEditCommentData!) {
            editComment(data: $data)
          }
        `,
        variables: { data: editCommentData },
      }),
    });

    // Checking the error of comment editing with bad content
    const jsonData = await editCommentResponse.json();
    expect(jsonData.errors[0].message).toBe('Internal Server Error');
  });

  // Test 47: Error requesting discussions with invalid message content invalid - byte 0x00
  it('should return an error for invalid byte sequence for encoding "UTF8": 0x00 (new message)', async () => {
    var commentData = {...testDiscussionCommentData };
    commentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    commentData.objectDiscussion.toObject = ToObject.COMPONENT;
    // New comment content with SQL injection prevention testing
    commentData.messageContent = sqlInjections[sqlInjections.length - 1];

    // Submitting a request to edit comment
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation CreateDiscussionComment($args: IptDiscussionCommentData!) {
            registerDiscussionComment(args: $args)
          }
        `,
        variables: { args: commentData },
      }),
    });
    const jsonData = await response.json();
    // Checking the error of new comment with bad content
    expect(jsonData.errors[0].message).toBe('BadRequest: Failed check data');
  });

  // Test 48: Creating comments with replies for each object type (toObject)
  it('should create comments with replies for each toObject type', async () => {
    var commentData = {...testDiscussionCommentData };
    commentData.objectDiscussion.objectUuid = componentUuidNoStandard;
    commentData.objectDiscussion.toObject = ToObject.COMPONENT;
    var authorizationToken = authorizationTokenSecond;
    var topIndex = 0;
    var commentUuids = [];

    // Create comments for each sql injection
    for (const sqlInjectionItem of sqlInjections) {
      topIndex += 1;
      commentData.messageContent = `${sqlInjectionItem}`;
      const response = await fetch(api, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${authorizationToken}`,
        },
        body: JSON.stringify({
          query: `
            mutation CreateDiscussionComment($args: IptDiscussionCommentData!) {
              registerDiscussionComment(args: $args)
            }
          `,
          variables: { args: commentData },
        }),
      });
      const jsonData = await response.json();
      if (jsonData.data == null) {
        // The last sql injection is processed separately
        if (topIndex == sqlInjections.length) {
          expect(jsonData.errors[0].message).toBe('BadRequest: Failed check data');
        } else {
          // expect(jsonData).toBe(0);
          var bugLog = commentData;
          expect(jsonData).toBe(`
            topIndex: ${topIndex},
            toObjectUuid: ${commentData.objectDiscussion.objectUuid},
            toObjectType: ${commentData.objectDiscussion.toObject},
            bugLog.discussionUuid: ${bugLog.discussionUuid},
            bugLog.messageContent: ${bugLog.messageContent},
            bugLog.objectDiscussion.objectUuid: ${bugLog.objectDiscussion.objectUuid},
            bugLog.objectDiscussion.toObject: ${bugLog.objectDiscussion.toObject},
            bugLog.parentCommentUuid: ${bugLog.parentCommentUuid},
          `);
        }
      } else {
        commentUuids.push(jsonData.data.registerDiscussionComment)
      }
    }

    const validArgs = {
      objectUuid: commentData.objectDiscussion.objectUuid,
      toObject: commentData.objectDiscussion.toObject,
      filterByUuids: commentUuids,
    };
    // Check replies to sqlInjections comments
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationToken}`,
      },
      body: JSON.stringify({
        query: `
          query Discussions($args: IptObjectDiscussionsArg!, $sort: IptSort, $paginate: IptPaginate) {
            discussions(args: $args, sort: $sort, paginate: $paginate) {
              uuid \
              title \
              isPinned \
              lastActivityAt \
              createdAt \
              repliesCount \
              comments { \
                ${discussionCommentFieldResponse} \
              } \
            }
          }
        `,
        variables: {
          args: validArgs,
          sort: null,
          paginate: null,
        },
      }),
    });
    const jsonData = await response.json();
    // expect(jsonData).toBe(0);
    const { data: { discussions } } = jsonData;
    expect(discussions).toBeDefined();
    expect(discussions).toBeInstanceOf(Array);
    expect(discussions.length).toBe(1);  // Make sure the brought back the only discussion are an array
    expect(discussions[0].comments.length).toBe(17);  // Make sure the returned comments have the correct array length
    expect(commentUuids.length).toBe(7);
    expect(discussions[0].comments[10].uuid).toBe(validArgs.filterByUuids[0]);
    expect(discussions[0].comments[10].messageContent).toBe(sqlInjections[0]);
    expect(discussions[0].comments[11].uuid).toBe(validArgs.filterByUuids[1]);
    expect(discussions[0].comments[11].messageContent).toBe(sqlInjections[1]);
    expect(discussions[0].comments[12].uuid).toBe(validArgs.filterByUuids[2]);
    expect(discussions[0].comments[12].messageContent).toBe(sqlInjections[2]);
    expect(discussions[0].comments[13].uuid).toBe(validArgs.filterByUuids[3]);
    expect(discussions[0].comments[13].messageContent).toBe(sqlInjections[3]);
    expect(discussions[0].comments[14].uuid).toBe(validArgs.filterByUuids[4]);
    expect(discussions[0].comments[14].messageContent).toBe(sqlInjections[4]);
    expect(discussions[0].comments[15].uuid).toBe(validArgs.filterByUuids[5]);
    expect(discussions[0].comments[15].messageContent).toBe(sqlInjections[5]);
  });

  it('should reject unauthorized deletion attempts', async () => {
    const response = await fetch(api, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        query: `
          mutation deleteDiscussionComment($commentUuid: UUID!) {
            deleteDiscussionComment(commentUuid: $commentUuid)
          }
        `,
        variables: { commentUuid: test7RepliesCommentThreeUuid },
      }),
    });
    const jsonData = await response.json();
    expect(jsonData.errors[0].message).toBe('BadRequest: Token not found');
  });

  it('should return false for comment created by another user owner', async () => {
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenSecond}`,
      },
      body: JSON.stringify({
        query: `
          mutation deleteDiscussionComment($commentUuid: UUID!) {
            deleteDiscussionComment(commentUuid: $commentUuid)
          }
        `,
        variables: { commentUuid: test7RepliesCommentThreeUuid },
      }),
    });
    // const jsonData = await response.json();
    const {
      data: { deleteDiscussionComment },
    } = await response.json();
    expect(deleteDiscussionComment).toBe(false);
  });

  it('should successfully delete a comment with valid UUID', async () => {
        const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation deleteDiscussionComment($commentUuid: UUID!) {
            deleteDiscussionComment(commentUuid: $commentUuid)
          }
        `,
        variables: { commentUuid: test7RepliesCommentThreeUuid },
      }),
    });
    // const jsonData = await response.json();
    const {
      data: { deleteDiscussionComment },
    } = await response.json();
    expect(deleteDiscussionComment).toBe(true);
  });

  it('should return false for non-existent comment', async () => {
    const response = await fetch(api, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${authorizationTokenFirst}`,
      },
      body: JSON.stringify({
        query: `
          mutation deleteDiscussionComment($commentUuid: UUID!) {
            deleteDiscussionComment(commentUuid: $commentUuid)
          }
        `,
        variables: { commentUuid: test7RepliesCommentThreeUuid },
      }),
    });
    // const jsonData = await response.json();
    const {
      data: { deleteDiscussionComment },
    } = await response.json();
    expect(deleteDiscussionComment).toBe(false);
  });
});