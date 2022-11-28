import { DejavuV2 } from "../../target/types/dejavu_v2";
import { PublicKey } from "@solana/web3.js";
import { Program } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import { BN } from "bn.js";
import createToken from "./create-token";
import type { Connection, Signer } from "@solana/web3.js";

interface Input {
  connection: Connection;
  organization: {
    id: number;
    fee?: number;
  };
  accounts: {
    payerSign: Signer;
    user: PublicKey;
    mint?: PublicKey;
  };
}

interface Output {
  accounts: {
    organization: PublicKey;
    vaultAccount: PublicKey;
  };
}

export default async (
  program: Program<DejavuV2>,
  input: Input
): Promise<Output> => {
  const [organization] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("organization"), Buffer.from(`${input.organization.id}`)],
    program.programId
  );

  const [vaultAccount] = await anchor.web3.PublicKey.findProgramAddress(
    [organization.toBuffer(), Buffer.from("vault")],
    program.programId
  );

  const [oracleBets] = await anchor.web3.PublicKey.findProgramAddress(
    [organization.toBuffer(), Buffer.from("vault")],
    program.programId
  );

  const mint =
    input.accounts.mint ||
    (
      await createToken({
        connection: input.connection,
        token: {
          amount: 1000,
        },

        accounts: {
          payer: input.accounts.user,
          payerSign: input.accounts.payerSign,
        },
      })
    ).accounts.mint;

  await program.methods
    .createOrganization({
      id: new BN(input.organization.id),
      fee: new BN(input.organization.fee || 0),
    })
    .accounts({
      organization: organization,
      user: input.accounts.user,
      mint: mint,
      vaultAccount: vaultAccount,
    })
    .rpc();

  return {
    accounts: {
      organization,
      vaultAccount,
    },
  };
};
