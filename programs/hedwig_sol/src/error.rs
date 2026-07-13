use anchor_lang::prelude::*;

#[error_code]
pub enum HedwigError {
    /// Role name is empty or exceeds the 32-byte limit.
    #[msg("Role name must be 1-32 bytes")]
    InvalidRoleName,

    /// Org name is empty or exceeds the 64-byte limit.
    #[msg("Org name must be 1-64 bytes")]
    InvalidOrgName,

    /// The signer is not the org authority.
    #[msg("Signer is not the org authority")]
    Unauthorized,

    /// The signer is not the role admin.
    #[msg("Signer is not the role admin")]
    NotRoleAdmin,

    /// The role has been disabled and cannot accept new members or pass checks.
    #[msg("Role is disabled")]
    RoleDisabled,

    /// Membership has expired.
    #[msg("Role membership has expired")]
    MembershipExpired,

    /// expires_at must be 0 (never-expires) or strictly in the future.
    #[msg("expires_at must be zero or greater than the current time")]
    InvalidExpiration,

    /// A counter would overflow or underflow on this operation.
    #[msg("Counter overflow or underflow")]
    MathOverflow,
}
