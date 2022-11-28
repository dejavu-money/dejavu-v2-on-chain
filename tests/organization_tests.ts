// import * as anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { DejavuV2 } from "../target/types/dejavu_v2";
// import CreateToken from "./utils/create-token";
// import CreateOrganization from "./utils/create-organization";
// import { assert } from "chai";
// import { BN } from "bn.js";
// import * as Token from "@solana/spl-token";
// import { Transaction, sendAndConfirmTransaction } from "@solana/web3.js";

// describe("Organizations methods", () => {
//   const provider = anchor.AnchorProvider.env();
//   const payer = anchor.web3.Keypair.fromSecretKey(
//     Buffer.from(
//       JSON.parse(
//         require("fs").readFileSync(process.env.ANCHOR_WALLET, {
//           encoding: "utf-8",
//         })
//       )
//     )
//   );
//   anchor.setProvider(provider);

//   const program = anchor.workspace.DejavuV2 as Program<DejavuV2>;

//   describe("#create-organization", async () => {
//     it("creates an organization", async () => {
//       const organizationId = new Date().getTime();
//       const token = await CreateToken({
//         connection: provider.connection,
//         token: {
//           amount: 1,
//         },
//         accounts: {
//           payer: provider.wallet.publicKey,
//           payerSign: payer,
//         },
//       });

//       const {
//         accounts: { organization },
//       } = await CreateOrganization(program, {
//         connection: provider.connection,
//         accounts: {
//           payerSign: payer,
//           user: provider.wallet.publicKey,
//           mint: token.accounts.mint,
//         },
//         organization: {
//           id: organizationId,
//           fee: 10,
//         },
//       });

//       const organizationData = await program.account.organization.fetch(
//         organization
//       );

//       assert.ok(
//         organizationData.authority.equals(provider.wallet.publicKey),
//         "verify if the authority was assigned"
//       );

//       assert.ok(
//         organizationData.fee.toNumber() === 10,
//         "verify if the fee was assigned"
//       );

//       assert.ok(
//         organizationData.mint.equals(token.accounts.mint),
//         "verify if the mint was assigned"
//       );
//     });
//   });

//   describe("#update-organization", async () => {
//     it("updates an organization", async () => {
//       const organizationId = new Date().getTime();
//       const token = await CreateToken({
//         connection: provider.connection,
//         token: {
//           amount: 1,
//         },
//         accounts: {
//           payer: provider.wallet.publicKey,
//           payerSign: payer,
//         },
//       });

//       const {
//         accounts: { organization },
//       } = await CreateOrganization(program, {
//         connection: provider.connection,
//         accounts: {
//           payerSign: payer,
//           user: provider.wallet.publicKey,
//           mint: token.accounts.mint,
//         },
//         organization: {
//           id: organizationId,
//           fee: 10,
//         },
//       });

//       let organizationData = await program.account.organization.fetch(
//         organization
//       );

//       assert.ok(
//         organizationData.fee.toNumber() === 10,
//         "verify if the fee was assigned"
//       );

//       await program.methods
//         .updateOrganization({ fee: new BN(20) })
//         .accounts({
//           organization: organization,
//           user: provider.wallet.publicKey,
//         })
//         .rpc();

//       organizationData = await program.account.organization.fetch(organization);

//       assert.ok(
//         organizationData.fee.toNumber() === 20,
//         "verify if the fee was updated"
//       );
//     });
//   });

//   describe("#withdraw-from-organization", async () => {
//     it("withdraw from organization", async () => {
//       const organizationId = new Date().getTime();
//       const token = await CreateToken({
//         connection: provider.connection,
//         token: {
//           amount: 1000,
//         },
//         accounts: {
//           payer: provider.wallet.publicKey,
//           payerSign: payer,
//         },
//       });

//       const {
//         accounts: { organization, vaultAccount },
//       } = await CreateOrganization(program, {
//         connection: provider.connection,
//         accounts: {
//           payerSign: payer,
//           user: provider.wallet.publicKey,
//           mint: token.accounts.mint,
//         },
//         organization: {
//           id: organizationId,
//           fee: 10,
//         },
//       });

//       // Add token transfer instructions to transaction
//       const transaction = new Transaction().add(
//         Token.createTransferInstruction(
//           token.accounts.payerMintAccount,
//           vaultAccount,
//           provider.wallet.publicKey,
//           100,
//           []
//         )
//       );

//       await sendAndConfirmTransaction(provider.connection, transaction, [
//         payer,
//       ]);

//       let vaulBalance = await provider.connection.getTokenAccountBalance(
//         vaultAccount
//       );

//       let userBalance = await provider.connection.getTokenAccountBalance(
//         token.accounts.payerMintAccount
//       );

//       assert.equal(
//         Number(vaulBalance.value.amount),
//         100,
//         "verify if the funds were transfer to the wallet"
//       );

//       assert.equal(
//         Number(userBalance.value.amount),
//         900,
//         "verify if the funds were transfer to the vault"
//       );

//       await program.methods
//         .withdrawFromOrganization({
//           amount: new BN(100),
//         })
//         .accounts({
//           organization: organization,
//           user: provider.wallet.publicKey,
//           organizationTokenAccount: vaultAccount,
//           userTokenAccount: token.accounts.payerMintAccount,
//         })
//         .rpc();

//       vaulBalance = await provider.connection.getTokenAccountBalance(
//         vaultAccount
//       );

//       userBalance = await provider.connection.getTokenAccountBalance(
//         token.accounts.payerMintAccount
//       );

//       assert.equal(
//         Number(vaulBalance.value.amount),
//         0,
//         "verify if the funds were transfer to the user wallet"
//       );

//       assert.equal(
//         Number(userBalance.value.amount),
//         1000,
//         "verify if the funds were transfer"
//       );
//     });
//   });
// });
