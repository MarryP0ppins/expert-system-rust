pub struct ObjectAttributeAttributevalue {
    pub id: i32,
    pub object_id: i32,
    pub attribute_value_id: i32,
    pub attribute_id: i32,
}

pub struct NewObjectAttributeAttributevalue {
    pub object_id: i32,
    pub attribute_id: i32,
    pub attribute_value_id: i32,
}

pub struct NewObjectAttributeAttributevalueWithoutObject {
    pub attribute_id: i32,
    pub attribute_value_id: i32,
}
