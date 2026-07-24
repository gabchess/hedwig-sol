import { BN } from "@anchor-lang/core";
import { PublicKey } from "@solana/web3.js";

export const HEDWIG_PROGRAM_ID = new PublicKey(
  "H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC"
);

export const NO_EXPIRY = new BN(0);

export type ExpiresAtInput = BN | Date | number | null | undefined;

const I64_MAX = new BN("9223372036854775807");

function assertName(name: string, label: string, maxBytes: number): string {
  const encoded = Buffer.from(name, "utf8");
  if (encoded.toString("utf8") !== name) {
    throw new TypeError(`${label} name must be valid UTF-8`);
  }
  if (encoded.length < 1 || encoded.length > maxBytes) {
    throw new RangeError(`${label} name must be 1 to ${maxBytes} UTF-8 bytes`);
  }
  return name;
}

export function assertOrgName(name: string): string {
  return assertName(name, "Org", 64);
}

export function assertRoleName(name: string): string {
  return assertName(name, "Role", 32);
}

export function toExpiresAt(value?: ExpiresAtInput): BN {
  if (value == null) {
    return new BN(0);
  }

  if (BN.isBN(value)) {
    if (value.isNeg() || value.gt(I64_MAX)) {
      throw new RangeError("Expiry must fit a non-negative signed i64");
    }
    return value.clone();
  }

  const seconds =
    value instanceof Date ? Math.floor(value.getTime() / 1_000) : value;
  if (!Number.isSafeInteger(seconds) || seconds < 0) {
    throw new RangeError(
      "Expiry must be a non-negative safe integer number of Unix seconds"
    );
  }
  return new BN(seconds);
}

export function deriveOrgPda(authority: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("org"), authority.toBuffer()],
    HEDWIG_PROGRAM_ID
  );
}

export function deriveRolePda(
  org: PublicKey,
  name: string
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("role"), org.toBuffer(), Buffer.from(assertRoleName(name))],
    HEDWIG_PROGRAM_ID
  );
}

export function deriveMemberPda(
  role: PublicKey,
  holder: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("member"), role.toBuffer(), holder.toBuffer()],
    HEDWIG_PROGRAM_ID
  );
}
