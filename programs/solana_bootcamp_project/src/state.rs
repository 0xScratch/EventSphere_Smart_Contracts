use anchor_lang::prelude::*;

#[account]
pub struct EventContract {
    pub organizer: Pubkey,
    pub name: String,
    pub description: String,
    pub location: String,
    pub date: i64,
    pub ticket_quantity: u32,
    pub ticket_price: u64,
    pub tickets_minted: u32,
    pub soul_bound_token_mint: String,
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
