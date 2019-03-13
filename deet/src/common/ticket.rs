use uuid::Uuid;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TicketId(pub Uuid);

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Priority {
    Priority1,
    Priority2,
    Priority3,
    Priority4,
    Priority5,
}
