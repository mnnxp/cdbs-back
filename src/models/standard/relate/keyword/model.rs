use crate::schema::*;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword standard models
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = keyword_to_standard)]
pub(crate) struct StandardKeyword {
    pub(crate) standard_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = keyword_to_standard)]
pub(crate) struct InsertableStandardKeyword {
    pub(crate) standard_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

/// Data for adding keywords (tags) to the standard by identifiers
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardKeywordsData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// Keyword identifiers (list)
    pub(crate) keyword_ids: Vec<i32>,
}

/// Data for adding/removing keywords (tags) to a standard by name
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardKeywordsNames {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// Keywords (list)
    pub(crate) keywords: Vec<String>,
}

impl From<&IptStandardKeywordsData> for Vec<InsertableStandardKeyword> {
    fn from(ipt_data: &IptStandardKeywordsData) -> Vec<InsertableStandardKeyword> {
        let IptStandardKeywordsData {
            standard_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each keyword
        for keyword_id in keyword_ids {
            if keyword_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableStandardKeyword {
                    standard_uuid: *standard_uuid,
                    keyword_id: *keyword_id,
                })
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DeleteStandardKeywords {
    pub(crate) standard_uuid: Uuid,
    pub(crate) keyword_ids: Vec<i32>,
}

impl From<&IptStandardKeywordsData> for DeleteStandardKeywords {
    fn from(ipt_data: &IptStandardKeywordsData) -> Self {
        let IptStandardKeywordsData {
            standard_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut good_keyword_ids: Vec<i32> = Vec::new();
        // filter bad keywords id
        for keyword_id in keyword_ids {
            if keyword_id > &0 {
                good_keyword_ids.push(*keyword_id)
            }
        }

        Self{
            standard_uuid: *standard_uuid,
            keyword_ids: good_keyword_ids,
        }
    }
}