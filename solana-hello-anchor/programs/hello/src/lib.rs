use anchor_lang::prelude::*;

declare_id!("FnqZvYUJKQf18JupjXhUTiyHCJrQs6xSgtA63MbMxD9c");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
