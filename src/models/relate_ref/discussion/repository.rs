use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::discussion::access::CommentCriteria;
use crate::models::search::filter::Filter;
use crate::models::search::model::ObjectUuid;
use crate::schema::discus_to_company::dsl as discus_to_company;
use crate::schema::discus_to_component::dsl as discus_to_component;
use crate::schema::discus_to_service::dsl as discus_to_service;
use crate::schema::discussion_comment_list::dsl as discussion_comment_list;
use crate::schema::discussion_ref::dsl as discussion_ref;
// use crate::models::search::order::Paginate;
use super::model::{
    CommentQueryOptions, DiscusToCompany, DiscusToComponent, DiscusToService, Discussion,
    DiscussionCommentList, DiscussionTo,
};
use crate::models::company::access::util::check_company_access;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::supplier_service::access::util::check_access_service_for_user;
use diesel::prelude::*;
use uuid::Uuid;

impl DiscussionTo {
    pub(crate) fn check_access(
        &self,
        logged_user_uuid: &Uuid,
        need_access_level: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<bool> {
        debug!("Checking user access to the object {:?}", self);
        match self {
            Self::Company(company_uuid) => {
                check_company_access(logged_user_uuid, company_uuid, need_access_level, conn)
            }
            Self::Component(component_uuid) => check_access_component_for_user(
                logged_user_uuid,
                component_uuid,
                need_access_level,
                conn,
            ),
            Self::Service(service_uuid) => check_access_service_for_user(
                logged_user_uuid,
                service_uuid,
                need_access_level,
                conn,
            ),
        }
    }

    pub(crate) fn get_discuss_uuids(&self, conn: &mut PgConnection) -> ServiceResult<Vec<Uuid>> {
        let res = match self {
            Self::Company(company_uuid) => discus_to_company::discus_to_company
                .filter(discus_to_company::company_uuid.eq(company_uuid))
                .select(discus_to_company::discussion_uuid)
                .limit(1000)
                .load::<Uuid>(conn),
            Self::Component(component_uuid) => discus_to_component::discus_to_component
                .filter(discus_to_component::component_uuid.eq(component_uuid))
                .select(discus_to_component::discussion_uuid)
                .limit(1000)
                .load::<Uuid>(conn),
            Self::Service(service_uuid) => discus_to_service::discus_to_service
                .filter(discus_to_service::service_uuid.eq(service_uuid))
                .select(discus_to_service::discussion_uuid)
                .limit(1000)
                .load::<Uuid>(conn),
        };
        debug!("Discuss {:?} are found for {:?}", res, self);
        res.map_err(|err| {
            debug!("Failed get discussion: {:?}", err);
            ServiceError::InternalServerError
        })
    }

    /// Returns the active discussion for the object if there is no discussion.
    /// If a discussion identifier is given and the discussion is not associated with the object, an error is returned.
    pub(crate) fn get_associated_discussion_uuid(
        &self,
        discussion_uuid: &Option<Uuid>,
        conn: &mut PgConnection,
    ) -> ServiceResult<Uuid> {
        if let Some(du) = discussion_uuid {
            if self.is_equals(&DiscussionTo::by_discuss_uuid(du, conn)?) {
                return Ok(*du);
            }
            debug!("Failed get discussion with filter: {:?}", du);
            return Err(get_err_msg(ErrorMessage::NotFoundDiscussion));
        }
        let current_discus_uuids = self.get_discuss_uuids(conn)?;
        debug!(
            "Appropriate discussions were found: {:?}",
            current_discus_uuids
        );
        match current_discus_uuids.first() {
            Some(cdu) => Ok(*cdu),
            None => {
                debug!("Not found discussion (current_discus_uuids is empty)");
                Err(ServiceError::InternalServerError)
            }
        }
    }

    /// Compares two instances of `DiscussionTo` and returns `true` if they are equal, otherwise `false`
    pub(crate) fn is_equals(&self, other: &DiscussionTo) -> bool {
        match (self, other) {
            (Self::Company(company_uuid), Self::Company(other_company_uuid)) => {
                company_uuid == other_company_uuid
            }
            (Self::Component(component_uuid), Self::Component(other_component_uuid)) => {
                component_uuid == other_component_uuid
            }
            (Self::Service(service_uuid), Self::Service(other_service_uuid)) => {
                service_uuid == other_service_uuid
            }
            _ => false, // Different enumeration options or different UUIDs
        }
    }

    /// Returns the Uuid of the object with which the discussion is associated
    pub(crate) fn by_discuss_uuid(
        discussion_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Self> {
        // search for a discussion-related company
        if let Some(company_uuid) = discus_to_company::discus_to_company
            .filter(discus_to_company::discussion_uuid.eq(discussion_uuid))
            .select(discus_to_company::company_uuid)
            .first::<Uuid>(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed check discussion to Company: {:?}", err);
                ServiceError::InternalServerError
            })?
        {
            return Ok(DiscussionTo::Company(company_uuid));
        }

        // search for a discussion-related component
        if let Some(component_uuid) = discus_to_component::discus_to_component
            .filter(discus_to_component::discussion_uuid.eq(discussion_uuid))
            .select(discus_to_component::component_uuid)
            .first::<Uuid>(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed check discussion to Component: {:?}", err);
                ServiceError::InternalServerError
            })?
        {
            return Ok(DiscussionTo::Component(component_uuid));
        }

        // search for a discussion-related service
        if let Some(service_uuid) = discus_to_service::discus_to_service
            .filter(discus_to_service::discussion_uuid.eq(discussion_uuid))
            .select(discus_to_service::service_uuid)
            .first::<Uuid>(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed check discussion to Service: {:?}", err);
                ServiceError::InternalServerError
            })?
        {
            return Ok(DiscussionTo::Service(service_uuid));
        }

        debug!(
            "no connection to the discussion {:?} of subject was found",
            discussion_uuid
        );
        Err(get_err_msg(ErrorMessage::FailedCheckData))
    }

    /// Creates a link between the discussion and the subject
    pub(crate) fn relate_discussion_to_object(
        &self,
        discussion_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<usize> {
        match self {
            Self::Company(company_uuid) => {
                // discus_to_company
                let insert_data = DiscusToCompany {
                    discussion_uuid: *discussion_uuid,
                    company_uuid: *company_uuid,
                };
                diesel::insert_into(discus_to_company::discus_to_company)
                    .values(insert_data)
                    .execute(conn)
                    .map_err(|err| {
                        debug!("Failed insert row in discus_to_company: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
            Self::Component(component_uuid) => {
                // discus_to_component
                let insert_data = DiscusToComponent {
                    discussion_uuid: *discussion_uuid,
                    component_uuid: *component_uuid,
                };
                diesel::insert_into(discus_to_component::discus_to_component)
                    .values(insert_data)
                    .execute(conn)
                    .map_err(|err| {
                        debug!("Failed insert row in discus_to_component: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
            Self::Service(service_uuid) => {
                // discus_to_service
                let insert_data = DiscusToService {
                    discussion_uuid: *discussion_uuid,
                    service_uuid: *service_uuid,
                };
                diesel::insert_into(discus_to_service::discus_to_service)
                    .values(insert_data)
                    .execute(conn)
                    .map_err(|err| {
                        debug!("Failed insert row in discus_to_service: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        }
    }
}

impl Discussion {
    pub(crate) fn get_by_uuids(
        discussion_uuids: &[Uuid],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<Discussion>> {
        discussion_ref::discussion_ref
            .filter(discussion_ref::uuid.eq_any(discussion_uuids))
            .load::<Discussion>(conn)
            .map_err(|err| {
                debug!("Failed get discussion: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    pub(crate) fn comments_count(
        comment_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i64> {
        discussion_comment_list::discussion_comment_list
            .filter(discussion_comment_list::discussion_uuid.eq(comment_uuid))
            .count()
            .get_result(conn)
            .map_err(|err| {
                debug!("Failed to get comment counter for discussion: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl DiscussionCommentList {
    pub(crate) fn get_uuids(
        select_args: &CommentQueryOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<Uuid>> {
        let filter_uuids = Filter::parsing("uuid", &select_args.comment_uuids);
        let query = format!("
        SELECT uuid
        FROM discussion_comment_list
        WHERE discussion_uuid = '{discussion_uuid}' {check_self_parent} {filter_uuids} {sort} {paginate};",
            discussion_uuid = select_args.discussion_uuid,
            check_self_parent = CommentCriteria::filter_by_parent(select_args.parent_uuid),
            filter_uuids = filter_uuids.get_complete(),
            sort = select_args.sort.get_complete(),
            paginate = select_args.paginate.get_complete(),
        );
        debug!(
            "SQL discussion comment list query (set parent_uuid: {:?}): {}",
            select_args.parent_uuid, query
        );
        let temp: Vec<ObjectUuid> = diesel::sql_query(query).load(conn).map_err(|err| {
            debug!("Failed get discussion comment: {:?}", err);
            ServiceError::InternalServerError
        })?;
        Ok(ObjectUuid::get_uuids(&temp))
    }

    pub(crate) fn get_by_uuid(comment_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<Self> {
        discussion_comment_list::discussion_comment_list
            .filter(discussion_comment_list::uuid.eq(comment_uuid))
            .first::<DiscussionCommentList>(conn)
            .map_err(|err| {
                debug!("Failed get discussion comment data: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    pub(crate) fn replies_count(
        comment_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i64> {
        discussion_comment_list::discussion_comment_list
            .filter(
                discussion_comment_list::parent_comment_uuid
                    .eq(comment_uuid)
                    .and(discussion_comment_list::uuid.ne(comment_uuid)),
            )
            .count()
            .get_result(conn)
            .map_err(|err| {
                debug!("Failed get comments-replies count: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
