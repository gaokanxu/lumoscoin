use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use lumoscoin_logic::cpi::accounts::{Initialize, Increment};
use lumoscoin_logic::program::LumoscoinLogic;

declare_id!("SecondProxyContractID");

#[program]
pub mod lumoscoin_second_proxy {
    use super::*;

    pub fn second_proxy_initialize(ctx: Context<SecondProxyInitialize>, initial_count: u64) -> ProgramResult {
        let logic_program = ctx.accounts.logic_program.to_account_info();
        let cpi_accounts = Initialize {
            counter: ctx.accounts.counter.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(logic_program, cpi_accounts);
        lumoscoin_logic::cpi::initialize(cpi_ctx, initial_count)
    }

    pub fn second_proxy_increment(ctx: Context<SecondProxyIncrement>) -> ProgramResult {
        let logic_program = ctx.accounts.logic_program.to_account_info();
        let cpi_accounts = Increment {
            counter: ctx.accounts.counter.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(logic_program, cpi_accounts);
        lumoscoin_logic::cpi::increment(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct SecondProxyInitialize<'info> {
    #[account(mut)]
    pub counter: Account<'info, lumoscoin_logic::Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub logic_program: Program<'info, LumoscoinLogic>,
}

#[derive(Accounts)]
pub struct SecondProxyIncrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, lumoscoin_logic::Counter>,
    pub logic_program: Program<'info, LumoscoinLogic>,
}

