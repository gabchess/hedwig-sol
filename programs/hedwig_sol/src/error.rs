use anchor_lang::prelude::*;

#[error_code]
pub enum HedwigError {
    /// Role name exceeds the 32-byte limit.
    #[msg("Role name must be 1-32 bytes")]
    InvalidRoleName,

    /// Org name exceeds the 64-byte limit.
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

    /// The member account already exists for this holder and role.
    #[msg("Holder already has this role")]
    AlreadyAssigned,

    /// Membership has expired.
    #[msg("Role membership has expired")]
    MembershipExpired,
}
