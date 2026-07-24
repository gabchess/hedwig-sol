/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/hedwig_sol.json`.
 */
export type HedwigSol = {
  address: "H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC";
  metadata: {
    name: "hedwigSol";
    version: "0.1.0";
    spec: "0.1.0";
    description: "Composable onchain roles for Solana programs";
  };
  docs: [
    "Hedwig: onchain roles primitive for Solana.",
    "",
    "Grant roles, not keys."
  ];
  instructions: [
    {
      name: "assignRole";
      docs: ["Assign a role to a holder pubkey."];
      discriminator: [255, 174, 125, 180, 203, 155, 202, 131];
      accounts: [
        {
          name: "member";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [109, 101, 109, 98, 101, 114];
              },
              {
                kind: "account";
                path: "role";
              },
              {
                kind: "account";
                path: "holder";
              }
            ];
          };
        },
        {
          name: "role";
          writable: true;
        },
        {
          name: "holder";
          docs: [
            "No signature required: the admin signs on the holder's behalf."
          ];
        },
        {
          name: "admin";
          writable: true;
          signer: true;
          relations: ["role"];
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [
        {
          name: "expiresAt";
          type: "i64";
        }
      ];
    },
    {
      name: "checkRole";
      docs: ["Verify that a holder currently holds a role (CPI-queryable)."];
      discriminator: [142, 221, 97, 79, 34, 70, 95, 203];
      accounts: [
        {
          name: "member";
          pda: {
            seeds: [
              {
                kind: "const";
                value: [109, 101, 109, 98, 101, 114];
              },
              {
                kind: "account";
                path: "role";
              },
              {
                kind: "account";
                path: "holder";
              }
            ];
          };
        },
        {
          name: "role";
          relations: ["member"];
        },
        {
          name: "holder";
          docs: ["required; the check is read-only."];
          relations: ["member"];
        }
      ];
      args: [];
    },
    {
      name: "createOrg";
      docs: ["Create a new organization (top-level role namespace)."];
      discriminator: [48, 115, 187, 249, 36, 3, 186, 175];
      accounts: [
        {
          name: "org";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [111, 114, 103];
              },
              {
                kind: "account";
                path: "authority";
              }
            ];
          };
        },
        {
          name: "authority";
          writable: true;
          signer: true;
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [
        {
          name: "name";
          type: "string";
        }
      ];
    },
    {
      name: "createRole";
      docs: ["Create a named role under an org."];
      discriminator: [170, 147, 127, 223, 222, 112, 205, 163];
      accounts: [
        {
          name: "role";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [114, 111, 108, 101];
              },
              {
                kind: "account";
                path: "org";
              },
              {
                kind: "arg";
                path: "name";
              }
            ];
          };
        },
        {
          name: "org";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [111, 114, 103];
              },
              {
                kind: "account";
                path: "authority";
              }
            ];
          };
        },
        {
          name: "authority";
          writable: true;
          signer: true;
          relations: ["org"];
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [
        {
          name: "name";
          type: "string";
        }
      ];
    },
    {
      name: "revokeRole";
      docs: ["Revoke a role from a holder, closing the member PDA."];
      discriminator: [179, 232, 2, 180, 48, 227, 82, 7];
      accounts: [
        {
          name: "member";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [109, 101, 109, 98, 101, 114];
              },
              {
                kind: "account";
                path: "role";
              },
              {
                kind: "account";
                path: "member.holder";
                account: "member";
              }
            ];
          };
        },
        {
          name: "role";
          writable: true;
          relations: ["member"];
        },
        {
          name: "admin";
          writable: true;
          signer: true;
          relations: ["role"];
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [];
    },
    {
      name: "setRoleEnabled";
      docs: [
        "Enable or disable a role. Circuit breaker: halts `check_role` and new",
        "`assign_role` calls for the whole role without revoking individual",
        "members. Only the role admin can call this."
      ];
      discriminator: [51, 2, 28, 221, 248, 220, 73, 20];
      accounts: [
        {
          name: "role";
          writable: true;
        },
        {
          name: "admin";
          signer: true;
          relations: ["role"];
        }
      ];
      args: [
        {
          name: "enabled";
          type: "bool";
        }
      ];
    }
  ];
  accounts: [
    {
      name: "member";
      discriminator: [54, 19, 162, 21, 29, 166, 17, 198];
    },
    {
      name: "org";
      discriminator: [33, 88, 128, 218, 37, 86, 39, 107];
    },
    {
      name: "role";
      discriminator: [46, 219, 197, 24, 233, 249, 253, 154];
    }
  ];
  events: [
    {
      name: "orgCreated";
      discriminator: [147, 145, 192, 255, 185, 216, 83, 7];
    },
    {
      name: "roleAssigned";
      discriminator: [15, 207, 225, 171, 169, 117, 98, 131];
    },
    {
      name: "roleCreated";
      discriminator: [203, 8, 94, 252, 142, 13, 51, 221];
    },
    {
      name: "roleEnabledSet";
      discriminator: [22, 242, 208, 155, 200, 121, 235, 36];
    },
    {
      name: "roleRevoked";
      discriminator: [167, 183, 52, 229, 126, 206, 62, 61];
    }
  ];
  errors: [
    {
      code: 6000;
      name: "invalidRoleName";
      msg: "Role name must be 1-32 bytes";
    },
    {
      code: 6001;
      name: "invalidOrgName";
      msg: "Org name must be 1-64 bytes";
    },
    {
      code: 6002;
      name: "unauthorized";
      msg: "Signer is not the org authority";
    },
    {
      code: 6003;
      name: "notRoleAdmin";
      msg: "Signer is not the role admin";
    },
    {
      code: 6004;
      name: "roleDisabled";
      msg: "Role is disabled";
    },
    {
      code: 6005;
      name: "membershipExpired";
      msg: "Role membership has expired";
    },
    {
      code: 6006;
      name: "invalidExpiration";
      msg: "expires_at must be zero or greater than the current time";
    },
    {
      code: 6007;
      name: "mathOverflow";
      msg: "Counter overflow or underflow";
    }
  ];
  types: [
    {
      name: "member";
      docs: [
        "Membership record: proof that a pubkey holds a given role.",
        'PDA seeds: ["member", role_key, holder]'
      ];
      type: {
        kind: "struct";
        fields: [
          {
            name: "role";
            docs: ["The role PDA this membership is for."];
            type: "pubkey";
          },
          {
            name: "holder";
            docs: [
              "The pubkey (wallet, program, or agent key) holding this role."
            ];
            type: "pubkey";
          },
          {
            name: "grantedAt";
            docs: ["Unix timestamp when this membership was granted."];
            type: "i64";
          },
          {
            name: "expiresAt";
            docs: ["Optional expiry (Unix timestamp). 0 = no expiry."];
            type: "i64";
          },
          {
            name: "bump";
            docs: ["PDA bump."];
            type: "u8";
          }
        ];
      };
    },
    {
      name: "org";
      docs: [
        "An organization is the top-level namespace for a set of roles.",
        'PDA seeds: ["org", authority]',
        "Authority is the pubkey that controls this org."
      ];
      type: {
        kind: "struct";
        fields: [
          {
            name: "authority";
            docs: [
              "The account that created this org and can manage its roles."
            ];
            type: "pubkey";
          },
          {
            name: "name";
            docs: [
              "A human-readable label for this org (off-chain convenience, opaque on-chain)."
            ];
            type: "string";
          },
          {
            name: "roleCount";
            docs: ["Number of roles created under this org."];
            type: "u64";
          },
          {
            name: "bump";
            docs: ["PDA bump."];
            type: "u8";
          }
        ];
      };
    },
    {
      name: "orgCreated";
      type: {
        kind: "struct";
        fields: [
          {
            name: "org";
            type: "pubkey";
          },
          {
            name: "authority";
            type: "pubkey";
          },
          {
            name: "name";
            type: "string";
          }
        ];
      };
    },
    {
      name: "role";
      docs: [
        "A named role under an org.",
        'PDA seeds: ["role", org_key, role_name_bytes (up to 32 bytes)]'
      ];
      type: {
        kind: "struct";
        fields: [
          {
            name: "org";
            docs: ["The org this role belongs to."];
            type: "pubkey";
          },
          {
            name: "name";
            docs: ["Name of the role (up to 32 bytes, UTF-8)."];
            type: "string";
          },
          {
            name: "admin";
            docs: [
              "The authority that can assign or revoke this role and toggle it.",
              "Set to the org authority at creation; the current program cannot rotate it."
            ];
            type: "pubkey";
          },
          {
            name: "memberCount";
            docs: ["Number of active members holding this role."];
            type: "u64";
          },
          {
            name: "enabled";
            docs: [
              "Whether this role is enabled. Disabled roles fail CPI checks."
            ];
            type: "bool";
          },
          {
            name: "bump";
            docs: ["PDA bump."];
            type: "u8";
          }
        ];
      };
    },
    {
      name: "roleAssigned";
      type: {
        kind: "struct";
        fields: [
          {
            name: "member";
            type: "pubkey";
          },
          {
            name: "role";
            type: "pubkey";
          },
          {
            name: "holder";
            type: "pubkey";
          },
          {
            name: "grantedAt";
            type: "i64";
          },
          {
            name: "expiresAt";
            type: "i64";
          }
        ];
      };
    },
    {
      name: "roleCreated";
      type: {
        kind: "struct";
        fields: [
          {
            name: "role";
            type: "pubkey";
          },
          {
            name: "org";
            type: "pubkey";
          },
          {
            name: "name";
            type: "string";
          },
          {
            name: "admin";
            type: "pubkey";
          }
        ];
      };
    },
    {
      name: "roleEnabledSet";
      type: {
        kind: "struct";
        fields: [
          {
            name: "role";
            type: "pubkey";
          },
          {
            name: "enabled";
            type: "bool";
          }
        ];
      };
    },
    {
      name: "roleRevoked";
      type: {
        kind: "struct";
        fields: [
          {
            name: "role";
            type: "pubkey";
          },
          {
            name: "holder";
            type: "pubkey";
          }
        ];
      };
    }
  ];
};
