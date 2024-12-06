use anchor_lang::prelude::*;

declare_id!("97MbJZuRnS8WFsT3AwC9eQgJNKuCKEyEopyPnhoPuCs2");

#[program]
pub mod solana_bootcamp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
