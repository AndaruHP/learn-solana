use anchor_lang::prelude::*;

declare_id!("A7DDqXWWR8nDXQEfm7nViBX84N8RFQmrRa3hnDrkqBge");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
