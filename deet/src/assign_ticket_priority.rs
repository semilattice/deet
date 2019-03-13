use common::ticket::Priority;
use std::time::SystemTime;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssignTicketPriority {
    pub timestamp: SystemTime,
    pub new_priority: Priority,
}

crdt_ord_max!(AssignTicketPriority);
