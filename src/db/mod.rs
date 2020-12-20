use serde::{Deserialize, Serialize};
use serenity::model::id::{UserId, MessageId, ChannelId};

pub mod sled;

pub trait DB: Sync+Send {
    fn get_user(&self, user_id: UserId) -> User;

    fn get_answer(&self, answer_id: AnswerId) -> Answer;
    
    fn get_question(&self, question_id: QuestionId) -> Question;

    fn get_answers_for_question(&self, question_id: QuestionId) -> Vec<Answer>;

    fn get_message_info(&self, message_id: MessageId) -> MessageInfo;

    fn get_channel_info(&self, channel_id: ChannelId) -> ChannelInfo;
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct AnswerId(u64);

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct QuestionId(u64);

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct User {

}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Answer {

}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Question {

}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct MessageInfo {
    question: Option<QuestionId>,
    answer: Option<AnswerId>
    // Probably enum instead honestly
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ChannelInfo {
    question: Option<QuestionId>
    // I wasn't sure if I want anything else to go here, but I thought I'd leave space
}