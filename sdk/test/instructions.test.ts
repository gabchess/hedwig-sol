import {
  BN,
  BorshInstructionCoder,
  type Program,
  type Provider,
} from "@anchor-lang/core";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  type AccountMeta,
  type ConfirmOptions,
  type Signer,
  type TransactionInstruction,
} from "@solana/web3.js";
import { expect } from "chai";

import {
  HEDWIG_PROGRAM_ID,
  buildAssignRoleInstruction,
  buildCheckRoleInstruction,
  buildCreateOrgInstruction,
  buildCreateRoleInstruction,
  buildRevokeRoleInstruction,
  buildSetRoleEnabledInstruction,
  createHedwigProgram,
  deriveMemberPda,
  deriveOrgPda,
  deriveRolePda,
  sendAssignRole,
  sendCheckRole,
  sendCreateOrg,
  sendCreateRole,
  sendRevokeRole,
  sendSetRoleEnabled,
  type AssignRoleInput,
  type CheckRoleInput,
  type CreateOrgInput,
  type CreateRoleInput,
  type HedwigSol,
  type RevokeRoleInput,
  type SendInstructionOptions,
  type SetRoleEnabledInput,
} from "../src";

type DecodedData = Record<string, unknown>;

const coder = createHedwigProgram({} as Provider).coder
  .instruction as BorshInstructionCoder;
const authority = new PublicKey("11111111111111111111111111111111");
const holder = new PublicKey("SysvarC1ock11111111111111111111111111111111");
const [org] = deriveOrgPda(authority);
const [role] = deriveRolePda(org, "treasurer");
const [member] = deriveMemberPda(role, holder);

function expectedMeta(
  pubkey: PublicKey,
  isWritable: boolean,
  isSigner: boolean
): AccountMeta {
  return { pubkey, isWritable, isSigner };
}

function expectKeys(
  instruction: TransactionInstruction,
  expected: AccountMeta[]
): void {
  expect(
    instruction.keys.map(({ pubkey, isWritable, isSigner }) => ({
      pubkey: pubkey.toBase58(),
      isWritable,
      isSigner,
    }))
  ).to.deep.equal(
    expected.map(({ pubkey, isWritable, isSigner }) => ({
      pubkey: pubkey.toBase58(),
      isWritable,
      isSigner,
    }))
  );
}

function expectDecoded(
  instruction: TransactionInstruction,
  name: string,
  data: DecodedData
): void {
  expect(instruction.programId.equals(HEDWIG_PROGRAM_ID)).to.equal(true);
  const decoded = coder.decode(instruction.data);
  expect(decoded).not.to.equal(null);
  expect(decoded?.name).to.equal(name);

  const actual = decoded?.data as DecodedData;
  for (const [key, value] of Object.entries(data)) {
    const actualValue = actual[key];
    if (BN.isBN(value)) {
      expect((actualValue as BN).toString()).to.equal(value.toString());
    } else {
      expect(actualValue).to.deep.equal(value);
    }
  }
  expect(Object.keys(actual)).to.have.members(Object.keys(data));
}

function expectRawData(
  instruction: TransactionInstruction,
  expectedHex: string
): void {
  expect(instruction.data.toString("hex")).to.equal(expectedHex);
}

function instructionProvider(): Provider {
  return {} as Provider;
}

describe("Hedwig instruction builders", () => {
  it("exports the generated HedwigSol program type", () => {
    const program: Program<HedwigSol> = createHedwigProgram(
      instructionProvider()
    );
    expect(program.programId.equals(HEDWIG_PROGRAM_ID)).to.equal(true);
  });

  it("builds createOrg with exact data, PDA, and ordered account metas", async () => {
    const instruction = await buildCreateOrgInstruction(instructionProvider(), {
      authority,
      name: "treasury",
    });

    expectDecoded(instruction, "createOrg", { name: "treasury" });
    expectRawData(instruction, "3073bbf92403baaf080000007472656173757279");
    expectKeys(instruction, [
      expectedMeta(org, true, false),
      expectedMeta(authority, true, true),
      expectedMeta(SystemProgram.programId, false, false),
    ]);
  });

  it("builds createRole with exact data, PDA, and ordered account metas", async () => {
    const instruction = await buildCreateRoleInstruction(
      instructionProvider(),
      {
        org,
        authority,
        name: "treasurer",
      }
    );

    expectDecoded(instruction, "createRole", { name: "treasurer" });
    expectRawData(instruction, "aa937fdfde70cda309000000747265617375726572");
    expectKeys(instruction, [
      expectedMeta(role, true, false),
      expectedMeta(org, true, false),
      expectedMeta(authority, true, true),
      expectedMeta(SystemProgram.programId, false, false),
    ]);
  });

  it("builds assignRole with exact data, PDA, and ordered account metas", async () => {
    const instruction = await buildAssignRoleInstruction(
      instructionProvider(),
      {
        role,
        holder,
        admin: authority,
        expiresAt: 1_893_456_000,
      }
    );

    expectDecoded(instruction, "assignRole", {
      expiresAt: new BN("1893456000"),
    });
    expectRawData(instruction, "ffae7db4cb9bca8380d8db7000000000");
    expectKeys(instruction, [
      expectedMeta(member, true, false),
      expectedMeta(role, true, false),
      expectedMeta(holder, false, false),
      expectedMeta(authority, true, true),
      expectedMeta(SystemProgram.programId, false, false),
    ]);
  });

  it("builds checkRole without marking the holder as a signer", async () => {
    const instruction = await buildCheckRoleInstruction(instructionProvider(), {
      role,
      holder,
    });

    expectDecoded(instruction, "checkRole", {});
    expectRawData(instruction, "8edd614f22465fcb");
    expectKeys(instruction, [
      expectedMeta(member, false, false),
      expectedMeta(role, false, false),
      expectedMeta(holder, false, false),
    ]);
    expect(instruction.keys[2].isSigner).to.equal(false);
  });

  it("builds revokeRole with exact data, PDA, and ordered account metas", async () => {
    const instruction = await buildRevokeRoleInstruction(
      instructionProvider(),
      {
        role,
        holder,
        admin: authority,
      }
    );

    expectDecoded(instruction, "revokeRole", {});
    expectRawData(instruction, "b3e802b430e35207");
    expectKeys(instruction, [
      expectedMeta(member, true, false),
      expectedMeta(role, true, false),
      expectedMeta(authority, true, true),
      expectedMeta(SystemProgram.programId, false, false),
    ]);
  });

  it("builds setRoleEnabled with exact data and ordered account metas", async () => {
    const instruction = await buildSetRoleEnabledInstruction(
      instructionProvider(),
      {
        role,
        admin: authority,
        enabled: false,
      }
    );

    expectDecoded(instruction, "setRoleEnabled", { enabled: false });
    expectRawData(instruction, "33021cddf8dc491400");
    expectKeys(instruction, [
      expectedMeta(role, true, false),
      expectedMeta(authority, false, true),
    ]);
  });
});

type SenderCase =
  | {
      name: "createOrg";
      input: CreateOrgInput;
      send: typeof sendCreateOrg;
    }
  | {
      name: "createRole";
      input: CreateRoleInput;
      send: typeof sendCreateRole;
    }
  | {
      name: "assignRole";
      input: AssignRoleInput;
      send: typeof sendAssignRole;
    }
  | {
      name: "checkRole";
      input: CheckRoleInput;
      send: typeof sendCheckRole;
    }
  | {
      name: "revokeRole";
      input: RevokeRoleInput;
      send: typeof sendRevokeRole;
    }
  | {
      name: "setRoleEnabled";
      input: SetRoleEnabledInput;
      send: typeof sendSetRoleEnabled;
    };

const senderCases: SenderCase[] = [
  {
    name: "createOrg",
    input: { authority, name: "treasury" },
    send: sendCreateOrg,
  },
  {
    name: "createRole",
    input: { org, authority, name: "treasurer" },
    send: sendCreateRole,
  },
  {
    name: "assignRole",
    input: { role, holder, admin: authority },
    send: sendAssignRole,
  },
  {
    name: "checkRole",
    input: { role, holder },
    send: sendCheckRole,
  },
  {
    name: "revokeRole",
    input: { role, holder, admin: authority },
    send: sendRevokeRole,
  },
  {
    name: "setRoleEnabled",
    input: { role, admin: authority, enabled: false },
    send: sendSetRoleEnabled,
  },
];

describe("Hedwig instruction senders", () => {
  it("rejects with the documented error when the provider cannot send", async () => {
    let rejection: unknown;

    try {
      await sendCheckRole(
        instructionProvider(),
        { role, holder },
        {
          signers: [],
        }
      );
    } catch (error) {
      rejection = error;
    }

    expect(rejection).to.be.instanceOf(Error);
    expect((rejection as Error).constructor).to.equal(Error);
    expect((rejection as Error).message).to.equal(
      "Provider does not support sendAndConfirm"
    );
  });

  for (const testCase of senderCases) {
    it(`forwards the exact signer array and confirmation options for ${testCase.name}`, async () => {
      const signers: Signer[] = [Keypair.generate()];
      const confirmOptions: ConfirmOptions = {
        commitment: "processed",
        skipPreflight: true,
      };
      const options: SendInstructionOptions = {
        signers,
        confirmOptions,
      };
      let observedTransaction: Transaction | undefined;
      let observedSigners: Signer[] | undefined;
      let observedOptions: ConfirmOptions | undefined;

      const provider = {
        sendAndConfirm: async (
          transaction: Transaction,
          forwardedSigners?: Signer[],
          forwardedOptions?: ConfirmOptions
        ) => {
          observedTransaction = transaction;
          observedSigners = forwardedSigners;
          observedOptions = forwardedOptions;
          return "fake-signature";
        },
      } as unknown as Provider;

      const signature = await testCase.send(
        provider,
        testCase.input as never,
        options
      );

      expect(signature).to.equal("fake-signature");
      expect(observedTransaction).to.be.instanceOf(Transaction);
      expect(observedTransaction?.instructions).to.have.length(1);
      expect(observedSigners).to.equal(signers);
      expect(observedOptions).to.equal(confirmOptions);
    });
  }
});
