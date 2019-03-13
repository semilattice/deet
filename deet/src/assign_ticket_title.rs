use std::time::SystemTime;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssignTicketTitle {
    pub timestamp: SystemTime,
    pub new_title: String,
}

crdt_ord_max!(AssignTicketTitle);
