use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct SystemListPagination {
    pub name: Option<String>,
    pub username: Option<String>,
    #[param(default = json!(0))]
    pub page: Option<i32>,
    #[param(default = json!(20))]
    pub count: Option<i32>,
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
