use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use lumoscoin_second_proxy::cpi::accounts::{SecondProxyInitialize, SecondProxyIncrement};
use lumoscoin_second_proxy::program::LumoscoinSecondProxy;

declare_id!("FirstProxyContractID");

#[program]
pub mod lumoscoin_first_proxy {
    use super::*;

    pub fn proxy_initialize(ctx: Context<ProxyInitialize>, initial_count: u64) -> ProgramResult {
        let second_proxy_program = ctx.accounts.second_proxy_program.to_account_info();
        let cpi_accounts = SecondProxyInitialize {
            counter: ctx.accounts.counter.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            logic_program: ctx.accounts.logic_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(second_proxy_program, cpi_accounts);
        lumoscoin_second_proxy::cpi::second_proxy_initialize(cpi_ctx, initial_count)
    }

    pub fn proxy_increment(ctx: Context<ProxyIncrement>) -> ProgramResult {
        let second_proxy_program = ctx.accounts.second_proxy_program.to_account_info();
        let cpi_accounts = SecondProxyIncrement {
            counter: ctx.accounts.counter.to_account_info(),
            logic_program: ctx.accounts.logic_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(second_proxy_program, cpi_accounts);
        lumoscoin_second_proxy::cpi::second_proxy_increment(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct ProxyInitialize<'info> {
    #[account(mut)]
    pub counter: Account<'info, lumoscoin_logic::Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub logic_program: Program<'info, LumoscoinLogic>,
    pub second_proxy_program: Program<'info, LumoscoinSecondProxy>,
}

#[derive(Accounts)]
pub struct ProxyIncrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, lumoscoin_logic::Counter>,
    pub logic_program: Program<'info, LumoscoinLogic>,
    pub second_proxy_program: Program<'info, LumoscoinSecondProxy>,
}

