use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<PurchaseTickets>, quantity: u32) -> Result<()> {
    require!(quantity <= 5, TicketingError::ExceedsMaxTickets);
    let event = &mut ctx.accounts.event;
    let user = &ctx.accounts.user;
    require!(quantity <= 5, TicketingError::ExceedsMaxTickets);
    require!(
        event.tickets_minted + quantity <= event.ticket_quantity,
        TicketingError::NoTicketsAvailable
    );
    let purchase = &mut ctx.accounts.ticket_purchase;
    purchase.user = *user.key;
    purchase.event_id = event.key();
    purchase.quantity = quantity;
    purchase.timestamp = Clock::get()?.unix_timestamp;
    event.tickets_minted = event
        .tickets_minted
        .checked_add(quantity)
        .ok_or(TicketingError::InvalidQuantity)?;
    Ok(())
}

#[derive(Accounts)]
pub struct PurchaseTickets<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub event: Account<'info, EventContract>,

    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<TicketPurchase>(),
        seeds = [
            b"ticket_purchase",
            user.key().as_ref(),
            event.key().as_ref()
        ],
        bump
    )]
    pub ticket_purchase: Account<'info, TicketPurchase>,

    pub system_program: Program<'info, System>,
}
