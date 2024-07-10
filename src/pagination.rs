use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams, Debug)]
pub struct SystemListPagination {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub username: Option<String>,
    #[param(default = json!(1))]
    pub page: Option<i32>,
    #[param(default = json!(20))]
    pub per_page: Option<i32>,
    pub all_types: Option<bool>,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct SystemStars {
    pub inc: Option<bool>,
    pub dec: Option<bool>,
}

#[derive(Deserialize, IntoParams)]
pub struct HistoryListPagination {
    pub system: Option<i32>,
    pub user: Option<i32>,
}

#[derive(Deserialize, IntoParams)]
pub struct AnswerListPagination {
    pub question_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct AttributeValueListPagination {
    pub attribute_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct AttributeListPagination {
    pub system_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct ClauseListPagination {
    pub rule_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct ObjectListPagination {
    pub system_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct QuestionListPagination {
    pub system_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct RuleListPagination {
    pub system_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct LikeListPagination {
    pub user_id: i32,
}
