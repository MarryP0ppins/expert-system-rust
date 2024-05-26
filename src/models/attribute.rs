use super::attribute_value::AttributeValue;

pub struct Attribute {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
}

pub struct NewAttribute {
    pub system_id: i32,
    pub name: String,
}

pub struct NewAttributeWithAttributeValuesName {
    pub system_id: i32,
    pub name: String,
    pub values_name: Vec<String>,
}

pub struct UpdateAttribute {
    pub id: i32,
    pub name: String,
}

pub struct AttributeWithAttributeValues {
    pub id: i32,
    pub system_id: i32,
    pub name: String,
    pub values: Vec<AttributeValue>,
}
