pub struct AttributeValue {
    pub id: i32,
    pub attribute_id: i32,
    pub value: String,
}

pub struct NewAttributeValue {
    pub attribute_id: i32,
    pub value: String,
}

pub struct UpdateAttributeValue {
    pub id: i32,
    pub value: String,
}
