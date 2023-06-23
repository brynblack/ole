use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, PartialEq, Serialize)]
pub struct CourseInfo {
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Default, Deserialize, PartialEq, Serialize)]
pub struct LessonInfo {
    pub name: String,
    pub content: String,
    pub image: String,
}

#[derive(Deserialize)]
pub struct NewAcc {
    pub username: String,
    pub password: String,
    pub pfp: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub token: String,
}

#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct AccountData {
    pub pfp: String,
}
