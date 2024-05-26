use super::object_attribute_attributevalue::{
    NewObjectAttributeAttributevalueWithoutObject, ObjectAttributeAttributevalue,
};

pub struct Object {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

pub struct NewObject {
    pub system_id: i32,
    pub name: String,
}

pub struct NewObjectWithAttributesValueIds {
    pub system_id: i32,
    pub name: String,
    pub object_attribute_attributevalue_ids: Vec<NewObjectAttributeAttributevalueWithoutObject>,
}

pub struct ObjectWithAttributesValues {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub object_attribute_attributevalue_ids: Vec<ObjectAttributeAttributevalue>,
}

pub struct UpdateObject {
    pub id: i32,
    pub name: String,
}
