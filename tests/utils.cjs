const { expectTX } = require("@saberhq/chai-solana");
const { Keypair, SystemProgram, PublicKey, Signer } = require("@solana/web3.js");
const {  TOKEN_PROGRAM_ID } = require("@solana/spl-token");
const { Provider } = require("@saberhq/solana-contrib");
const { TransactionEnvelope } = require("@saberhq/solana-contrib");
const { TokenAccountLayout, createTokenAccount } = require("@saberhq/token-utils");


async function createBalanceSandbox(provider, poolMint, nftMint,  owner, payer) {

  let spt = await createTokenAccount(
   {
     mint: poolMint,
     owner: owner,
     provider: provider
   }
    );
  if (spt.tx) {
    await expectTX(spt.tx).to.be.fulfilled;
  }


  const vault = await createTokenAccount(
    {
      mint: nftMint,
      owner: owner,
      provider: provider
    }
     );
   if (vault.tx) {
     await expectTX(vault.tx).to.be.fulfilled;
   }

   const vaultStake = await createTokenAccount(
    {
      mint: nftMint,
      owner: owner,
      provider: provider
    }
     );
   if (vaultStake.tx) {
     await expectTX(vaultStake.tx).to.be.fulfilled;
   }

  const vaultPw = await createTokenAccount(
    {
      mint: nftMint,
      owner: owner,
      provider: provider
    }
     );
   if (vaultPw.tx) {
     await expectTX(vaultPw.tx).to.be.fulfilled;
   }


  return {
      spt: spt.key,
      vault: vault.key,
      vaultStake: vaultStake.key,
      vaultPw: vaultPw.key,
    }

}

module.exports = {
  createBalanceSandbox,
};





async function createTokenAccountIx({provider, mint, owner,  payer}) {

    const tokenAccount = new Keypair().publicKey;

    console.log("payer", payer);

      // Allocate memory for the account
  const balanceNeeded = await Token.getMinBalanceRentForExemptAccount(
    provider.connection
  );


    return {
        key: tokenAccount,
        tx: new TransactionEnvelope(
          provider,
          [
            SystemProgram.createAccount({
              fromPubkey: payer.publicKey,
              newAccountPubkey: tokenAccount,
              lamports: balanceNeeded,
              space: TokenAccountLayout.span,
              programId: TOKEN_PROGRAM_ID,
            }),
            Token.createInitAccountInstruction(
              TOKEN_PROGRAM_ID,
              mint,
              tokenAccount,
              owner
            ),
          ],
          [payer]
        ),
      };
}