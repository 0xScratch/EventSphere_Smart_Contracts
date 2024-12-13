use anchor_lang::prelude::*;
use crate::state::*;





pub fn handler(ctx: Context<GetUserTickets>) -> Result<()> {
    // Implementation
    Ok(())
}


#[derive(Accounts)]
pub struct GetUserTickets<'info> {
    pub user: Signer<'info>,
    pub event: Account<'info, EventContract>,
}