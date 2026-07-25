import { Program, type Provider } from "@anchor-lang/core";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  type ConfirmOptions,
  type Signer,
  type TransactionInstruction,
  type TransactionSignature,
} from "@solana/web3.js";

import idl from "./idl/hedwig_sol.json";
import type { HedwigSol } from "./idl/hedwig_sol";
import {
  assertOrgName,
  assertRoleName,
  deriveMemberPda,
  deriveOrgPda,
  deriveRolePda,
  toExpiresAt,
  type ExpiresAtInput,
} from "./pdas";

export interface CreateOrgInput {
  authority: PublicKey;
  name: string;
}

export interface CreateRoleInput {
  org: PublicKey;
  authority: PublicKey;
  name: string;
}

export interface AssignRoleInput {
  role: PublicKey;
  holder: PublicKey;
  admin: PublicKey;
  expiresAt?: ExpiresAtInput;
}

export interface CheckRoleInput {
  role: PublicKey;
  holder: PublicKey;
}

export interface RevokeRoleInput {
  role: PublicKey;
  holder: PublicKey;
  admin: PublicKey;
}

export interface SetRoleEnabledInput {
  role: PublicKey;
  admin: PublicKey;
  enabled: boolean;
}

export interface SendInstructionOptions {
  signers: Signer[];
  confirmOptions?: ConfirmOptions;
}

export function createHedwigProgram(provider: Provider): Program<HedwigSol> {
  return new Program(idl as unknown as HedwigSol, provider);
}

export async function buildCreateOrgInstruction(
  provider: Provider,
  input: CreateOrgInput
): Promise<TransactionInstruction> {
  const name = assertOrgName(input.name);
  const [org] = deriveOrgPda(input.authority);

  return createHedwigProgram(provider)
    .methods.createOrg(name)
    .accountsStrict({
      org,
      authority: input.authority,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export async function buildCreateRoleInstruction(
  provider: Provider,
  input: CreateRoleInput
): Promise<TransactionInstruction> {
  const name = assertRoleName(input.name);
  const [role] = deriveRolePda(input.org, name);

  return createHedwigProgram(provider)
    .methods.createRole(name)
    .accountsStrict({
      role,
      org: input.org,
      authority: input.authority,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export async function buildAssignRoleInstruction(
  provider: Provider,
  input: AssignRoleInput
): Promise<TransactionInstruction> {
  const [member] = deriveMemberPda(input.role, input.holder);

  return createHedwigProgram(provider)
    .methods.assignRole(toExpiresAt(input.expiresAt))
    .accountsStrict({
      member,
      role: input.role,
      holder: input.holder,
      admin: input.admin,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export async function buildCheckRoleInstruction(
  provider: Provider,
  input: CheckRoleInput
): Promise<TransactionInstruction> {
  const [member] = deriveMemberPda(input.role, input.holder);

  return createHedwigProgram(provider)
    .methods.checkRole()
    .accountsStrict({
      member,
      role: input.role,
      holder: input.holder,
    })
    .instruction();
}

export async function buildRevokeRoleInstruction(
  provider: Provider,
  input: RevokeRoleInput
): Promise<TransactionInstruction> {
  const [member] = deriveMemberPda(input.role, input.holder);

  return createHedwigProgram(provider)
    .methods.revokeRole()
    .accountsStrict({
      member,
      role: input.role,
      admin: input.admin,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export async function buildSetRoleEnabledInstruction(
  provider: Provider,
  input: SetRoleEnabledInput
): Promise<TransactionInstruction> {
  return createHedwigProgram(provider)
    .methods.setRoleEnabled(input.enabled)
    .accountsStrict({
      role: input.role,
      admin: input.admin,
    })
    .instruction();
}

async function sendInstruction(
  provider: Provider,
  instruction: Promise<TransactionInstruction>,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  if (!provider.sendAndConfirm) {
    throw new Error("Provider does not support sendAndConfirm");
  }

  const transaction = new Transaction().add(await instruction);
  return provider.sendAndConfirm(
    transaction,
    options.signers,
    options.confirmOptions
  );
}

export function sendCreateOrg(
  provider: Provider,
  input: CreateOrgInput,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  return sendInstruction(
    provider,
    buildCreateOrgInstruction(provider, input),
    options
  );
}

export function sendCreateRole(
  provider: Provider,
  input: CreateRoleInput,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  return sendInstruction(
    provider,
    buildCreateRoleInstruction(provider, input),
    options
  );
}

export function sendAssignRole(
  provider: Provider,
  input: AssignRoleInput,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  return sendInstruction(
    provider,
    buildAssignRoleInstruction(provider, input),
    options
  );
}

export function sendCheckRole(
  provider: Provider,
  input: CheckRoleInput,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  return sendInstruction(
    provider,
    buildCheckRoleInstruction(provider, input),
    options
  );
}

export function sendRevokeRole(
  provider: Provider,
  input: RevokeRoleInput,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  return sendInstruction(
    provider,
    buildRevokeRoleInstruction(provider, input),
    options
  );
}

export function sendSetRoleEnabled(
  provider: Provider,
  input: SetRoleEnabledInput,
  options: SendInstructionOptions
): Promise<TransactionSignature> {
  return sendInstruction(
    provider,
    buildSetRoleEnabledInstruction(provider, input),
    options
  );
}
