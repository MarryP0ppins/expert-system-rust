use serde::Deserialize;

#[derive(Deserialize)]
pub struct SystemListPagination {
    pub name: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct HistoryListPagination {
    pub system: Option<i32>,
    pub user: Option<i32>,
}