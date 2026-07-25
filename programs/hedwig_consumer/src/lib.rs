use anchor_lang::prelude::*;

declare_id!("52D3pTYvMwLYbiigY5xg55n4HmtEzTKCEicx1Cojzo9a");

pub const COUNTER_SEED: &[u8] = b"counter";

#[program]
pub mod hedwig_consumer {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.required_role = ctx.accounts.required_role.key();
        counter.value = 0;
        counter.bump = ctx.bumps.counter;
        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let cpi_accounts = hedwig_sol::cpi::accounts::CheckRole {
            member: ctx.accounts.member.to_account_info(),
            role: ctx.accounts.required_role.to_account_info(),
            holder: ctx.accounts.authority.to_account_info(),
        };
        hedwig_sol::cpi::check_role(CpiContext::new(
            ctx.accounts.hedwig_program.key(),
            cpi_accounts,
        ))?;

        let counter = &mut ctx.accounts.counter;
        counter.value = counter
            .value
            .checked_add(1)
            .ok_or(ConsumerError::CounterOverflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(
        init,
        payer = authority,
        space = Counter::LEN,
        seeds = [COUNTER_SEED, authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub required_role: Account<'info, hedwig_sol::Role>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut,
        seeds = [COUNTER_SEED, authority.key().as_ref()],
        bump = counter.bump,
        has_one = authority,
        has_one = required_role,
    )]
    pub counter: Account<'info, Counter>,

    pub authority: Signer<'info>,

    pub member: Account<'info, hedwig_sol::Member>,

    pub required_role: Account<'info, hedwig_sol::Role>,

    pub hedwig_program: Program<'info, hedwig_sol::program::HedwigSol>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub required_role: Pubkey,
    pub value: u64,
    pub bump: u8,
}

impl Counter {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
}

#[error_code]
pub enum ConsumerError {
    #[msg("Counter overflow")]
    CounterOverflow,
}
