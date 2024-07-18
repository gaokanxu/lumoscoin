use anchor_lang::prelude::*;

declare_id!("LogicContractID");

#[program]
pub mod lumoscoin_logic {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_count: u64) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count = initial_count;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

