import { BN } from "@anchor-lang/core";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

import {
  HEDWIG_PROGRAM_ID,
  NO_EXPIRY,
  assertOrgName,
  assertRoleName,
  deriveMemberPda,
  deriveOrgPda,
  deriveRolePda,
  toExpiresAt,
} from "../src";

function expectExactError(
  action: () => unknown,
  errorType: typeof Error,
  message: string
): void {
  let thrown: unknown;

  try {
    action();
  } catch (error) {
    thrown = error;
  }

  expect(thrown).to.be.instanceOf(errorType);
  expect((thrown as Error).constructor).to.equal(errorType);
  expect((thrown as Error).message).to.equal(message);
}

describe("Hedwig PDA and argument helpers", () => {
  const authority = new PublicKey("11111111111111111111111111111111");
  const holder = new PublicKey("SysvarC1ock11111111111111111111111111111111");

  it("exports the fixed Hedwig program ID", () => {
    expect(HEDWIG_PROGRAM_ID.toBase58()).to.equal(
      "H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC"
    );
  });

  it("derives the known Org, Role, and Member PDA vectors", () => {
    const [org, orgBump] = deriveOrgPda(authority);
    const [role, roleBump] = deriveRolePda(org, "treasurer");
    const [member, memberBump] = deriveMemberPda(role, holder);

    expect(org.toBase58()).to.equal(
      "2gESMaStH2V2puYWFaxTEofiSvpokBQ5ppYusjtjkmEB"
    );
    expect(orgBump).to.equal(254);
    expect(role.toBase58()).to.equal(
      "XuAZop1C9KpkSpqi9ZxauBDuiSxv7cHjP8q96tjtqbJ"
    );
    expect(roleBump).to.equal(254);
    expect(member.toBase58()).to.equal(
      "5bLnSidSs3wxDPFtBGBRKTCMa5qkHwCfdzc3kxYYA7az"
    );
    expect(memberBump).to.equal(253);
  });

  it("accepts Org names from 1 through 64 UTF-8 bytes", () => {
    expect(assertOrgName("a")).to.equal("a");
    expect(assertOrgName("a".repeat(64))).to.equal("a".repeat(64));
    expect(assertOrgName("é".repeat(32))).to.equal("é".repeat(32));
  });

  it("rejects empty and over-limit Org names by UTF-8 byte length", () => {
    const message = "Org name must be 1 to 64 UTF-8 bytes";
    expectExactError(() => assertOrgName(""), RangeError, message);
    expectExactError(() => assertOrgName("a".repeat(65)), RangeError, message);
    expectExactError(() => assertOrgName("é".repeat(33)), RangeError, message);
  });

  it("rejects malformed UTF-16 Org names", () => {
    expectExactError(
      () => assertOrgName("\ud800"),
      TypeError,
      "Org name must be valid UTF-8"
    );
  });

  it("accepts Role names from 1 through 32 UTF-8 bytes", () => {
    expect(assertRoleName("a")).to.equal("a");
    expect(assertRoleName("a".repeat(32))).to.equal("a".repeat(32));
    expect(assertRoleName("é".repeat(16))).to.equal("é".repeat(16));
  });

  it("rejects empty and over-limit Role names by UTF-8 byte length", () => {
    const message = "Role name must be 1 to 32 UTF-8 bytes";
    expectExactError(() => assertRoleName(""), RangeError, message);
    expectExactError(() => assertRoleName("a".repeat(33)), RangeError, message);
    expectExactError(() => assertRoleName("é".repeat(17)), RangeError, message);
  });

  it("rejects malformed UTF-16 Role names", () => {
    expectExactError(
      () => assertRoleName("\udfff"),
      TypeError,
      "Role name must be valid UTF-8"
    );
  });

  it("maps absent and null expiry to the no-expiry sentinel", () => {
    expect(NO_EXPIRY.toString()).to.equal("0");
    expect(toExpiresAt().toString()).to.equal("0");
    expect(toExpiresAt(null).toString()).to.equal("0");
  });

  it("maps Date, integer seconds, and BN expiry values", () => {
    expect(
      toExpiresAt(new Date("2030-01-01T00:00:00.000Z")).toString()
    ).to.equal("1893456000");
    expect(toExpiresAt(1_893_456_000).toString()).to.equal("1893456000");
    expect(toExpiresAt(new BN("1893456000")).toString()).to.equal("1893456000");
  });

  it("accepts the maximum signed i64 expiry", () => {
    expect(toExpiresAt(new BN("9223372036854775807")).toString()).to.equal(
      "9223372036854775807"
    );
  });

  it("rejects an expiry above the maximum signed i64", () => {
    expectExactError(
      () => toExpiresAt(new BN("9223372036854775808")),
      RangeError,
      "Expiry must fit a non-negative signed i64"
    );
  });

  it("rejects negative, fractional, and unsafe number expiry values", () => {
    const message =
      "Expiry must be a non-negative safe integer number of Unix seconds";
    expectExactError(() => toExpiresAt(-1), RangeError, message);
    expectExactError(() => toExpiresAt(1.5), RangeError, message);
    expectExactError(
      () => toExpiresAt(Number.MAX_SAFE_INTEGER + 1),
      RangeError,
      message
    );
  });
});
