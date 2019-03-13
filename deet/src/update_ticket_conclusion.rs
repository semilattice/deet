use uuid::Uuid;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UpdateTicketConclusion {
    pub unique: Uuid,
    // TODO: Represent text transformations.
}
