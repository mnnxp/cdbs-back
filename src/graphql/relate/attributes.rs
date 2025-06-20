use async_graphql::InputObject;

#[derive(Debug, InputObject)]
pub(crate) struct IptPaginate {
    #[graphql(default = 1)]
    pub(crate) current_page: i32,
    #[graphql(default = 5)]
    pub(crate) per_page: i32,
}

impl Default for IptPaginate {
    fn default() -> Self {
        Self {
            current_page: 1,
            per_page: 5,
        }
    }
}

#[derive(Default, Debug, InputObject)]
pub(crate) struct IptSort {
    pub(crate) by_field: String,
    #[graphql(default = false)]
    pub(crate) as_desc: bool,
}
