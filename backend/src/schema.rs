use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFeedbackSchema {
    pub text: String,
    pub rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFeedbackSchema {
    pub text: Option<String>,
    pub rating: Option<i32>,
}
