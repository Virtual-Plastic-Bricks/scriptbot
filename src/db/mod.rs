use serde::{Deserialize, Serialize};

pub mod sled;

pub trait DB: Sync+Send {

}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct AnswerId(u64);

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct QuestionId(u64);