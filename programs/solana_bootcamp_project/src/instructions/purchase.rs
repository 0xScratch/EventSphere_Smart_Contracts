use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;

pub fn handler(ctx: Context<PurchaseTickets>, quantity: u32) -> Result<()> {
    require!(quantity <= 5, TicketingError::ExceedsMaxTickets);
    let event = &mut ctx.accounts.event;
    let user = &ctx.accounts.user;
    require!(quantity <= 5, TicketingError::ExceedsMaxTickets);
    require!(
        event.tickets_minted + quantity <= event.ticket_quantity,
        TicketingError::NoTicketsAvailable
    );
    let payment_amount = event
        .ticket_price
        .checked_mul(quantity as u64)
        .ok_or(TicketingError::InvalidQuantity)?;

    // Transfer SOL from user to event organizer
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &event.organizer,
            payment_amount,
        ),
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.event_organizer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
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

    /// CHECK: This is the event organizer's account that receives payment
    #[account(
        mut,
        constraint = event_organizer.key() == event.organizer
    )]
    pub event_organizer: AccountInfo<'info>,

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