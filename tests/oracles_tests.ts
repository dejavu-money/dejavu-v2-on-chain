import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DejavuV2 } from "../target/types/dejavu_v2";
import CreateToken from "./utils/create-token";
import CreateOrganization from "./utils/create-organization";
import { assert } from "chai";
import { BN } from "bn.js";
import * as Token from "@solana/spl-token";
import { Transaction, sendAndConfirmTransaction } from "@solana/web3.js";

enum GameResult {
  TeamAWin = 0,
  TeamBWin = 1,
  DRAW = 2
}

interface OracleBets {
  globalBetsIndex: number,
  gameResult: number,
}

describe("Oracles methods", () => {
  const provider = anchor.AnchorProvider.env();
  const payer = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(
      JSON.parse(
        require("fs").readFileSync(process.env.ANCHOR_WALLET, {
          encoding: "utf-8",
        })
      )
    )
  );
  anchor.setProvider(provider);

  const program = anchor.workspace.DejavuV2 as Program<DejavuV2>;

  // describe("#create-oracle", async () => {
  //   it("creates an oracle", async () => {
  //     const organizationId = new Date().getTime();
  //     const oracleId = new Date().getTime();
  //     const startAtUtcUnix = new Date().getTime();

  //     const token = await CreateToken({
  //       connection: provider.connection,
  //       token: {
  //         amount: 1,
  //       },
  //       accounts: {
  //         payer: provider.wallet.publicKey,
  //         payerSign: payer,
  //       },
  //     });

  //     const {
  //       accounts: { organization },
  //     } = await CreateOrganization(program, {
  //       connection: provider.connection,
  //       accounts: {
  //         payerSign: payer,
  //         user: provider.wallet.publicKey,
  //         mint: token.accounts.mint,
  //       },
  //       organization: {
  //         id: organizationId,
  //         fee: 10,
  //       },
  //     });

  //     const [oracle] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         Buffer.from("oracle"),
  //         organization.toBuffer(),
  //         Buffer.from(`${oracleId}`),
  //       ],
  //       program.programId
  //     );

  //     const [oracleVault] = await anchor.web3.PublicKey.findProgramAddress(
  //       [oracle.toBuffer(), Buffer.from("vault")],
  //       program.programId
  //     );

  //     const [oracleBets] = await anchor.web3.PublicKey.findProgramAddress(
  //       [oracle.toBuffer(), Buffer.from("bets")],
  //       program.programId
  //     );

  //     await program.methods
  //       .createOracle({
  //         id: new BN(oracleId),
  //         teamIdA: 4,
  //         teamIdB: 5,
  //         contextReference: 10,
  //         contextReferenceId: new BN(432),
  //         startAtUtcUnix: new BN(startAtUtcUnix),
  //       })
  //       .accounts({
  //         oracle: oracle,
  //         organization: organization,
  //         user: provider.wallet.publicKey,
  //         mint: token.accounts.mint,
  //         vault: oracleVault,
  //         oracleBets: oracleBets
  //       })
  //       .rpc();

  //     const oracleData = await program.account.oracle.fetch(oracle);

  //     assert.ok(oracleData.id.eq(new BN(oracleId)), "verify oracle id");

  //     assert.equal(oracleData.game.teamAId, 4, "verify team a id");

  //     assert.equal(oracleData.game.teamBId, 5, "verify team b id");

  //     assert.equal(
  //       oracleData.game.contextReference,
  //       10,
  //       "verify game context reference"
  //     );

  //     assert.ok(
  //       oracleData.game.contextReferenceId.eq(new BN(432)),
  //       "verify game context reference id"
  //     );

  //     assert.equal(oracleData.game.statusId, 0, "verify game status id");
  //   });
  // });

  describe("#join-oracle", async () => {
    it("joins ..", async () => {
      const organizationId = new Date().getTime();
      const oracleId = new Date().getTime();
      const startAtUtcUnix = new Date().getTime();

      const token = await CreateToken({
        connection: provider.connection,
        token: {
          amount: 1,
        },
        accounts: {
          payer: provider.wallet.publicKey,
          payerSign: payer,
        },
      });

      const {
        accounts: { organization },
      } = await CreateOrganization(program, {
        connection: provider.connection,
        accounts: {
          payerSign: payer,
          user: provider.wallet.publicKey,
          mint: token.accounts.mint,
        },
        organization: {
          id: organizationId,
          fee: 10,
        },
      });

      const [oracle] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("oracle"),
          organization.toBuffer(),
          Buffer.from(`${oracleId}`),
        ],
        program.programId
      );

      const [playerBet] = await anchor.web3.PublicKey.findProgramAddress(
        [
          oracle.toBuffer(),
          Buffer.from('player-0'),
        ],
        program.programId
      );

      const [oracleVault] = await anchor.web3.PublicKey.findProgramAddress(
        [oracle.toBuffer(), Buffer.from("vault")],
        program.programId
      );

      const [oracleBets] = await anchor.web3.PublicKey.findProgramAddress(
        [oracle.toBuffer(), Buffer.from("bets")],
        program.programId
      );

      await program.methods
        .createOracle({
          id: new BN(oracleId),
          initAmount: new BN(1),
          teamIdA: 4,
          teamIdB: 5,
          contextReference: 10,
          contextReferenceId: new BN(432),
          startAtUtcUnix: new BN(startAtUtcUnix),
        })
        .accounts({
          oracle: oracle,
          organization: organization,
          user: provider.wallet.publicKey,
          mint: token.accounts.mint,
          vault: oracleVault,
          oracleBets: oracleBets
        })
        .rpc();

        await program.methods.joinOracle({
          betIndex: 0,
          gameResult: 0,
        }).accounts(
          {
            oracle: oracle,
            oracleBets: oracleBets,
            organization: organization,
            payer: provider.wallet.publicKey,
            user: provider.wallet.publicKey,
            mint: token.accounts.mint,
            vault: oracleVault,
            playerBet: playerBet,
            userTokenAccount: token.accounts.payerMintAccount
          }
        ).rpc();

        const betsData = await program.account.bets.fetch(oracleBets);
        const playerBetData = await program.account.playerBet.fetch(playerBet);
        const bets = betsData.bets as OracleBets[];

        assert.equal(
          bets[0].globalBetsIndex,
          0,
          'verify bet index'
        );

        assert.equal(
          bets[0].gameResult,
          0,
          'verify bet game result'
        );

        assert.equal(
          playerBetData.index,
          0,
          'verify player bet index'
        );

        assert.ok(
          playerBetData.oracle.equals(oracle),
          'verify player bet oracle'
        );

        assert.ok(
          playerBetData.user.equals(provider.wallet.publicKey),
          'verify player bet created_by'
        );

    });
  });
});
