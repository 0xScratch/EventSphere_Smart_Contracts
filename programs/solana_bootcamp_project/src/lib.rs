mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;

declare_id!("97MbJZuRnS8WFsT3AwC9eQgJNKuCKEyEopyPnhoPuCs2");

#[program]
pub mod solana_bootcamp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn purchase_tickets(ctx: Context<PurchaseTickets>, quantity: u32) -> Result<()> {
        instructions::purchase::handler(ctx, quantity)
    }

    pub fn get_user_tickets(ctx: Context<GetUserTickets>) -> Result<()> {
        instructions::get_tickets::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
