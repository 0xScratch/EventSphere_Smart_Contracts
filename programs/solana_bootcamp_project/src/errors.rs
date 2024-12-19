use anchor_lang::prelude::*;

#[error_code]
pub enum TicketingError {
    #[msg("Cannot purchase more than 5 tickets")]
    ExceedsMaxTickets,
    #[msg("No tickets available")]
    NoTicketsAvailable,
    #[msg("Invalid quantity")]
    InvalidQuantity,
}
