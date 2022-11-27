import { PublicKey } from "@solana/web3.js";
import * as Token from "@solana/spl-token";

import type { Connection, Signer } from "@solana/web3.js";

interface Input {
  connection: Connection;

  token: {
    amount: number;
    decimals?: number;
  };

  accounts: {
    payerSign: Signer;
    payer: PublicKey;
  };
}

interface Output {
  accounts: {
    mint: PublicKey;
    payerMintAccount: PublicKey;
  };
}

export default async (input: Input): Promise<Output> => {
  const mint = await Token.createMint(
    input.connection,
    input.accounts.payerSign,
    input.accounts.payer,
    null,
    input.token.decimals || 0
  );

  const payerMintAccount = await Token.createAccount(
    input.connection,
    input.accounts.payerSign,
    mint,
    input.accounts.payer
  );

  await Token.mintTo(
    input.connection,
    input.accounts.payerSign,
    mint,
    payerMintAccount,
    input.accounts.payerSign,
    input.token.amount
  );

  return {
    accounts: {
      mint,
      payerMintAccount,
    },
  };
};
