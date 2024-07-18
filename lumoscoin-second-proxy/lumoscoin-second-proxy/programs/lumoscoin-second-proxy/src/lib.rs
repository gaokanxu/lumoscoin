use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use lumoscoin_first_proxy::cpi::accounts::{ProxyInitialize, ProxyIncrement};
use lumoscoin_first_proxy::program::LumoscoinFirstProxy;

declare_id!("SecondProxyContractID");

#[program]
pub mod lumoscoin_second_proxy {
    use super::*;

    pub fn second_proxy_initialize(ctx: Context<SecondProxyInitialize>, initial_count: u64) -> ProgramResult {
        let first_proxy_program = ctx.accounts.first_proxy_program.to_account_info();
        let cpi_accounts = ProxyInitialize {
            counter: ctx.accounts.counter.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            logic_program: ctx.accounts.logic_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(first_proxy_program, cpi_accounts);
        lumoscoin_first_proxy::cpi::proxy_initialize(cpi_ctx, initial_count)
    }

    pub fn second_proxy_increment(ctx: Context<SecondProxyIncrement>) -> ProgramResult {
        let first_proxy_program = ctx.accounts.first_proxy_program.to_account_info();
        let cpi_accounts = ProxyIncrement {
            counter: ctx.accounts.counter.to_account_info(),
            logic_program: ctx.accounts.logic_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(first_proxy_program, cpi_accounts);
        lumoscoin_first_proxy::cpi::proxy_increment(cpi_ctx)
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
    pub first_proxy_program: Program<'info, LumoscoinFirstProxy>,
}

#[derive(Accounts)]
pub struct SecondProxyIncrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, lumoscoin_logic::Counter>,
    pub logic_program: Program<'info, LumoscoinLogic>,
    pub first_proxy_program: Program<'info, LumoscoinFirstProxy>,
}

