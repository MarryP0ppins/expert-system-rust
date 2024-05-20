use crate::schema::systems;
use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    answer::Answer,
    attribute::Attribute,
    attribute_value::AttributeValue,
    clause::Clause,
    object::Object,
    object_attribute_attributevalue::ObjectAttributeAttributevalue,
    question::{Question, QuestionWithAnswers},
    rule::{Rule, RuleWithClausesAndEffects},
    rule_attribute_attributevalue::RuleAttributeAttributeValue,
    rule_question_answer::RuleQuestionAnswer,
};

#[derive(Queryable, Serialize, Deserialize, Identifiable, ToSchema, Clone, Debug)]
#[diesel(table_name=systems)]
pub struct System {
    pub id: i32,
    pub user_id: i32,
    pub about: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub private: bool,
    pub image_uri: Option<String>,
}

#[derive(Queryable, Serialize, Clone, ToSchema)]
pub struct SystemsWithPageCount {
    pub systems: Vec<System>,
    pub pages: i64,
}

#[derive(Queryable, ToSchema, TryFromMultipart)]
pub struct NewSystemMultipart {
    pub about: Option<String>,
    pub name: String,
    #[schema(value_type = String, format = Binary)]
    #[form_data(limit = "1MiB")]
    pub image: Option<FieldData<Bytes>>,
    pub private: bool,
}

#[derive(Queryable, Insertable, Deserialize, ToSchema)]
#[diesel(table_name=systems)]
pub struct NewSystem {
    pub user_id: i32,
    pub about: Option<String>,
    pub name: String,
    pub image_uri: Option<String>,
    pub private: bool,
}

#[derive(Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name=systems)]
pub struct UpdateSystem {
    pub about: Option<String>,
    pub name: Option<String>,
    pub image_uri: Option<String>,
    pub private: Option<bool>,
}

#[derive(ToSchema, TryFromMultipart, Debug)]
pub struct UpdateSystemMultipart {
    pub about: Option<String>,
    pub name: Option<String>,
    #[schema(value_type = Option<String>, format = Binary)]
    #[form_data(limit = "1MiB")]
    pub image: Option<FieldData<Bytes>>,
    pub private: Option<bool>,
    pub is_image_removed: Option<bool>,
}

#[derive(Queryable, Serialize, ToSchema, Clone)]
pub struct SystemData {
    pub questions: Vec<QuestionWithAnswers>,
    pub rules: Vec<RuleWithClausesAndEffects>,
}

#[derive(Deserialize, ToSchema)]
pub struct SystemDelete {
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct SystemBackup {
    pub system: System,                                                      //
    pub objects: Vec<Object>,                                                //
    pub object_attribute_attributevalue: Vec<ObjectAttributeAttributevalue>, //
    pub attributes: Vec<Attribute>,                                          //
    pub attributes_values: Vec<AttributeValue>,                              //
    pub rules: Vec<Rule>,                                                    //
    pub rule_attribute_attributevalue: Vec<RuleAttributeAttributeValue>,     //
    pub clauses: Vec<Clause>,                                                //
    pub questions: Vec<Question>,                                            //
    pub answers: Vec<Answer>,                                                //
    pub rule_question_answer: Vec<RuleQuestionAnswer>,
}
