use chrono::NaiveDateTime;

pub struct UserWithoutPassword {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_name: String,
    pub is_superuser: bool,
}
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_name: String,
    pub is_superuser: bool,
    pub password: String,
}

pub struct NewUser {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_superuser: bool,
    pub password: String,
}

pub struct UserLogin {
    pub email: String,
    pub password: String,
}

pub struct UpdateUserResponse {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub new_password: Option<String>,
}

pub struct UpdateUser {
    pub email: Option<String>,
    //pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    //pub is_superuser: Option<bool>,
    pub password: Option<String>,
}
