use super::answer::Answer;

pub struct Question {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

pub struct NewQuestion {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
}

pub struct NewQuestionWithAnswersBody {
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers_body: Vec<String>,
}
pub struct UpdateQuestion {
    pub id: i32,
    pub body: Option<String>,
    pub with_chooses: Option<bool>,
}

pub struct QuestionWithAnswers {
    pub id: i32,
    pub system_id: i32,
    pub body: String,
    pub with_chooses: bool,
    pub answers: Vec<Answer>,
}
