use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::supplier_service::access::util::check_user_access_provided_by_company;
use diesel::prelude::*;
use uuid::Uuid;

/// Access operations for RBAC permission checking.
///
/// Each operation maps to an access level:
/// - `Read`   → level 3 (view only)
/// - `Write`  → level 2 (create, update relations, add child objects)
/// - `Manage` → level 1 (update core object data)
///
/// Note: Deleting an object and managing access rights are OWNER-only operations,
/// not covered by these levels. Use `check_is_owner()` for those.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AccessOperation {
    /// Read-only access. Level 3.
    /// Allows viewing the object and its metadata.
    Read,
    /// Write access. Level 2.
    /// Allows creating child objects, adding relations (specs, keywords, files),
    /// and creating modifications.
    Write,
    /// Manage access. Level 1.
    /// Allows updating core object data (name, description, etc.),
    /// but NOT deleting the object or managing access rights.
    Manage,
}

impl AccessOperation {
    /// Преобразуем действие в числовой уровень доступа (1-3)
    /// 1 - полный доступ, 2 - запись, 3 - чтение
    pub(crate) fn to_access_level(self) -> i32 {
        match self {
            AccessOperation::Manage => 1,
            AccessOperation::Write => 2,
            AccessOperation::Read => 3,
        }
    }
}

/// Типы ресурсов, к которым ограничиваем доступ
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) enum AccessEntity {
    Company,
    Component,
    Standard,
    Service,
    User,
    // Discussion,
}

/// Основная структура для проверки прав
pub(crate) struct PermissionChecker<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> PermissionChecker<'a> {
    pub(crate) fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    /// Проверить, имеет ли пользователь право на действие
    pub(crate) fn check(
        &mut self,
        user_uuid: &Uuid,
        access_entity: AccessEntity,
        access_entity_uuid: &Uuid,
        action: AccessOperation,
    ) -> ServiceResult<bool> {
        match access_entity {
            AccessEntity::Component => {
                self.check_component_access(user_uuid, access_entity_uuid, action)
            }
            AccessEntity::Standard => {
                self.check_standard_access(user_uuid, access_entity_uuid, action)
            }
            AccessEntity::Service => {
                self.check_service_access(user_uuid, access_entity_uuid, action)
            }
            AccessEntity::Company => {
                self.check_company_access(user_uuid, access_entity_uuid, action)
            }
            AccessEntity::User => self.check_user_access(user_uuid, access_entity_uuid, action),
            // AccessEntity::Discussion => self.check_discussion_access(user_uuid, access_entity_uuid, action),
        }
    }

    /// Проверить доступ к компоненту
    fn check_component_access(
        &mut self,
        user_uuid: &Uuid,
        component_uuid: &Uuid,
        action: AccessOperation,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::util::{
            check_access_component_for_user, check_is_owner,
        };

        let need_access_level = action.to_access_level();

        // Быстрый путь: если пользователь владелец
        if check_is_owner(user_uuid, component_uuid, self.conn)? {
            return Ok(true);
        }

        check_access_component_for_user(user_uuid, component_uuid, need_access_level, self.conn)
    }

    /// Проверить доступ к стандарту
    fn check_standard_access(
        &mut self,
        user_uuid: &Uuid,
        standard_uuid: &Uuid,
        action: AccessOperation,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::util::{
            check_access_standard_for_user, check_is_owner,
        };

        let need_access_level = action.to_access_level();

        if check_is_owner(user_uuid, standard_uuid, self.conn)? {
            return Ok(true);
        }

        check_access_standard_for_user(user_uuid, standard_uuid, need_access_level, self.conn)
    }

    /// Проверить доступ к сервису
    fn check_service_access(
        &mut self,
        user_uuid: &Uuid,
        service_uuid: &Uuid,
        action: AccessOperation,
    ) -> ServiceResult<bool> {
        use crate::models::supplier_service::access::util::{
            check_access_service_for_user, check_is_owner,
        };

        let need_access_level = action.to_access_level();

        match action {
            // Уровень Manage подразумевает административные действия над сервисом.
            // Проверяем права через компанию-поставщика (имеет ли пользователь соответствующие полномочия внутри организации, владеющей сервисом).
            AccessOperation::Manage => check_user_access_provided_by_company(
                user_uuid,
                service_uuid,
                need_access_level,
                self.conn,
            ),
            // Проверка: является ли пользователь прямым владельцем сервиса? Если да — доступ на редактирование разрешен.
            AccessOperation::Write => check_is_owner(user_uuid, service_uuid, self.conn),
            // 2. Дополнительная проверка: есть ли у пользователя делегированные права или права через участие в проекте/команде на нужном уровне.
            _ => {
                check_access_service_for_user(user_uuid, service_uuid, need_access_level, self.conn)
            }
        }
    }

    /// Проверить доступ к компании
    fn check_company_access(
        &mut self,
        user_uuid: &Uuid,
        company_uuid: &Uuid,
        action: AccessOperation,
    ) -> ServiceResult<bool> {
        use crate::models::company::access::util::{check_company_access, check_is_owner_company};

        let need_access_level = action.to_access_level();

        if check_is_owner_company(user_uuid, company_uuid, self.conn)? {
            return Ok(true);
        }

        check_company_access(user_uuid, company_uuid, need_access_level, self.conn)
    }

    /// Проверить доступ к пользователю
    fn check_user_access(
        &mut self,
        logged_user_uuid: &Uuid,
        target_user_uuid: &Uuid,
        action: AccessOperation,
    ) -> ServiceResult<bool> {
        use crate::models::user::access::util::check_access_user_for_user;

        // Пользователь всегда имеет полный доступ к себе
        if logged_user_uuid == target_user_uuid {
            return Ok(true);
        }

        let need_access_level = action.to_access_level();
        check_access_user_for_user(
            logged_user_uuid,
            target_user_uuid,
            need_access_level,
            self.conn,
        )
    }

    // /// Проверить доступ к обсуждению
    // fn check_discussion_access(
    //     &mut self,
    //     user_uuid: &Uuid,
    //     discussion_uuid: &Uuid,
    //     action: AccessOperation,
    // ) -> ServiceResult<bool> {
    //     use crate::models::relate_ref::discussion::model::DiscussionTo;

    //     // Находим объект, к которому привязано обсуждение
    //     let discussion_to = DiscussionTo::by_discuss_uuid(discussion_uuid, self.conn)?;

    //     // Проверяем доступ к этому объекту
    //     match discussion_to {
    //         DiscussionTo::Company(company_uuid) => {
    //             self.check_company_access(user_uuid, &company_uuid, action)
    //         }
    //         DiscussionTo::Component(component_uuid) => {
    //             self.check_component_access(user_uuid, &component_uuid, action)
    //         }
    //         DiscussionTo::Service(service_uuid) => {
    //             self.check_service_access(user_uuid, &service_uuid, action)
    //         }
    //     }
    // }
}

/// Обёртка для быстрой проверки
pub(crate) fn check_permission(
    user_uuid: &Uuid,
    access_entity: AccessEntity,
    access_entity_uuid: &Uuid,
    action: AccessOperation,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let mut checker = PermissionChecker::new(conn);
    checker.check(user_uuid, access_entity, access_entity_uuid, action)
}

/// Проверка с автоматической ошибкой (для удобства в бизнес-логике)
pub(crate) fn require_permission(
    user_uuid: &Uuid,
    access_entity: AccessEntity,
    access_entity_uuid: &Uuid,
    action: AccessOperation,
    conn: &mut PgConnection,
) -> ServiceResult<()> {
    if !check_permission(user_uuid, access_entity, access_entity_uuid, action, conn)? {
        return Err(get_err_msg(ErrorMessage::AccessDenied));
    }
    Ok(())
}
