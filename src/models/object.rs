use crate::schema::objects;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::system::System;

#[derive(Debug, Queryable, Serialize, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(System))]
#[diesel(table_name=objects)]
pub struct Object {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable, Deserialize)]
#[diesel(table_name=objects)]
pub struct NewObject {
    pub system_id: i32,
    pub name: String
}

#[derive(Debug, Queryable, Deserialize)]
pub struct NewObjectWithAttributesValueIds {
    pub system_id: i32,
    pub name: String,
    pub attributes_value_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, AsChangeset, Clone)]
#[diesel(table_name=objects)]
pub struct UpdateAnswer {
    pub id: i32,
    pub name: String,
}
