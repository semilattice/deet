use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PostTicketComment {
    pub timestamp: SystemTime,
    pub unique: Uuid,
    pub body: String,
}
