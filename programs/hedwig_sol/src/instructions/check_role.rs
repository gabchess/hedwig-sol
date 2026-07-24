use anchor_lang::prelude::*;

use crate::{
    constants::MEMBER_SEED,
    error::HedwigError,
    state::{Member, Role},
};

/// Verifies that a holder currently holds a given role.
///
/// This instruction is the CPI target for other Solana programs that need
/// to gate access on role membership. It performs three checks:
///   1. The member PDA is validly derived (Anchor constraint).
///   2. The role is enabled.
///   3. If expires_at is set, membership has not lapsed.
///
/// On success this is a no-op (it does not mutate state). On failure it
/// returns an error the calling program can inspect.
///
/// CPI pattern for a consuming program:
/// ```ignore
/// hedwig_sol::cpi::check_role(
///     CpiContext::new(
///         ctx.accounts.hedwig_program.key(),
///         hedwig_sol::cpi::accounts::CheckRole {
///             member: ctx.accounts.member.to_account_info(),
///             role: ctx.accounts.role.to_account_info(),
///             holder: ctx.accounts.actor.to_account_info(),
///         },
///     ),
/// )?;
/// ```
pub fn handler(ctx: Context<CheckRole>) -> Result<()> {
    let role = &ctx.accounts.role;
    require!(role.enabled, HedwigError::RoleDisabled);

    let member = &ctx.accounts.member;
    if member.expires_at != 0 {
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp <= member.expires_at,
            HedwigError::MembershipExpired
        );
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CheckRole<'info> {
    #[account(
        seeds = [MEMBER_SEED, role.key().as_ref(), holder.key().as_ref()],
        bump = member.bump,
        has_one = role,
        has_one = holder,
    )]
    pub member: Account<'info, Member>,

    pub role: Account<'info, Role>,

    /// CHECK: Member seeds and `has_one = holder` bind this pubkey to the
    /// membership being checked. No signature is required because this
    /// instruction verifies membership, not control of the holder key.
    pub holder: UncheckedAccount<'info>,
}
