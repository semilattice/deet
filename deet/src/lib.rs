//! DEET is a software program for tracking bugs, progress, and requirements.
//!
//! Alongside the [common] module you will find one module for every use case.
//!
//! [common]: common/index.html

extern crate algebra;
extern crate uuid;

#[macro_use] pub mod common;

pub mod assign_ticket_priority;
pub mod assign_ticket_title;
pub mod post_ticket_comment;
pub mod update_ticket_conclusion;

use assign_ticket_priority::AssignTicketPriority;
use assign_ticket_title::AssignTicketTitle;
use common::ticket::TicketId;
use post_ticket_comment::PostTicketComment;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use update_ticket_conclusion::UpdateTicketConclusion;

/// An operation can be thought of as a function that transforms the
/// application state.
///
/// Operations are transferred between clients, and form a bounded semilattice
/// so that they can be combined. The application state can be derived from the
/// result of combining all operations. It is important to realize that the
/// application state exists only in the mind; this crate does not define a
/// corresponding data type. Do not go looking for it.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Operation {
    pub assign_ticket_priority: BTreeMap<TicketId, AssignTicketPriority>,
    pub assign_ticket_title: BTreeMap<TicketId, AssignTicketTitle>,
    pub post_ticket_comment: BTreeMap<TicketId, BTreeSet<PostTicketComment>>,
    pub update_ticket_conclusion: BTreeMap<TicketId, BTreeSet<UpdateTicketConclusion>>,
}

crdt_compound!(Operation, assign_ticket_priority, assign_ticket_title,
post_ticket_comment, update_ticket_conclusion);
