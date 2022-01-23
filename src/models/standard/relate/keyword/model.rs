use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::standard::model::Standard;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(standard_uuid, keyword_id)]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[belongs_to(Keyword, foreign_key = "keyword_id")]
#[table_name = "keyword_to_standard"]
pub(crate) struct StandardKeyword {
    pub(crate) standard_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub(crate) struct StandardKeywordRelatedData {
    pub(crate) keyword: Keyword,
    pub(crate) standard_uuid: Uuid,
}

// impl StandardKeywordRelatedData {
//     /// Create struct with StandardKeyword data, Keyword data set default
//     pub(crate) fn new(standard_uuid: &Uuid) -> Self {
//         Self{
//             keyword: Default::default(),
//             standard_uuid: *standard_uuid,
//         }
//     }
//
//     /// Change keyword data
//     pub(crate) fn put_keyword(&mut self, keyword: Keyword) {
//         self.keyword = keyword;
//     }
// }

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_standard"]
pub(crate) struct InsertableStandardKeyword {
    pub(crate) standard_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardKeywordsData {
    pub(crate) standard_uuid: Uuid,
    pub(crate) keyword_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardKeywordsNames {
    pub(crate) standard_uuid: Uuid,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardKeywordsArg {
    pub(crate) standard_uuid:  Uuid,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct StandardKeywordsArg {
    pub(crate) standard_uuid:  Uuid,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptStandardKeywordsArg> for StandardKeywordsArg {
    fn from(data: IptStandardKeywordsArg) -> Self {
        let IptStandardKeywordsArg {
            standard_uuid,
            limit,
            offset,
        } = data;

        Self {
            standard_uuid,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
