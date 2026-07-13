/**
 * Devnet end-to-end lifecycle demo for the Hedwig roles primitive.
 *
 * Exercises the five instructions in the current live devnet deployment:
 *   create_org -> create_role -> assign_role -> check_role -> revoke_role
 *
 * Usage: see app/README.md
 */
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import {
  AnchorProvider,
  BN,
  Program,
  setProvider,
  Wallet,
  type Idl,
} from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  clusterApiUrl,
} from "@solana/web3.js";

const PROGRAM_ID = new PublicKey(
  "H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC"
);

function loadKeypair(): Keypair {
  const walletPath =
    process.env.ANCHOR_WALLET ||
    path.join(os.homedir(), ".config", "solana", "id.json");
  const raw = fs.readFileSync(walletPath, "utf-8");
  const secret = Uint8Array.from(JSON.parse(raw));
  return Keypair.fromSecretKey(secret);
}

async function loadIdl(
  program: PublicKey,
  provider: AnchorProvider
): Promise<Idl> {
  const localIdlPath = path.join(
    __dirname,
    "..",
    "target",
    "idl",
    "hedwig_sol.json"
  );
  if (fs.existsSync(localIdlPath)) {
    console.log(`[setup] loading IDL from ${localIdlPath}`);
    return JSON.parse(fs.readFileSync(localIdlPath, "utf-8")) as Idl;
  }
  console.log(
    "[setup] no local IDL found, fetching from chain via Program.fetchIdl"
  );
  const idl = await Program.fetchIdl(program, provider);
  if (!idl) {
    throw new Error(
      "Could not resolve IDL: no local target/idl/hedwig_sol.json and no on-chain IDL account found."
    );
  }
  return idl;
}

function randomOrgName(): string {
  const suffix = Math.random().toString(36).slice(2, 10);
  return `demo-org-${suffix}`;
}

async function main() {
  const rpcUrl = process.env.HELIUS_RPC_URL || clusterApiUrl("devnet");
  const connection = new Connection(rpcUrl, "confirmed");

  const payer = loadKeypair();
  const wallet = new Wallet(payer);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  setProvider(provider);

  console.log(`[setup] wallet: ${payer.publicKey.toBase58()}`);
  const balanceLamports = await connection.getBalance(payer.publicKey);
  console.log(`[setup] balance: ${balanceLamports / 1e9} SOL`);
  if (balanceLamports === 0) {
    throw new Error(
      `Wallet ${payer.publicKey.toBase58()} has 0 SOL on devnet. Fund it with: solana airdrop 1 ${payer.publicKey.toBase58()} --url devnet`
    );
  }

  const idl = await loadIdl(PROGRAM_ID, provider);
  const program = new Program(idl, provider);

  const orgName = randomOrgName();
  const roleName = "treasurer";
  const holder = Keypair.generate();

  console.log(`[setup] org name: ${orgName}`);
  console.log(`[setup] role name: ${roleName}`);
  console.log(`[setup] holder: ${holder.publicKey.toBase58()}`);

  const [orgPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("org"), payer.publicKey.toBuffer()],
    PROGRAM_ID
  );
  const [rolePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("role"), orgPda.toBuffer(), Buffer.from(roleName)],
    PROGRAM_ID
  );
  const [memberPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("member"), rolePda.toBuffer(), holder.publicKey.toBuffer()],
    PROGRAM_ID
  );

  // 1. create_org
  const createOrgSig = await program.methods
    .createOrg(orgName)
    .accounts({
      org: orgPda,
      authority: payer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log(`[create_org] org=${orgPda.toBase58()} tx=${createOrgSig}`);

  // 2. create_role
  const createRoleSig = await program.methods
    .createRole(roleName)
    .accounts({
      role: rolePda,
      org: orgPda,
      authority: payer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log(`[create_role] role=${rolePda.toBase58()} tx=${createRoleSig}`);

  // 3. assign_role (no expiry: pass 0)
  const assignRoleSig = await program.methods
    .assignRole(new BN(0))
    .accounts({
      member: memberPda,
      role: rolePda,
      holder: holder.publicKey,
      admin: payer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log(
    `[assign_role] member=${memberPda.toBase58()} tx=${assignRoleSig}`
  );

  // 4. check_role -- read back the resulting PDAs to prove state.
  const memberAccount: any = await (program.account as any).member.fetch(
    memberPda
  );
  const roleAccount: any = await (program.account as any).role.fetch(rolePda);
  console.log(
    `[check_role:state] role.enabled=${
      roleAccount.enabled
    } role.memberCount=${roleAccount.memberCount.toString()} member.holder=${memberAccount.holder.toBase58()} member.expiresAt=${memberAccount.expiresAt.toString()}`
  );

  const checkRoleSig = await program.methods
    .checkRole()
    .accounts({
      member: memberPda,
      role: rolePda,
      holder: holder.publicKey,
    })
    .rpc();
  console.log(
    `[check_role] verified holder=${holder.publicKey.toBase58()} tx=${checkRoleSig}`
  );

  // 5. revoke_role
  const revokeRoleSig = await program.methods
    .revokeRole()
    .accounts({
      member: memberPda,
      role: rolePda,
      admin: payer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log(
    `[revoke_role] member=${memberPda.toBase58()} tx=${revokeRoleSig}`
  );

  const memberClosed = await connection.getAccountInfo(memberPda);
  if (memberClosed !== null) {
    throw new Error(
      `Expected member PDA ${memberPda.toBase58()} to be closed after revoke_role, but it still exists.`
    );
  }
  console.log(`[revoke_role:state] member PDA closed, rent returned to admin`);

  console.log("full lifecycle OK on devnet");
}

main().catch((err) => {
  console.error("[error] demo failed:");
  console.error(err);
  process.exit(1);
});
