use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<PurchaseTickets>, quantity: u32) -> Result<()> {
    require!(quantity <= 5, TicketingError::ExceedsMaxTickets);
    // Implementation
    Ok(())
}

#[derive(Accounts)]
pub struct PurchaseTickets<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, EventAccount>,
    pub system_program: Program<'info, System>,
}
