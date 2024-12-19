use crate::state::*;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<GetUserTickets>) -> Result<()> {
    let purchase = &ctx.accounts.ticket_purchase;
    msg!("User: {}", purchase.user);
    msg!("Event: {}", purchase.event_id);
    msg!("Quantity: {}", purchase.quantity);
    msg!("Purchase time: {}", purchase.timestamp);

    Ok(())
}

#[derive(Accounts)]
pub struct GetUserTickets<'info> {
    pub user: Signer<'info>,
    pub event: Account<'info, EventContract>,

    #[account(
        seeds = [
            b"ticket_purchase",
            user.key().as_ref(),
            event.key().as_ref()
        ],
        bump,
        has_one = user,
    )]
    pub ticket_purchase: Account<'info, TicketPurchase>,
}
