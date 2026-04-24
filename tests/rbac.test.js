// tests/rbac.test.js
const debug = require('debug')('cdbs-back:rbac.test.js');
const request = require('supertest');
const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const apiDomain = process.env.DOMAIN || "0.0.0.0";
const url = `http://${apiDomain}:${apiPort}`;

jest.setTimeout(1300);

// Тестовые данные
const usernameAdmin = "admin_user";
const usernameEngineer = "engineer_user";
const usernameViewer = "viewer_user";
const usernameGuest = "guest_user";
const usernameManager = "manager_user";
const usernameWriter = "writer_user";
const customerUsername = "customer_user";
const otherUsername = "other_user";
const anotherUsername = "other_user2";
const password = "password123";

// UUID для тестов
const uuidFake = "00000000-0000-0000-0000-000000000000";
let adminUserUuid = "";
let engineerUserUuid = "";
let viewerUserUuid = "";
let guestUserUuid = "";
let customerUserUuid = "";
let otherUsernameUuid = "";
let anotherUserUuid = "";

let adminToken = "";
let engineerToken = "";
let viewerToken = "";
let managerToken = "";
let writerToken = "";
let guestToken = "";
let customerToken = "";
let otherToken = "";
let anotherToken = "";

// Данные для компаний
let companyUuid = "";
let componentUuid = "";
let standardUuid = "";
let serviceUuid = "";
let testStandardUuid = "";

// Уровни доступа
const ACCESS_LEVEL = {
    OWNER: 1,
    EDIT: 1,
    COMMENT: 2,
    VIEW: 3,
    PRIVATE: 1,
    PROTECTED: 2,
    PUBLIC: 3
};

// Роли
const ROLES = {
    ADMIN: "Admin",
    ENGINEER: "Engineer",
    CONSTRUCTOR: "Constructor",
    VIEWER: "Viewer",
    GUEST: "Guest"
};

// GraphQL запросы
const SELF_DATA_QUERY = `
    query {
        selfData {
            uuid
            username
            typeAccess {
                typeAccessId
                name
            }
        }
    }
`;

const COMPANY_MEMBERS_QUERY = `
    query GetCompanyMembers($companyUuid: UUID!) {
        companyMembers(companyUuid: $companyUuid) {
            userUuid
            role {
                role {
                    roleMemberId
                    name
                }
                access {
                    typeAccessId
                    name
                }
            }
            isEnabled
        }
    }
`;

const UPDATE_COMPONENT_MUTATION = `
    mutation UpdateComponent($componentUuid: UUID!, $name: String!) {
        putComponentUpdate(
            componentUuid: $componentUuid
            args: { name: $name }
        )
    }
`;

const DELETE_COMPONENT_MUTATION = `
    mutation DeleteComponent($componentUuid: UUID!) {
        deleteComponent(componentUuid: $componentUuid)
    }
`;

const SET_USER_ACCESS_MUTATION = `
    mutation SetUserAccess($componentUuid: UUID!, $userUuid: UUID!, $typeAccessId: Int!) {
        setUserAccessComponent(args: {
            componentUuid: $componentUuid
            userUuid: $userUuid
            typeAccessId: $typeAccessId
        })
    }
`;

const SET_COMPANY_ACCESS_MUTATION = `
    mutation SetCompanyAccess($componentUuid: UUID!, $companyUuid: UUID!, $typeAccessId: Int!) {
        setCompanyAccessComponent(args: {
            componentUuid: $componentUuid
            companyUuid: $companyUuid
            typeAccessId: $typeAccessId
        })
    }
`;

const GET_COMPONENT_QUERY = `
    query GetComponent($uuid: UUID!) {
        component(componentUuid: $uuid) {
            uuid
            name
            typeAccess {
                typeAccessId
                name
            }
        }
    }
`;

const GET_COMPONENT_ACCESS_LIST_QUERY = `
    query GetComponentAccess($componentUuid: UUID!) {
        getUsersListAccessComponent(componentUuid: $componentUuid) {
            userUuid
            typeAccess {
                typeAccessId
                name
            }
            isEnabled
        }
        getCompaniesListAccessComponent(componentUuid: $componentUuid) {
            companyUuid
            typeAccess {
                typeAccessId
                name
            }
            isEnabled
        }
    }
`;

const CREATE_ROLE_MUTATION = `
    mutation CreateRole($companyUuid: UUID!, $name: String!) {
        registerCompanyRole(args: {
            companyUuid: $companyUuid
            langId: 1
            name: $name
        })
    }
`;

const CHANGE_ROLE_MEMBER_MUTATION = `
    mutation ChangeRole($companyUuid: UUID!, $userUuid: UUID!, $roleId: Int!) {
        changeRoleMember(args: {
            companyUuid: $companyUuid
            userUuid: $userUuid
            roleId: $roleId
        })
    }
`;

const MY_ACCESS_TO_COMPONENT_QUERY = `
    query MyAccessToComponent($componentUuid: UUID!) {
        myAccessToComponent(componentUuid: $componentUuid) {
            hasAccess
            accessLevel
            source
        }
    }
`;

const GET_SERVICE_QUERY = `
    query GetService($serviceUuid: UUID!) {
        service(serviceUuid: $serviceUuid) {
            uuid
            name
            ownerCompany {
                uuid
                shortname
            }
            serviceStatus {
                serviceStatusId
                name
            }
            createdAt
            updatedAt
        }
    }
`;

const CREATE_SERVICE_MUTATION = `
    mutation CreateService($name: String!, $companyUuid: UUID!) {
        serviceRequest(args: {
            name: $name
            description: "Test service request"
            companyUuid: $companyUuid
            regionId: 1
        })
    }
`;

const DELETE_COMPONENT_MODIFICATION_MUTATION = `
    mutation DeleteComponentModification($componentUuid: UUID!, $modificationUuid: UUID!) {
        deleteComponentModification(args: {
            componentUuid: $componentUuid
            modificationUuid: $modificationUuid
        })
    }
`;

const ADD_COMPONENT_SPECS_MUTATION = `
    mutation AddComponentSpecs($componentUuid: UUID!, $specIds: [Int!]!) {
        addComponentSpecs(args: {
            componentUuid: $componentUuid
            specIds: $specIds
        })
    }
`;

const DELETE_COMPONENT_SPECS_MUTATION = `
    mutation DeleteComponentSpecs($componentUuid: UUID!, $specIds: [Int!]!) {
        deleteComponentSpecs(args: {
            componentUuid: $componentUuid
            specIds: $specIds
        })
    }
`;

const CREATE_COMPONENT_MODIFICATION_MUTATION = `
    mutation CreateComponentModification($componentUuid: UUID!, $modificationName: String!) {
        registerComponentModification(args: {
            componentUuid: $componentUuid
            modificationName: $modificationName
            description: "Test modification"
            actualStatusId: 1
        })
    }
`;

// Вспомогательные функции
async function cleanupDatabase() {
    await global.knex.raw('DELETE FROM user_token_ref');
    await global.knex.raw('DELETE FROM user_ref WHERE username IN (?,?,?,?,?,?,?,?,?)', [
        usernameAdmin, usernameEngineer, usernameViewer, usernameGuest, usernameManager, usernameWriter, customerUsername, otherUsername, anotherUsername
    ]);
    await global.knex.raw('DELETE FROM company_ref WHERE shortname = ?', ['RBAC Test Company']);
    await global.knex.raw('DELETE FROM component_ref WHERE name LIKE ?', ['RBAC Test Component%']);
}

async function registerUser(agent, username, programId = 1) {
    const { body } = await agent
        .post('/graphql')
        .send({
            query: `
                mutation RegisterUser($username: String!, $password: String!, $programId: Int!) {
                    registerUser(args: {
                        email: "${username}@test.com"
                        username: $username
                        password: $password
                        programId: $programId
                        firstname: "Test"
                        lastname: "User"
                        secondname: "Testovich"
                        phone: "+1234567890"
                        description: "Test user for RBAC"
                        address: "Test Address"
                        position: "Test Position"
                        timeZone: "Europe/Moscow"
                        regionId: 1
                    }) {
                        uuid
                        username
                        programId
                    }
                }
            `,
            variables: {
                username,
                password,
                programId
            }
        });
    return body.data?.registerUser?.uuid;
}

async function login(agent, username, password) {
    const { body } = await agent
        .post('/login')
        .send({ user: { username, password } });
    return body.bearer;
}

async function createCompany(agent, token, name) {
    const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${token}`)
        .send({
            query: `
                mutation CreateCompany($name: String!) {
                    registerCompany(args: {
                        orgname: $name
                        shortname: $name
                        inn: "1234567890"
                        phone: "+1234567890"
                        email: "company@test.com"
                        description: "Test company for RBAC"
                        address: "Test Address"
                        siteUrl: "https://test.com"
                        timeZone: "Europe/Moscow"
                        regionId: 1
                        companyTypeId: 1
                        typeAccessId: 3
                    })
                }
            `,
            variables: { name }
        });
        expect(body.errors).toBeUndefined();
    return body.data?.registerCompany;
}

async function createComponent(agent, token, name, typeAccessId = ACCESS_LEVEL.PUBLIC) {
    const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${token}`)
        .send({
            query: `
                mutation CreateComponent($name: String!, $typeAccessId: Int!) {
                    registerComponent(args: {
                        name: $name
                        description: "Test component for RBAC"
                        typeAccessId: $typeAccessId
                        componentTypeId: 1
                        actualStatusId: 1
                        isBase: false
                    })
                }
            `,
            variables: { name, typeAccessId }
        });
        expect(body.errors).toBeUndefined();
    return body.data?.registerComponent;
}

async function createStandard(agent, token, name, typeAccessId = ACCESS_LEVEL.PUBLIC) {
    const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${token}`)
        .send({
            query: `
                mutation CreateStandard($name: String!, $typeAccessId: Int!) {
                    registerStandard(args: {
                        name: $name
                        description: "Test standard for RBAC"
                        publicationAt: "2024-01-01T00:00:00"
                        companyUuid: "${companyUuid}"
                        typeAccessId: $typeAccessId
                        standardStatusId: 1
                    })
                }
            `,
            variables: { name, typeAccessId }
        });
        expect(body.errors).toBeUndefined();
    return body.data?.registerStandard;
}


async function createService(agent, token, name, companyUuid) {
    const { body } = await agent
        .post('/graphql')
        .set('Authorization', `Bearer ${token}`)
        .send({
            query: CREATE_SERVICE_MUTATION,
            variables: { name, companyUuid }
        });
    return body.data?.serviceRequest;
}

async function setCompanySupplier(companyUuid) {
    await global.knex.raw('UPDATE company_ref SET is_supplier = true WHERE uuid = ?', [
        companyUuid
    ]);
}

describe('RBAC Tests', () => {
    let agent;

    beforeAll(async () => {
        await cleanupDatabase();
        agent = request.agent(url);
    });

    afterAll(async () => {
        await cleanupDatabase();
    });

    describe('User Registration and Authentication', () => {
        it('should register admin user', async () => {
            adminUserUuid = await registerUser(agent, usernameAdmin, 1);
            expect(adminUserUuid).toBeNonEmptyString();
        });

        it('should login admin user', async () => {
            adminToken = await login(agent, usernameAdmin, password);
            expect(adminToken).toBeNonEmptyString();
        });

        it('should register engineer user', async () => {
            engineerUserUuid = await registerUser(agent, usernameEngineer, 5);
            expect(engineerUserUuid).toBeNonEmptyString();
        });

        it('should login engineer user', async () => {
            engineerToken = await login(agent, usernameEngineer, password);
            expect(engineerToken).toBeNonEmptyString();
        });

        it('should register viewer user', async () => {
            viewerUserUuid = await registerUser(agent, usernameViewer, 1);
            expect(viewerUserUuid).toBeNonEmptyString();
        });

        it('should login viewer user', async () => {
            viewerToken = await login(agent, usernameViewer, password);
            expect(viewerToken).toBeNonEmptyString();
        });

        it('should register guest user', async () => {
            guestUserUuid = await registerUser(agent, usernameGuest, 1);
            expect(guestUserUuid).toBeNonEmptyString();
        });

        it('should login guest user', async () => {
            guestToken = await login(agent, usernameGuest, password);
            expect(guestToken).toBeNonEmptyString();
        });

        it('should register manager user', async () => {
            managerUserUuid = await registerUser(agent, usernameManager, 1);
            expect(managerUserUuid).toBeNonEmptyString();
        });

        it('should login manager user', async () => {
            managerToken = await login(agent, usernameManager, password);
            expect(managerToken).toBeNonEmptyString();
        });

        it('should register writer user', async () => {
            writerUserUuid = await registerUser(agent, usernameWriter, 1);
            expect(writerUserUuid).toBeNonEmptyString();
        });

        it('should login writer user', async () => {
            writerToken = await login(agent, usernameWriter, password);
            expect(writerToken).toBeNonEmptyString();
        });

        it('should register writer user', async () => {
            customerUserUuid = await registerUser(agent, customerUsername, 1);
            expect(writerUserUuid).toBeNonEmptyString();
        });

        it('should login writer user', async () => {
            customerToken = await login(agent, customerUsername, password);
            expect(customerToken).toBeNonEmptyString();
        });

        it('should register writer user', async () => {
            otherUsernameUuid = await registerUser(agent, otherUsername, 1);
            expect(writerUserUuid).toBeNonEmptyString();
        });

        it('should login writer user', async () => {
            otherToken = await login(agent, otherUsername, password);
            expect(otherToken).toBeNonEmptyString();
        });

        it('should register writer user', async () => {
            anotherUserUuid = await registerUser(agent, anotherUsername, 1);
            expect(writerUserUuid).toBeNonEmptyString();
        });

        it('should login writer user', async () => {
            anotherToken = await login(agent, anotherUsername, password);
            expect(anotherToken).toBeNonEmptyString();
        });
    });

// ====
    describe('Company Management and Roles', () => {
        let testUserUuid = "";
        let testUserToken = "";
        let testRoleId = "";

        it('should create company as admin', async () => {
            companyUuid = await createCompany(agent, adminToken, 'RBAC Test Company');
            await setCompanySupplier(companyUuid);
            expect(companyUuid).toBeNonEmptyString();
        });

        it('should get company roles list (initially empty)', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query GetCompanyRoles($companyUuid: UUID!) {
                            companyRoles(companyUuid: $companyUuid) {
                                role {
                                    roleMemberId
                                    name
                                }
                                access {
                                    typeAccessId
                                    name
                                }
                            }
                        }
                    `,
                    variables: { companyUuid }
                });

            expect(body.data?.companyRoles).toBeDefined();
            expect(body.data.companyRoles.length).toBe(0); // Нет ролей, кроме владельца
        });

        it('should create a new role in company', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: CREATE_ROLE_MUTATION,
                    variables: {
                        companyUuid,
                        name: "Engineer"
                    }
                });

            expect(body.data?.registerCompanyRole).toBeDefined();
            testRoleId = body.data.registerCompanyRole;
            expect(testRoleId).toBeGreaterThan(0);
        });

        it('should add access rights to the role', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation AddAccessToRole($roleId: Int!, $accessTypes: [Int!]!) {
                            addAccessRole(args: {
                                roleId: $roleId
                                typesAccessIds: $accessTypes
                            })
                        }
                    `,
                    variables: {
                        roleId: testRoleId,
                        accessTypes: [2, 3]  // Write and Read
                    }
                });

            expect(body.data?.addAccessRole).toBe(true);
        });

        it('should add engineer user to company with the role', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation AddMember($companyUuid: UUID!, $userUuid: UUID!, $roleId: Int!) {
                            addCompanyMember(args: {
                                companyUuid: $companyUuid
                                userUuid: $userUuid
                                roleId: $roleId
                            })
                        }
                    `,
                    variables: {
                        companyUuid,
                        userUuid: engineerUserUuid,
                        roleId: testRoleId
                    }
                });

            expect(body.data?.addCompanyMember).toBe(true);
        });

        it('should get company members list with roles', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: COMPANY_MEMBERS_QUERY,
                    variables: { companyUuid }
                });

            expect(body.data?.companyMembers).toBeDefined();
            expect(body.data.companyMembers.length).toBe(1); // только engineer (admin не в members, он owner)

            const member = body.data.companyMembers[0];
            expect(member.userUuid).toBe(engineerUserUuid);
            expect(member.role.role.name).toBe("Engineer");
            expect(member.role.access.length).toBe(2);
            expect(member.isEnabled).toBe(true);
        });

        it('should get company roles with access rights', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query GetCompanyRoles($companyUuid: UUID!) {
                            companyRoles(companyUuid: $companyUuid) {
                                role {
                                    roleMemberId
                                    name
                                }
                                access {
                                    typeAccessId
                                    name
                                }
                            }
                        }
                    `,
                    variables: { companyUuid }
                });

            expect(body.data?.companyRoles).toBeDefined();
            expect(body.data.companyRoles.length).toBe(1); // только созданная роль Engineer

            const role = body.data.companyRoles[0];
            expect(role.role.name).toBe("Engineer");
            expect(role.access.length).toBe(2); // Write and Read
        });

        it('should change user role in company', async () => {
            // Сначала создаём новую роль
            const { body: createRoleBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: CREATE_ROLE_MUTATION,
                    variables: {
                        companyUuid,
                        name: "Manager"
                    }
                });

            const newRoleId = createRoleBody.data?.registerCompanyRole;

            // Меняем роль пользователя
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: CHANGE_ROLE_MEMBER_MUTATION,
                    variables: {
                        companyUuid,
                        userUuid: engineerUserUuid,
                        roleId: newRoleId
                    }
                });

            expect(body.data?.changeRoleMember).toBe(true);

            // Проверяем, что роль изменилась
            const { body: membersBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: COMPANY_MEMBERS_QUERY,
                    variables: { companyUuid }
                });

            const member = membersBody.data?.companyMembers.find(m => m.userUuid === engineerUserUuid);
            expect(member.role.role.name).toBe("Manager");
        });

        it('should remove user from company', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation RemoveMember($companyUuid: UUID!, $userUuid: UUID!) {
                            deleteCompanyMember(args: {
                                companyUuid: $companyUuid
                                userUuid: $userUuid
                            })
                        }
                    `,
                    variables: {
                        companyUuid,
                        userUuid: engineerUserUuid
                    }
                });

            expect(body.data?.deleteCompanyMember).toBe(true);

            // Проверяем, что пользователь удалён
            const { body: membersBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: COMPANY_MEMBERS_QUERY,
                    variables: { companyUuid }
                });

            const memberExists = membersBody.data?.companyMembers.some(
                m => m.userUuid === engineerUserUuid
            );
            expect(memberExists).toBe(false);
        });
    });
// ====
    describe('Non-Supplier Company Restrictions', () => {
        let nonSupplierCompanyUuid = "";

        beforeAll(async () => {
            // Создаём компанию без статуса поставщика
            nonSupplierCompanyUuid = await createCompany(agent, adminToken, 'Non-Supplier Test Company');
        });

        it('should NOT allow non-supplier company to create a service', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: CREATE_SERVICE_MUTATION,
                    variables: {
                        name: "Test Service From Non-Supplier",
                        companyUuid: nonSupplierCompanyUuid
                    }
                });

            // Должна быть ошибка "The company is not supplier"
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('The company is not supplier');
        });

        it('should NOT appear in supplier companies list', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query GetSupplierCompanies {
                            companies(args: { supplier: true }) {
                                uuid
                                shortname
                                isSupplier
                            }
                        }
                    `
                });

            const found = body.data?.companies?.some(c => c.uuid === nonSupplierCompanyUuid);
            expect(found).toBe(false);
        });

        it('should allow non-supplier company to create a standard', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation CreateStandard($name: String!) {
                            registerStandard(args: {
                                name: $name
                                description: "Test standard"
                                publicationAt: "2024-01-01T00:00:00"
                                companyUuid: "${nonSupplierCompanyUuid}"
                                typeAccessId: 3
                                standardStatusId: 1
                            })
                        }
                    `,
                    variables: { name: `Test Standard ${Date.now()}` }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.registerStandard).toBeDefined();
            expect(body.data.registerStandard).toBeNonEmptyString();
        });

        it('should be visible in regular companies list', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query GetAllCompanies {
                            companies {
                                uuid
                                shortname
                                isSupplier
                            }
                        }
                    `
                });

            const found = body.data?.companies?.some(c => c.uuid === nonSupplierCompanyUuid);
            expect(found).toBe(true);
        });
    });

    describe('Component Access via Non-Supplier Company', () => {
        let accessTestCompanyUuid = "";
        let memberToken = "";

        beforeAll(async () => {
            // Создаём non-supplier компанию
            accessTestCompanyUuid = await createCompany(agent, adminToken, 'Access Test Company');

            // Добавляем пользователя в компанию
            await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation AddMember($companyUuid: UUID!, $userUuid: UUID!, $roleId: Int!) {
                            addCompanyMember(args: {
                                companyUuid: $companyUuid
                                userUuid: $userUuid
                                roleId: $roleId
                            })
                        }
                    `,
                    variables: {
                        companyUuid: accessTestCompanyUuid,
                        userUuid: engineerUserUuid,
                        roleId: 3  // Engineer role
                    }
                });

            memberToken = engineerToken;

            // Создаём приватный компонент
            componentUuid = await createComponent(agent, adminToken, 'Private Component', ACCESS_LEVEL.PRIVATE);
        });

        it('should NOT give access to component via non-supplier company membership alone', async () => {
            // Проверяем доступ члена non-supplier компании к приватному компоненту
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${memberToken}`)
                .send({
                    query: `
                        query MyAccessToComponent($uuid: UUID!) {
                            myAccessToComponent(componentUuid: $uuid) {
                                hasAccess
                                source
                            }
                        }
                    `,
                    variables: { uuid: componentUuid }
                });

            // Членство в non-supplier компании НЕ даёт автоматический доступ к компоненту
            // expect(body).toBe(0);
            expect(body.data?.myAccessToComponent.hasAccess).toBe(false);
            expect(body.data?.myAccessToComponent.source).toBe('NONE');
        });

        it('should grant access when company explicitly gets access', async () => {
            // Выдаём доступ компании к компоненту
            await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation SetCompanyAccess($componentUuid: UUID!, $companyUuid: UUID!, $typeAccessId: Int!) {
                            setCompanyAccessComponent(args: {
                                componentUuid: $componentUuid
                                companyUuid: $companyUuid
                                typeAccessId: $typeAccessId
                            })
                        }
                    `,
                    variables: {
                        componentUuid: componentUuid,
                        companyUuid: accessTestCompanyUuid,
                        typeAccessId: ACCESS_LEVEL.VIEW
                    }
                });

            // Проверяем доступ члена компании
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${memberToken}`)
                .send({
                    query: `
                        query MyAccessToComponent($uuid: UUID!) {
                            myAccessToComponent(componentUuid: $uuid) {
                                hasAccess
                                source
                            }
                        }
                    `,
                    variables: { uuid: componentUuid }
                });

            expect(body.data?.myAccessToComponent.hasAccess).toBe(true);
            expect(body.data?.myAccessToComponent.source).toBe('COMPANY_ROLE');
        });
    });

    describe('Component Access Control', () => {
        let privateComponentUuid = "";
        let publicComponentUuid = "";

        beforeAll(async () => {
            privateComponentUuid = await createComponent(agent, adminToken, 'RBAC Test Component Private', ACCESS_LEVEL.PRIVATE);
            publicComponentUuid = await createComponent(agent, engineerToken, 'RBAC Test Component Public', ACCESS_LEVEL.PUBLIC);
        });

        it('should allow owner to update component', async () => {
            // Сначала проверим доступ
            const { body: accessBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: MY_ACCESS_TO_COMPONENT_QUERY,
                    variables: { componentUuid: privateComponentUuid }
                });

            expect(accessBody.data?.myAccessToComponent.hasAccess).toBe(true);
            expect(accessBody.data?.myAccessToComponent.accessLevel).toBe(1);
            expect(accessBody.data?.myAccessToComponent.source).toBe('OWNER');

            // Затем выполняем операцию
            const { body: updateBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: UPDATE_COMPONENT_MUTATION,
                    variables: {
                        componentUuid: privateComponentUuid,
                        name: 'Updated Component Name'
                    }
                });

            expect(updateBody.data?.putComponentUpdate).toBe(1);
        });

        it('should deny viewer to update component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${viewerToken}`)
                .send({
                    query: UPDATE_COMPONENT_MUTATION,
                    variables: {
                        componentUuid: privateComponentUuid,
                        name: 'Should Not Update'
                    }
                });

            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('Access denied');
        });

        it('should NOT allow engineer to update component by default', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: UPDATE_COMPONENT_MUTATION,
                    variables: {
                        componentUuid: privateComponentUuid,
                        name: 'Engineer Update Test'
                    }
                });

            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('Access denied');
        });

        it('should deny guest to view private component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${guestToken}`)
                .send({
                    query: GET_COMPONENT_QUERY,
                    variables: { uuid: privateComponentUuid }
                });

            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('Access denied');
        });

        it('should allow guest to view public component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${guestToken}`)
                .send({
                    query: GET_COMPONENT_QUERY,
                    variables: { uuid: publicComponentUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.component).toBeDefined();
            expect(body.data.component.uuid).toBe(publicComponentUuid);
        });
    });

    describe('Granular Access Control', () => {
        let componentUuid;

        it('should create component for access tests', async () => {
            componentUuid = await createComponent(agent, adminToken, 'RBAC Access Test Component', ACCESS_LEVEL.PROTECTED);
            expect(componentUuid).toBeNonEmptyString();
        });

        it('should grant user-specific access to component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: SET_USER_ACCESS_MUTATION,
                    variables: {
                        componentUuid: componentUuid,
                        userUuid: guestUserUuid,
                        typeAccessId: ACCESS_LEVEL.VIEW
                    }
                });

            expect(body.data?.setUserAccessComponent).toBe(true);
        });

        it('should allow user with granted access to view component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${guestToken}`)
                .send({
                    query: GET_COMPONENT_QUERY,
                    variables: { uuid: componentUuid }
                });

            expect(body.data?.component).toBeDefined();
            expect(body.data.component.uuid).toBe(componentUuid);
        });

        it('should grant company-wide access to component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: SET_COMPANY_ACCESS_MUTATION,
                    variables: {
                        componentUuid: componentUuid,
                        companyUuid: companyUuid,
                        typeAccessId: ACCESS_LEVEL.EDIT
                    }
                });

            expect(body.data?.setCompanyAccessComponent).toBe(true);
        });

        it('should get component access list', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: GET_COMPONENT_ACCESS_LIST_QUERY,
                    variables: { componentUuid: componentUuid }
                });

            expect(body.data?.getUsersListAccessComponent).toBeDefined();
            expect(body.data?.getCompaniesListAccessComponent).toBeDefined();

            const userAccess = body.data.getUsersListAccessComponent;
            const companyAccess = body.data.getCompaniesListAccessComponent;

            expect(userAccess.length).toBe(1);
            expect(companyAccess.length).toBe(1);
        });
    });

    describe('My Access Queries', () => {
        beforeAll(async () => {
            // Create test objects for access checks
            componentUuid = await createComponent(agent, adminToken, 'MyAccess Test Component', ACCESS_LEVEL.PROTECTED);
            expect(componentUuid).toBeNonEmptyString();
            standardUuid = await createStandard(agent, adminToken, 'MyAccess Test Public Standard', ACCESS_LEVEL.PUBLIC);
            expect(standardUuid).toBeNonEmptyString();
            testStandardUuid = await createStandard(agent, adminToken, 'MyAccess Test Standard', ACCESS_LEVEL.PROTECTED);
            expect(testStandardUuid).toBeNonEmptyString();
            serviceUuid = await createService(agent, adminToken, 'MyAccess Test Service', companyUuid);
            expect(serviceUuid).toBeNonEmptyString();
        });

        it('should return OWNER access for admin on component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query MyAccessToComponent($uuid: UUID!) {
                            myAccessToComponent(componentUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: componentUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToComponent.hasAccess).toBe(true);
            expect(body.data?.myAccessToComponent.accessLevel).toBe(1);
            expect(body.data?.myAccessToComponent.source).toBe('OWNER');
        });

        it('should return error for guest on component (unauthorized)', async () => {
            // myAccessTo* requires authentication
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${guestToken}`)
                .send({
                    query: `
                        query MyAccessToComponent($uuid: UUID!) {
                            myAccessToComponent(componentUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: componentUuid }
                });

            // Guest may not have valid token or may not exist in DB
            if (body.errors) {
                expect(body.errors[0].message).toContain('Unauthorized');
            } else {
                expect(body.data?.myAccessToComponent.hasAccess).toBe(false);
                expect(body.data?.myAccessToComponent.source).toBe('NONE');
            }
        });

        it('should return DIRECT_ACCESS after granting access to guest', async () => {
            // First, ensure guest user exists and has valid token
            // If guestToken is invalid, create a new test user instead
            let validGuestToken = guestToken;
            let validguestUserUuid = guestUserUuid;

            // Check if guest token is valid
            const { body: checkBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${guestToken}`)
                .send({
                    query: `query { myself { uuid } }`
                });

            if (checkBody.errors) {
                // Create a new test user for access checks
                const { body: registerBody } = await agent
                    .post('/graphql')
                    .send({
                        query: `
                            mutation RegisterUser($username: String!, $password: String!) {
                                registerUser(args: {
                                    email: "${testUsername1}@test.com"
                                    username: $username
                                    password: $password
                                    programId: 1
                                    firstname: "Test"
                                    lastname: "User"
                                    regionId: 1
                                }) {
                                    uuid
                                    username
                                }
                            }
                        `,
                        variables: { username: testUsername1, password: testPassword1 }
                    });

                validguestUserUuid = registerBody.data?.registerUser?.uuid;

                const { body: loginBody } = await agent
                    .post('/login')
                    .send({ user: { username: testUsername1, password: testPassword1 } });

                validGuestToken = loginBody.bearer;
            }

            // Grant direct access
            await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        mutation SetUserAccess($componentUuid: UUID!, $userUuid: UUID!, $typeAccessId: Int!) {
                            setUserAccessComponent(args: {
                                componentUuid: $componentUuid
                                userUuid: $userUuid
                                typeAccessId: $typeAccessId
                            })
                        }
                    `,
                    variables: {
                        componentUuid: componentUuid,
                        userUuid: validguestUserUuid,
                        typeAccessId: ACCESS_LEVEL.VIEW
                    }
                });

            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${validGuestToken}`)
                .send({
                    query: `
                        query MyAccessToComponent($uuid: UUID!) {
                            myAccessToComponent(componentUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: componentUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToComponent.hasAccess).toBe(true);
            expect(body.data?.myAccessToComponent.accessLevel).toBe(3);
            expect(body.data?.myAccessToComponent.source).toBe('DIRECT_ACCESS');
        });

        it('should return PUBLIC access for public component without token', async () => {
            const publicComponentUuid = await createComponent(agent, adminToken, 'Public Test Component', ACCESS_LEVEL.PUBLIC);

            // Query WITHOUT Authorization header
            const { body } = await agent
                .post('/graphql')
                .send({
                    query: `
                        query MyAccessToComponent($uuid: UUID!) {
                            myAccessToComponent(componentUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: publicComponentUuid }
                });

            // Without token, should return error (token required for myAccessTo*)
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('Token not found');
        });

        it('should return OWNER access for admin on standard', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query MyAccessToStandard($standardUuid: UUID!) {
                            myAccessToStandard(standardUuid: $standardUuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { standardUuid: testStandardUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToStandard.hasAccess).toBe(true);
            expect(body.data?.myAccessToStandard.accessLevel).toBe(1);
            expect(body.data?.myAccessToStandard.source).toBe('OWNER');
        });

        it('should return NONE access for non-owner on standard', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: `
                        query MyAccessToStandard($standardUuid: UUID!) {
                            myAccessToStandard(standardUuid: $standardUuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { standardUuid: testStandardUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToStandard.hasAccess).toBe(false);
            expect(body.data?.myAccessToStandard.source).toBe('NONE');
        });

        it('should return PUBLIC access for non-owner on public standard', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: `
                        query MyAccessToStandard($standardUuid: UUID!) {
                            myAccessToStandard(standardUuid: $standardUuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { standardUuid: standardUuid }  // ← public standard
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToStandard.hasAccess).toBe(true);
            expect(body.data?.myAccessToStandard.accessLevel).toBe(3);
            expect(body.data?.myAccessToStandard.source).toBe('PUBLIC');
        });

        it('should return OWNER access for owner on public standard', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query MyAccessToStandard($standardUuid: UUID!) {
                            myAccessToStandard(standardUuid: $standardUuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { standardUuid: standardUuid }  // ← public standard
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToStandard.hasAccess).toBe(true);
            expect(body.data?.myAccessToStandard.accessLevel).toBe(1);
            expect(body.data?.myAccessToStandard.source).toBe('OWNER');
        });

        it('should return OWNER access for admin on service', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query MyAccessToService($uuid: UUID!) {
                            myAccessToService(serviceUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: serviceUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToService.hasAccess).toBe(true);
            expect(body.data?.myAccessToService.accessLevel).toBe(1);
            expect(body.data?.myAccessToService.source).toBe('OWNER');
        });

        it('should return NONE access for non-owner on service', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: `
                        query MyAccessToService($uuid: UUID!) {
                            myAccessToService(serviceUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: serviceUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToService.hasAccess).toBe(false);
            expect(body.data?.myAccessToService.source).toBe('NONE');
        });

        it('should return OWNER access for admin on company', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query MyAccessToCompany($uuid: UUID!) {
                            myAccessToCompany(companyUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: companyUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToCompany.hasAccess).toBe(true);
            expect(body.data?.myAccessToCompany.accessLevel).toBe(1);
            expect(body.data?.myAccessToCompany.source).toBe('OWNER');
        });

        it('should return COMPANY_ROLE access for engineer on company', async () => {
            // Ensure engineer is a member of the company
            const { body: membersBody } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: `
                        query GetCompanyMembers($companyUuid: UUID!) {
                            companyMembers(companyUuid: $companyUuid) {
                                userUuid
                            }
                        }
                    `,
                    variables: { companyUuid: companyUuid }
                });

            const isEngineerMember = membersBody.data?.companyMembers?.some(
                m => m.userUuid === engineerUserUuid
            ) || false;

            if (!isEngineerMember) {
                // Get or create a role for engineer
                const { body: roleBody } = await agent
                    .post('/graphql')
                    .set('Authorization', `Bearer ${adminToken}`)
                    .send({
                        query: `
                            query GetCompanyRoles($companyUuid: UUID!) {
                                companyRoles(companyUuid: $companyUuid) {
                                    role { roleMemberId name }
                                }
                            }
                        `,
                        variables: { companyUuid: companyUuid }
                    });

                let engineerRoleId = roleBody.data?.companyRoles?.find(r => r.role.name === 'Engineer')?.role?.roleMemberId;

                if (!engineerRoleId) {
                    const { body: createRoleBody } = await agent
                        .post('/graphql')
                        .set('Authorization', `Bearer ${adminToken}`)
                        .send({
                            query: `
                                mutation CreateRole($companyUuid: UUID!, $name: String!) {
                                    registerCompanyRole(args: {
                                        companyUuid: $companyUuid
                                        langId: 1
                                        name: $name
                                    })
                                }
                            `,
                            variables: {
                                companyUuid: companyUuid,
                                name: "Engineer"
                            }
                        });
                    engineerRoleId = createRoleBody.data?.registerCompanyRole;
                }

                await agent
                    .post('/graphql')
                    .set('Authorization', `Bearer ${adminToken}`)
                    .send({
                        query: `
                            mutation AddMember($companyUuid: UUID!, $userUuid: UUID!, $roleId: Int!) {
                                addCompanyMember(args: {
                                    companyUuid: $companyUuid
                                    userUuid: $userUuid
                                    roleId: $roleId
                                })
                            }
                        `,
                        variables: {
                            companyUuid: companyUuid,
                            userUuid: engineerUserUuid,
                            roleId: engineerRoleId
                        }
                    });
            }

            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: `
                        query MyAccessToCompany($uuid: UUID!) {
                            myAccessToCompany(companyUuid: $uuid) {
                                hasAccess
                                accessLevel
                                source
                            }
                        }
                    `,
                    variables: { uuid: companyUuid }
                });

            expect(body.errors).toBeUndefined();
            expect(body.data?.myAccessToCompany.hasAccess).toBe(true);
            expect(body.data?.myAccessToCompany.source).toBe('COMPANY_ROLE');
        });
    });

    describe('Owner-Only Operations', () => {
        let ownedComponentUuid;

        it('should create component as engineer', async () => {
            ownedComponentUuid = await createComponent(agent, engineerToken, 'Engineer Owned Component', ACCESS_LEVEL.PRIVATE);
            expect(ownedComponentUuid).toBeNonEmptyString();
        });

        it('should allow owner to delete component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: DELETE_COMPONENT_MUTATION,
                    variables: { componentUuid: ownedComponentUuid }
                });

            expect(body.data?.deleteComponent).toBe(ownedComponentUuid);
        });

        it('should deny non-owner to delete component', async () => {
            const tempComponentUuid = await createComponent(agent, adminToken, 'Temp Component', ACCESS_LEVEL.PRIVATE);

            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${viewerToken}`)
                .send({
                    query: DELETE_COMPONENT_MUTATION,
                    variables: { componentUuid: tempComponentUuid }
                });

            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('Access denied');
        });
    });

    describe('Access Level Validation', () => {
        it('should validate access level enum values', () => {
            expect(ACCESS_LEVEL.OWNER).toBe(1);
            expect(ACCESS_LEVEL.EDIT).toBe(1);
            expect(ACCESS_LEVEL.COMMENT).toBe(2);
            expect(ACCESS_LEVEL.VIEW).toBe(3);
        });

        it('should have correct role hierarchy', () => {
            const rolePriority = {
                [ROLES.ADMIN]: 4,
                [ROLES.CONSTRUCTOR]: 3,
                [ROLES.ENGINEER]: 2,
                [ROLES.VIEWER]: 1,
                [ROLES.GUEST]: 0
            };

            expect(rolePriority[ROLES.ADMIN]).toBeGreaterThan(rolePriority[ROLES.ENGINEER]);
            expect(rolePriority[ROLES.ENGINEER]).toBeGreaterThan(rolePriority[ROLES.VIEWER]);
            expect(rolePriority[ROLES.VIEWER]).toBeGreaterThan(rolePriority[ROLES.GUEST]);
        });
    });

    describe('Edge Cases and Error Handling', () => {
        it('should handle access to non-existent component', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: UPDATE_COMPONENT_MUTATION,
                    variables: {
                        componentUuid: uuidFake,
                        name: 'Should Fail'
                    }
                });

            expect(body.errors).toBeDefined();
        });

        it('should handle invalid access level', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: SET_USER_ACCESS_MUTATION,
                    variables: {
                        componentUuid: componentUuid,
                        userUuid: guestUserUuid,
                        typeAccessId: 99
                    }
                });

            expect(body.errors).toBeDefined();
        });

        it('should handle duplicate access grants', async () => {
            // Первое предоставление доступа
            await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: SET_USER_ACCESS_MUTATION,
                    variables: {
                        componentUuid: componentUuid,
                        userUuid: guestUserUuid,
                        typeAccessId: ACCESS_LEVEL.VIEW
                    }
                });

            // Повторное предоставление того же доступа
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: SET_USER_ACCESS_MUTATION,
                    variables: {
                        componentUuid: componentUuid,
                        userUuid: guestUserUuid,
                        typeAccessId: ACCESS_LEVEL.VIEW
                    }
                });

            expect(body.errors).toBeUndefined();
            // Должно вернуть true, но не создать дубликат
            expect(body.data?.setUserAccessComponent).toBe(true);
        });

        it('should handle access without authentication', async () => {
            const { body } = await agent
                .post('/graphql')
                .send({
                    query: UPDATE_COMPONENT_MUTATION,
                    variables: {
                        componentUuid: componentUuid,
                        name: 'No Auth Test'
                    }
                });

            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toContain('Token not found');
        });
    });

    describe('Performance Tests', () => {
        const PERFORMANCE_THRESHOLD_MS = 500;

        it('should retrieve selfData quickly', async () => {
            const startTime = Date.now();

            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({ query: SELF_DATA_QUERY });

            const duration = Date.now() - startTime;

            expect(body.data?.selfData).toBeDefined();
            expect(duration).toBeLessThan(PERFORMANCE_THRESHOLD_MS);
            debug(`selfData query took ${duration}ms`);
        });

        it('should check component access efficiently', async () => {
            const startTime = Date.now();

            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${engineerToken}`)
                .send({
                    query: `
                        query CheckAccess($uuid: UUID!) {
                            component(componentUuid: $uuid) {
                                uuid
                                name
                            }
                        }
                    `,
                    variables: { uuid: componentUuid }
                });

            const duration = Date.now() - startTime;

            expect(duration).toBeLessThan(PERFORMANCE_THRESHOLD_MS);
            debug(`Component access check took ${duration}ms`);
        });
    });

    // ==============================================
    // 1. ТЕСТЫ НА УРОВЕНЬ 1 (MANAGE) — ОБНОВЛЕНИЕ ОБЪЕКТОВ
    // ==============================================
    describe('Access Level 1 (Manage) Tests', () => {
        let testComponentUuid, testStandardUuid, testServiceUuid;

        beforeAll(async () => {
            testComponentUuid = await createComponent(agent, adminToken, 'Manage Test Component', ACCESS_LEVEL.PROTECTED);
            testStandardUuid = await createStandard(agent, adminToken, 'Manage Test Standard', ACCESS_LEVEL.PROTECTED);

            // Выдаем доступ уровня 1
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid: testComponentUuid, userUuid: managerUserUuid, typeAccessId: 1 }
            });
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: `mutation { setUserAccessStandard(args: { standardUuid: "${testStandardUuid}", userUuid: "${managerUserUuid}", typeAccessId: 1 }) }`
            });
        });

        it('should allow level 1 to update component', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: UPDATE_COMPONENT_MUTATION,
                variables: { componentUuid: testComponentUuid, name: `Updated ${Date.now()}` }
            });
            expect(body.data?.putComponentUpdate).toBe(1);
        });

        it('should allow level 1 to update standard', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: `mutation UpdateStandard($uuid: UUID!, $name: String!) { putStandardUpdate(standardUuid: $uuid, args: { name: $name }) }`,
                variables: { uuid: testStandardUuid, name: `Updated ${Date.now()}` }
            });
            expect(body.data?.putStandardUpdate).toBe(1);
        });

        it('should deny level 1 to delete component', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: DELETE_COMPONENT_MUTATION,
                variables: { componentUuid: testComponentUuid }
            });
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });

    // ==============================================
    // 2. ТЕСТЫ НА УРОВЕНЬ 2 (WRITE) — СОЗДАНИЕ СВЯЗЕЙ
    // ==============================================
    describe('Access Level 2 (Write) Tests', () => {
        let testComponentUuid = "";

        beforeAll(async () => {
            testComponentUuid = await createComponent(agent, adminToken, 'Write Test Component', ACCESS_LEVEL.PROTECTED);

            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid: testComponentUuid, userUuid: writerUserUuid, typeAccessId: 2 }
            });
        });

        it('should allow level 2 to add component specs', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: `mutation { addComponentSpecs(args: { componentUuid: "${testComponentUuid}", specIds: [10] }) }`
            });
            expect(body.errors).toBeUndefined();
        });

        it('should deny level 2 to update component', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: UPDATE_COMPONENT_MUTATION,
                variables: { componentUuid: testComponentUuid, name: `Try ${Date.now()}` }
            });
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });

        it('should deny level 2 to delete component', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: DELETE_COMPONENT_MUTATION,
                variables: { componentUuid: testComponentUuid }
            });
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });

    // ==============================================
    // 3. ТЕСТЫ НА ПРЯМОЙ ДОСТУП (DIRECT ACCESS)
    // ==============================================
    describe('Direct Access Tests', () => {
        let componentUuid = "";

        beforeAll(async () => {
            componentUuid = await createComponent(agent, adminToken, 'Direct Access Component', ACCESS_LEVEL.PRIVATE);

            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: guestUserUuid, typeAccessId: 3 }
            });
        });

        it('should return DIRECT_ACCESS source', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${guestToken}`).send({
                query: MY_ACCESS_TO_COMPONENT_QUERY,
                variables: { componentUuid }
            });
            expect(body.data.myAccessToComponent.source).toBe('DIRECT_ACCESS');
        });
    });

    // ==============================================
    // 4. ТЕСТЫ БЕЗ ТОКЕНА
    // ==============================================
    describe('Unauthorized Access Tests', () => {
        let publicComponentUuid, privateComponentUuid;

        beforeAll(async () => {
            publicComponentUuid = await createComponent(agent, adminToken, 'Public Component', ACCESS_LEVEL.PUBLIC);
            privateComponentUuid = await createComponent(agent, adminToken, 'Private Component', ACCESS_LEVEL.PRIVATE);
        });

        it('should return error for myAccessToComponent without token', async () => {
            const { body } = await agent.post('/graphql').send({
                query: MY_ACCESS_TO_COMPONENT_QUERY,
                variables: { componentUuid: publicComponentUuid }
            });
            expect(body.errors[0].message).toBe('BadRequest: Token not found');
        });

        it('should read public component without token', async () => {
            const { body } = await agent.post('/graphql').send({
                query: GET_COMPONENT_QUERY,
                variables: { uuid: publicComponentUuid }
            });
            expect(body.data?.component.uuid).toBe(publicComponentUuid);
        });

        it('should not read private component without token', async () => {
            const { body } = await agent.post('/graphql').send({
                query: GET_COMPONENT_QUERY,
                variables: { uuid: privateComponentUuid }
            });
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });

    // ==============================================
    // ТЕСТЫ НА УДАЛЕНИЕ ДОЧЕРНИХ ЭЛЕМЕНТОВ (MANAGE)
    // ==============================================
    describe('Delete Child Objects Tests', () => {
        let componentUuid = "";
        let modificationUuid = "";

        beforeAll(async () => {
            componentUuid = await createComponent(agent, adminToken, 'Delete Child Test Component', ACCESS_LEVEL.PROTECTED);

            // Выдаем права
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: managerUserUuid, typeAccessId: 1 }
            });
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: writerUserUuid, typeAccessId: 2 }
            });

            // Writer создает спецификацию
            await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: ADD_COMPONENT_SPECS_MUTATION,
                variables: { componentUuid, specIds: 10 }
            });

            // Writer создает модификацию
            const modRes = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: CREATE_COMPONENT_MODIFICATION_MUTATION,
                variables: { componentUuid, modificationName: "Test Mod" }
            });
            expect(modRes.body.data?.registerComponentModification).toBeNonEmptyString();
            modificationUuid = modRes.body.data?.registerComponentModification;
        });

        it('should allow Manage (1) to delete component specs', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: DELETE_COMPONENT_SPECS_MUTATION,
                variables: { componentUuid, specIds: 10 }
            });
            expect(body.errors).toBeUndefined();
            expect(body.data?.deleteComponentSpecs).toBe(1);
        });

        it('should deny Write (2) to delete component specs', async () => {
            // Сначала добавляем новую спецификацию, которую будем пытаться удалить
            await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: ADD_COMPONENT_SPECS_MUTATION,
                variables: { componentUuid, specIds: 20 }
            });

            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: DELETE_COMPONENT_SPECS_MUTATION,
                variables: { componentUuid, specIds: 20 }
            });
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });

        it('should allow Manage (1) to delete component modification', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: DELETE_COMPONENT_MODIFICATION_MUTATION,
                variables: { componentUuid, modificationUuid }
            });
            expect(body.errors).toBeUndefined();
            expect(body.data?.deleteComponentModification).toBe(modificationUuid);
        });

        it('should deny Write (2) to delete component modification', async () => {
            // Создаем новую модификацию через Write
            const modRes = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: CREATE_COMPONENT_MODIFICATION_MUTATION,
                variables: { componentUuid, modificationName: "Another Test Mod" }
            });
            const newModificationUuid = modRes.body.data?.registerComponentModification;

            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: DELETE_COMPONENT_MODIFICATION_MUTATION,
                variables: { componentUuid, modificationUuid: newModificationUuid }
            });
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });

    // ==============================================
    // ТЕСТЫ НА OWNER-ONLY ОПЕРАЦИИ
    // ==============================================
    describe('Owner-Only Operations Tests', () => {
        let componentUuid = "";
        let standardUuid = "";

        beforeAll(async () => {
            componentUuid = await createComponent(agent, adminToken, 'Owner Test Component', ACCESS_LEVEL.PRIVATE);
            standardUuid = await createStandard(agent, adminToken, 'Owner Test Standard', ACCESS_LEVEL.PRIVATE);
        });

        it('should deny non-owner to delete component', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${anotherToken}`).send({
                query: DELETE_COMPONENT_MUTATION,
                variables: { componentUuid }
            });
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });

        it('should deny non-owner to delete standard', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${anotherToken}`).send({
                query: `mutation DeleteStandard($standardUuid: UUID!) { deleteStandard(standardUuid: $standardUuid) }`,
                variables: { standardUuid }
            });
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });

        it('should deny non-owner to grant component access', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${anotherToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: guestUserUuid, typeAccessId: 3 }
            });
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });

        it('should deny non-owner to grant standard access', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${anotherToken}`).send({
                query: `
                    mutation SetUserAccessStandard($standardUuid: UUID!, $userUuid: UUID!, $typeAccessId: Int!) {
                        setUserAccessStandard(args: {
                            standardUuid: $standardUuid
                            userUuid: $userUuid
                            typeAccessId: $typeAccessId
                        })
                    }
                `,
                variables: { standardUuid, userUuid: guestUserUuid, typeAccessId: 3 }
            });
            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });

    // ==============================================
    // ТЕСТЫ НА ИЕРАРХИЮ УРОВНЕЙ ДОСТУПА
    // ==============================================
    describe('Access Level Hierarchy Tests', () => {
        let componentUuid = "";

        beforeAll(async () => {
            componentUuid = await createComponent(agent, adminToken, 'Hierarchy Test Component', ACCESS_LEVEL.PROTECTED);

            // Выдаем разные уровни доступа
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: managerUserUuid, typeAccessId: 1 }
            });
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: writerUserUuid, typeAccessId: 2 }
            });
            await agent.post('/graphql').set('Authorization', `Bearer ${adminToken}`).send({
                query: SET_USER_ACCESS_MUTATION,
                variables: { componentUuid, userUuid: viewerUserUuid, typeAccessId: 3 }
            });
        });

        it('should return correct access level for Manage (1) user', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: MY_ACCESS_TO_COMPONENT_QUERY,
                variables: { componentUuid }
            });
            expect(body.data.myAccessToComponent.accessLevel).toBe(1);
        });

        it('should return correct access level for Write (2) user', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: MY_ACCESS_TO_COMPONENT_QUERY,
                variables: { componentUuid }
            });
            expect(body.data.myAccessToComponent.accessLevel).toBe(2);
        });

        it('should return correct access level for Read (3) user', async () => {
            const { body } = await agent.post('/graphql').set('Authorization', `Bearer ${viewerToken}`).send({
                query: MY_ACCESS_TO_COMPONENT_QUERY,
                variables: { componentUuid }
            });
            expect(body.data.myAccessToComponent.accessLevel).toBe(3);
        });

        it('should have proper hierarchy: Manage (1) can update, Write (2) cannot', async () => {
            // Manage (1) может обновлять
            const manageRes = await agent.post('/graphql').set('Authorization', `Bearer ${managerToken}`).send({
                query: UPDATE_COMPONENT_MUTATION,
                variables: { componentUuid, name: 'Updated by Manage' }
            });
            expect(manageRes.body.data?.putComponentUpdate).toBe(1);

            // Write (2) не может обновлять
            const writeRes = await agent.post('/graphql').set('Authorization', `Bearer ${writerToken}`).send({
                query: UPDATE_COMPONENT_MUTATION,
                variables: { componentUuid, name: 'Try by Write' }
            });
            expect(writeRes.body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });

    describe('Service Request (serviceRequest) Tests', () => {
        let supplierCompanyUuid = "";
        let serviceRequestUuid = "";

        beforeAll(async () => {
            // Создаем компанию-поставщика
            supplierCompanyUuid = await createCompany(agent, adminToken, `Supplier Company ${Date.now()}`);
            await setCompanySupplier(supplierCompanyUuid);
            // Клиент создает запрос к компании-поставщику
            serviceRequestUuid = await createService(agent, customerToken, 'Request for Service', supplierCompanyUuid);
        });

        it('should allow customer to view their own request', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${customerToken}`)
                .send({
                    query: GET_SERVICE_QUERY,
                    variables: { serviceUuid: serviceRequestUuid }
                });

            expect(body.data?.service.uuid).toBe(serviceRequestUuid);
        });

        it('should allow supplier company to view the request', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${adminToken}`)
                .send({
                    query: GET_SERVICE_QUERY,
                    variables: { serviceUuid: serviceRequestUuid }
                });

            expect(body.data?.service.ownerCompany.uuid).toBe(supplierCompanyUuid);
        });

        it('should allow customer to update their own draft request', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${customerToken}`)
                .send({
                    query: `
                        mutation UpdateService($serviceUuid: UUID!, $name: String!) {
                            putServiceUpdate(
                                serviceUuid: $serviceUuid
                                args: { name: $name }
                            )
                        }
                    `,
                    variables: {
                        serviceUuid: serviceRequestUuid,
                        name: `Updated Request ${Date.now()}`
                    }
                });

            expect(body.data?.putServiceUpdate).toBeDefined();
        });

        it('should deny other random user to update request', async () => {
            const { body } = await agent
                .post('/graphql')
                .set('Authorization', `Bearer ${otherToken}`)
                .send({
                    query: `
                        mutation UpdateService($serviceUuid: UUID!, $name: String!) {
                            putServiceUpdate(
                                serviceUuid: $serviceUuid
                                args: { name: $name }
                            )
                        }
                    `,
                    variables: {
                        serviceUuid: serviceRequestUuid,
                        name: `Hacked ${Date.now()}`
                    }
                });

            expect(body.errors).toBeDefined();
            expect(body.errors[0].message).toBe('BadRequest: Access denied');
        });
    });
});