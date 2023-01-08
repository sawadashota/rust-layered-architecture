use paging;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Pagination {
    page_token: Option<paging::Token>,
    limit: Option<usize>,
}

impl From<Pagination> for paging::Paging {
    fn from(value: Pagination) -> Self {
        paging::Paging{
            token: value.page_token.unwrap_or("".to_string()),
            limit: value.limit.unwrap_or(100),
        }
    }
}
