pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub body: String,
}

pub struct NewAnswer {
    pub question_id: i32,
    pub body: String,
}

pub struct UpdateAnswer {
    pub id: i32,
    pub body: String,
}
