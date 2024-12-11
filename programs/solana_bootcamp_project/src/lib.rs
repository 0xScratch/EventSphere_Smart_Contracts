use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token};

declare_id!("97MbJZuRnS8WFsT3AwC9eQgJNKuCKEyEopyPnhoPuCs2");

#[program]
pub mod solana_bootcamp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_event(
        ctx: Context<CreateEvent>,
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
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.sbt_mint.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.sbt_mint.to_account_info(),
            },
        );
        token::mint_to(cpi_ctx, 1)?; // Mint 1 SBT to the creator's token account

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init,payer=user,space=8+32+104+804)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>, // Program account for token operations

    #[account(mut)]
    pub sbt_mint: Account<'info, Mint>, // The Soulbound Token mint account
}

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
