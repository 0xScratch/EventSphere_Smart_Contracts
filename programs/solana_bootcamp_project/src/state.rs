use anchor_lang::prelude::*;

//TODO: replace with real event account
#[account]
#[derive(Default)]
pub struct EventAccount {
    pub authority: Pubkey,
    pub total_tickets: u32,
    pub available_tickets: u32,
}

#[account]
#[derive(Default)]
pub struct TicketPurchase {
    pub user: Pubkey,
    pub event_id: Pubkey,
    pub token_ids: Vec<Pubkey>,
    pub quantity: u32,
    pub timestamp: i64,
}
