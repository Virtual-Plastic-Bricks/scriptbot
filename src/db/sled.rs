use super::*;
// I called this module sled and made the module public. Whoops.
use ::sled::{Tree, Db};
use std::path::Path;
use serenity::model::id::{UserId, MessageId, ChannelId};
use rmp_serde::{Deserializer, Serializer, from_read as mp_from};
use std::convert::TryInto;

pub struct SledDB {
    db: Db,
    questions: Tree,
    answers: Tree,
    users: Tree,
    channels: Tree,
    messages: Tree,
}

impl SledDB {
    pub fn new(db_path: impl AsRef<Path>) -> Self {
        let db = ::sled::open(db_path).expect("What the fuck.");
        let question_tree = db.open_tree("Question").expect("Failed to open Question tree");
        let answer_tree = db.open_tree("Answer").expect("Failed to open Answer tree");
        let channel_tree = db.open_tree("Channel").expect("Failed to open Channel tree");
        let user_tree = db.open_tree("User").expect("Failed to open User tree");
        let message_tree = db.open_tree("Message").expect("Failed to open Message tree");

        SledDB {
            db,
            questions: question_tree,
            answers: answer_tree,
            users: user_tree,
            channels: channel_tree,
            messages: message_tree
        }
    }
}

macro_rules! parse_simple {
    ($e:expr) => {
        $e.unwrap().map(|data|mp_from(&*data).unwrap()).unwrap_or_default()
    };
}

impl DB for SledDB {
    fn get_user(&self, user: UserId) -> User {
        User {
            id: user,
            ..parse_simple!( self.users.get(user.key()))
        }
    }
    fn get_answer(&self, answer: AnswerId) -> Answer {
        Answer {
            id: answer,
            ..parse_simple!(self.answers.get(answer.key()))
        }
    }
    fn get_question(&self, question: QuestionId) -> Question {
        Question {
            id: question,
            ..parse_simple!(self.questions.get(question.key()))
        }
    }
    fn get_answers_for_question(&self, question: QuestionId) -> Vec<Answer> {
        self.questions.scan_prefix(question.key()).map(|answer| {
            let (answer_key, answer_data) = answer.unwrap();
            Answer {
                id: answer_id_from_key(&answer_key),
                ..mp_from(&*answer_data).unwrap() // I have written a fucking bomb.
            }
        }).collect()
    }
    fn get_message_info(&self, message: MessageId) -> MessageInfo {
        // Getting message info was supposed to be simple because I could check if Q(messageId) or
        // A(messageId) but then I changed AnswerId to be (QuestionId, u64) as it should be and
        // honestly my life is over because now I really do have to maintain a secondary index.
        MessageInfo {
            id: message,
            ..parse_simple!(self.messages.get(message.key()))
        }
    }
    fn get_channel_info(&self, channel: ChannelId) -> ChannelInfo {
        ChannelInfo {
            id: channel,
            ..parse_simple!(self.channels.get(channel.key()))
        }
    }
}

// Trait aliases don't exist and I don't feel like creating a dummy trait with blanket impl to allow
// `fn key<T: Key>(&self) -> T;` despite how clean it would make the signature for further impl
trait SledKey {
    type T: AsRef<[u8]>;
    fn key(&self) -> Self::T;
}

macro_rules! key_simple {
    ($t:ty) => {
        impl SledKey for $t {
            type T = [u8; 8];
            fn key(&self) -> [u8;8] {
                self.0.to_be_bytes()
            }
        }
    };
    ($t:ty, $($t2:ty),+) => {key_simple!($t); key_simple!($($t2),+);}
}

key_simple!(UserId, ChannelId, MessageId, QuestionId);

impl SledKey for AnswerId {
    type T = [u8; 16];
    fn key(&self) -> [u8; 16] {
        (((self.0.0 as u128) << 8) & (self.1 as u128)).to_be_bytes()
        // I couldn't find a more obvious way to get [u8;16] out of [[u8;8];2]
        // Other than maybe mem::transmute
        // But unsafe.
    }
}

fn answer_id_from_key(key: &[u8]) -> AnswerId {
    let (question_slice, answer_slice) = key.split_at(8);
    AnswerId(
        QuestionId(u64::from_be_bytes(question_slice.try_into().unwrap())),
        u64::from_be_bytes(answer_slice.try_into().unwrap())
    )
}

// There is not a way that I'm aware of to produce a macro which can template the format
/*
impl SledKey for $type {
    fn key<T: AsRef<[u8]>>(&self) -> T {
        $expr_using_self
    }
}
*/
// Because there is no way to capture the `self` declared inside of the macro