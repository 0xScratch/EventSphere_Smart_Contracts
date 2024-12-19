mod errors;

use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, Token};

declare_id!("CvBnu3xn3yrd2g73iYsBhiGWqMjpvPfiUTGVPpsQGJBN");
use instructions::*;

mod instructions;
mod state;

#[program]
pub mod solana_bootcamp_project {

    use super::*;

    pub fn create_event(
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
        proccess_create_event(
            ctx,
            organizer,
            name,
            description,
            location,
            date,
            ticket_quantity,
            ticket_price,
            tickets_minted,
        )
    }

    pub fn purchase_tickets(ctx: Context<PurchaseTickets>, quantity: u32) -> Result<()> {
        purchase::handler(ctx, quantity)
    }

    pub fn get_user_tickets(ctx: Context<GetUserTickets>) -> Result<()> {
        get_tickets::handler(ctx)
    }
}
