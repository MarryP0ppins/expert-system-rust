use axum::body::Bytes;
use axum_typed_multipart::FieldData;
use chrono::NaiveDateTime;

use super::{
    answer::Answer, attribute::Attribute, attribute_value::AttributeValue, clause::Clause,
    object::Object, object_attribute_attributevalue::ObjectAttributeAttributevalue,
    question::Question, rule::Rule, rule_attribute_attributevalue::RuleAttributeAttributeValue,
    rule_question_answer::RuleQuestionAnswer,
};

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

pub struct SystemsWithPageCount {
    pub systems: Vec<System>,
    pub pages: i64,
}

pub struct NewSystemMultipart {
    pub about: Option<String>,
    pub name: String,
    // #[schema(value_type = String, format = Binary)]
    //#[form_data(limit = "1MiB")]
    pub image: Option<FieldData<Bytes>>,
    pub private: bool,
}

pub struct NewSystem {
    pub user_id: i32,
    pub about: Option<String>,
    pub name: String,
    pub image_uri: Option<String>,
    pub private: bool,
}

pub struct UpdateSystem {
    pub about: Option<String>,
    pub name: Option<String>,
    pub image_uri: Option<String>,
    pub private: Option<bool>,
}

pub struct UpdateSystemMultipart {
    pub about: Option<String>,
    pub name: Option<String>,
    //#[schema(value_type = Option<String>, format = Binary)]
    //#[form_data(limit = "1MiB")]
    pub image: Option<FieldData<Bytes>>,
    pub private: Option<bool>,
    pub is_image_removed: Option<bool>,
}

pub struct SystemDelete {
    pub password: String,
}

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
