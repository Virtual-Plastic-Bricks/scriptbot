use super::*;
// I called this module sled and made the module public. Whoops.
use ::sled::{Tree, Db};
use std::path::Path;

pub struct SledDB {
    db: Db,
    questions: Tree,
    answers: Tree,
    users: Tree,
    channels: Tree
}

impl SledDB {
    pub fn new(db_path: impl AsRef<Path>) -> Self {
        let db = ::sled::open(db_path).expect("What the fuck.");
        let question_tree = db.open_tree("Question").expect("Failed to open Question tree");
        let answer_tree = db.open_tree("Answer").expect("Failed to open Answer tree");
        let channel_tree = db.open_tree("Channel").expect("Failed to open Channel tree");
        let user_tree = db.open_tree("User").expect("Failed to open User tree");

        SledDB {
            db,
            questions: question_tree,
            answers: answer_tree,
            users: user_tree,
            channels: channel_tree
        }
    }
}

impl DB for SledDB {

}