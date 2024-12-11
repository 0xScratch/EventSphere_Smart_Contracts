use anchor_lang::prelude::*;

#[account]
pub struct Event {
    organizer: Pubkey,
    name: String,
    description: String,
    location: String,
    date: i64,
    ticket_quantity: u32,
    ticket_price: u64,
    tickets_minted: u32,
    soul_bound_token_mint: Pubkey,
}
