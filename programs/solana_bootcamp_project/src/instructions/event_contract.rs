use crate::state::*;
use anchor_lang::prelude::*;

pub fn proccess_create_event(
    ctx: Context<CreateEventContract>,
    organizer: Pubkey,
    name: String,
    description: String,
    location: String,
    date: i64,
    ticket_quantity: u32,
    ticket_price: u64,
    tickets_minted: u32,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    event.organizer = organizer;
    event.name = name;
    event.description = description;
    event.location = location;
    event.date = date;
    event.ticket_quantity = ticket_quantity;
    event.ticket_price = ticket_price;
    event.tickets_minted = tickets_minted;

    // Mint
    // let cpi_ctx = CpiContext::new(
    //     ctx.accounts.token_program.to_account_info(),
    //     token::MintTo {
    //         mint: ctx.accounts.sbt_mint.to_account_info(),
    //         to: ctx.accounts.user.to_account_info(),
    //         authority: ctx.accounts.sbt_mint.to_account_info(),
    //     },
    // );
    // token::mint_to(cpi_ctx, 1)?; // Mint 1 SBT to the creator's token account

    Ok(())
}

#[derive(Accounts)]
pub struct CreateEventContract<'info> {
    #[account(init,payer=user,space=8+32+8+16+16+16+16+16)]
    pub event: Account<'info, EventContract>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[account(address = anchor_spl::token::ID)]
// pub token_program: Program<'info, Token>, // Program account for token operations

// #[account(mut)]
// pub sbt_mint: Account<'info, Mint>, // The Soulbound Token mint account
