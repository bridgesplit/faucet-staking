
import { BN, Program } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import { Lockup } from "../target/types/lockup";
import { assert, expect, use } from "chai";
import * as chai from "chai-as-promised";
import * as utils from "./utils.cjs";
import { NftFaucetStaking } from "../target/types/nft_faucet_staking";
import { chaiSolana, expectTX } from "@saberhq/chai-solana";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { PendingTransaction, Provider, SolanaAugmentedProvider, SolanaProvider } from "@saberhq/solana-contrib";
import { TransactionEnvelope } from "@saberhq/solana-contrib";
import { createInitMintInstructions, createMintToInstruction, getOrCreateATA, TOKEN_PROGRAM_ID, getTokenAccount, u64, createTokenAccount, createToken} from "@saberhq/token-utils";
import { sha256 } from "@project-serum/anchor/dist/cjs/utils";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { sleep } from "@saberhq/token-utils";


const anchorProvider = anchor.Provider.env();

// Configure the client to use the provider.
anchor.setProvider(anchorProvider);

const provider = SolanaProvider.init({
  connection: anchorProvider.connection,
  wallet: anchorProvider.wallet,
  opts: anchorProvider.opts,
});

const lockup = anchor.workspace.Lockup as Program<Lockup>;
const staking = anchor.workspace.NftFaucetStaking as Program<NftFaucetStaking>;

let lockupAddress = null;
const WHITELIST_SIZE = 10;


let mint = null;
let god = null;

const initializer = new Keypair();
const initializerPubkey = initializer.publicKey;

let initializerNFT0TokenAccount;

const registrar = new Keypair();
const [rewardQ] = findProgramAddressSync([registrar.publicKey.toBytes(), anchor.utils.bytes.utf8.encode("queue")], staking.programId);
const withdrawalTimelock = new anchor.BN(4);
const stakeRate = new anchor.BN(1);
const rewardQLen = 170;
let registrarAccount = null;
let registrarSigner;
const [
  _registrarSigner,
] = findProgramAddressSync(
  [registrar.publicKey.toBytes(), anchor.utils.bytes.utf8.encode("signer")],
  staking.programId
);


let memberSigner0;
registrarSigner = _registrarSigner;

const poolMintKP = new Keypair();
let poolMint = poolMintKP.publicKey;
let nftMint1;

let hash0 = sha256.hash("hash0");
let hash1 = sha256.hash("hash1");
let hash2 = sha256.hash("hash2");
let hash3 = sha256.hash("hash3");
let hash4 = sha256.hash("hash4");
let rewardMint;
let initializerMember0;

let nftMint0;

let rewardsVault;


use(chaiSolana);
describe("Lockup and Registry", () => {

  before("Set up environment", async () => {
   // Airdrop
      await expectTX(
        new PendingTransaction(
          provider.connection,
          await provider.connection.requestAirdrop(
            initializerPubkey,
            100 * LAMPORTS_PER_SOL
          )
        )
      ).to.be.fulfilled;
      const balance = await provider.connection.getBalance(initializerPubkey);
      expect(balance === 100 * LAMPORTS_PER_SOL, 'Airdrop unsuccessful');


      let poolMintTxn = await createInitMintInstructions({provider: provider, mintKP: poolMintKP, decimals:0, mintAuthority: registrarSigner});
      await expectTX(poolMintTxn).to.be.fulfilled; 


      const rewardMintKP = new Keypair();
      rewardMint = rewardMintKP.publicKey;
      let rewardMintTxn = await createInitMintInstructions({provider: provider, mintKP: rewardMintKP, decimals:0, mintAuthority: initializer.publicKey});
      await expectTX(rewardMintTxn).to.be.fulfilled; 

      const nftMint0KP = new Keypair();
      nftMint0 = nftMint0KP.publicKey;
      let nftMint0Txn = await createInitMintInstructions({provider: provider, mintKP: nftMint0KP, decimals:0, mintAuthority: initializer.publicKey});
      await expectTX(nftMint0Txn).to.be.fulfilled; 

      const nftMint1KP = new Keypair();
      nftMint1 = nftMint1KP.publicKey;
      let nftMint1Txn = await createInitMintInstructions({provider: provider, mintKP: nftMint1KP, decimals:0, mintAuthority: initializer.publicKey});
      await expectTX(nftMint1Txn).to.be.fulfilled; 


      let createRewardsVault = await createTokenAccount({mint: rewardMint,
        owner: initializer.publicKey,
        provider: provider
      });

      await expectTX(createRewardsVault.tx).to.be.fulfilled;

      rewardsVault = createRewardsVault.key;



      let atai = await getOrCreateATA({
        owner: initializerPubkey,
        mint: nftMint0,
        provider
      });

      assert.ok(atai.instruction);
      await expectTX(new TransactionEnvelope(provider, [atai.instruction])).to.be.fulfilled;
      initializerNFT0TokenAccount = atai.address;

      let mintNft0Txn = await createMintToInstruction({provider: provider, mint: nftMint0, mintAuthorityKP: initializer, to: initializerNFT0TokenAccount, amount:new u64(1)});
      await expectTX(mintNft0Txn).to.be.fulfilled;

      let rewardsVaultMintTxn = await createMintToInstruction({provider: provider, mint: rewardMint, mintAuthorityKP: initializer, to: rewardsVault, amount:new u64(255)});
      await expectTX(rewardsVaultMintTxn).to.be.fulfilled;
      rewardsVaultMintTxn = await createMintToInstruction({provider: provider, mint: rewardMint, mintAuthorityKP: initializer, to: rewardsVault, amount:new u64(255)});
      await expectTX(rewardsVaultMintTxn).to.be.fulfilled;
      rewardsVaultMintTxn = await createMintToInstruction({provider: provider, mint: rewardMint, mintAuthorityKP: initializer, to: rewardsVault, amount:new u64(255)});
      await expectTX(rewardsVaultMintTxn).to.be.fulfilled;

      [initializerMember0] = findProgramAddressSync([initializerPubkey.toBytes(), nftMint0.toBytes()], staking.programId);

      const [
        _memberSigner
      ] = findProgramAddressSync(
        [registrar.publicKey.toBytes(), initializerMember0.toBytes(), anchor.utils.bytes.utf8.encode("signer")],
        staking.programId
      );
      memberSigner0 = _memberSigner;



  });



  it("Initializes the registrar", async () => {
    let ix = await staking.instruction.initializeRegistrar(
        anchor.utils.bytes.utf8.encode(hash0),
        anchor.utils.bytes.utf8.encode(hash1),
        anchor.utils.bytes.utf8.encode(hash2),
        anchor.utils.bytes.utf8.encode(hash3),
        anchor.utils.bytes.utf8.encode(hash4),
        initializerPubkey,
        withdrawalTimelock,
        stakeRate,
        rewardQLen, 
       {
        accounts: {
          initializer: initializerPubkey,
          registrar: registrar.publicKey,
          poolMint,
          registrarSigner,
          rewardMint,
          rewardEventQ: rewardQ,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId
        },
       }
    );

    let txn = new TransactionEnvelope(provider, [ix], [registrar, initializer]);
    await expectTX(txn).to.be.fulfilled;

    registrarAccount = await staking.account.registrar.fetch(registrar.publicKey);
    assert.ok(registrarAccount.authority.equals(initializerPubkey));
    assert.ok(registrarAccount.poolMint.equals(poolMint));
    assert.ok(registrarAccount.stakeRate.eq(stakeRate));
    assert.ok(registrarAccount.rewardEventQ.equals(rewardQ));
    assert.ok(registrarAccount.withdrawalTimelock.eq(withdrawalTimelock));
  });

  it("Creates a member", async () => {
    let memberAccount = null;
    let balances = null;
    let balancesLocked = null;
    



    const _balances = await utils.createBalanceSandbox(
      provider,
      poolMint,
      nftMint0,
      memberSigner0,
      initializer
    );
    const _balancesLocked = await utils.createBalanceSandbox(
      provider,
      poolMint,
      nftMint0,
      memberSigner0,
      initializer
    );

    balances = _balances; 
    balancesLocked = _balancesLocked;
    const tx = staking.instruction.createMember(hash0,
      {
      accounts: {
        registrar: registrar.publicKey,
        member: initializerMember0,
        beneficiary: initializerPubkey,
        nftMint: nftMint0,
        memberSigner: memberSigner0,
        spt: balances.spt,
        vault: balances.vault,
        vaultStake: balances.vaultStake,
        vaultPw: balances.vaultPw,
        lockedVault: balancesLocked.vault,
        lockedVaultStake: balancesLocked.vaultStake,
        lockedSpt: balancesLocked.spt,
        lockedVaultPw: balancesLocked.vaultPw,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,

      }
    }
      )

    const signers = [initializer];

    const txn = new TransactionEnvelope(provider, [tx], [initializer]);

    await expectTX(txn).to.be.fulfilled;

    memberAccount = await staking.account.member.fetch(initializerMember0);

    assert.ok(memberAccount.registrar.equals(registrar.publicKey));
    assert.ok(memberAccount.beneficiary.equals(initializerPubkey));
    assert.ok(memberAccount.spt.equals(balances.spt));
    assert.ok(memberAccount.vault.equals(balances.vault));
    assert.ok(memberAccount.vaultStake.equals(balances.vaultStake));
    assert.ok(memberAccount.vaultPw.equals(balances.vaultPw));
    assert.ok(memberAccount.lockedSpt.equals(balancesLocked.spt));
    assert.ok(memberAccount.lockedVault.equals(balancesLocked.vault));
    assert.ok(memberAccount.lockedVaultStake.equals(balancesLocked.vaultStake));
    assert.ok(memberAccount.lockedVaultPw.equals(balancesLocked.vaultPw));
    assert.ok(memberAccount.metadata.equals(anchor.web3.PublicKey.default));
    assert.ok(memberAccount.rewardsCursor === 0);
    assert.ok(memberAccount.lastStakeTs.eq(new anchor.BN(0)));
  });

  it("Deposits (unlocked) to a member", async () => {
    let memberAccount = await staking.account.member.fetch(initializerMember0);
    const depositAmount = new anchor.BN(1);
    let ix = await staking.instruction.deposit(depositAmount, {
      accounts: {
        depositor: initializerNFT0TokenAccount,
        depositorAuthority: initializerPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
        vault: memberAccount.vault,
        beneficiary: initializerPubkey,
        member: initializerMember0,
      },
    });

    const txn = new TransactionEnvelope(provider, [ix], [initializer]);

    await expectTX(txn).to.be.fulfilled;

    const memberVault = await getTokenAccount(
      provider,
      memberAccount.vault
    );
    assert.ok(memberVault.amount.eq(depositAmount));
  });

  it("Stakes to a member (unlocked)", async () => {
    let memberAccount = await staking.account.member.fetch(initializerMember0);

    

    let ix = await staking.instruction.stake(false, {
      accounts: {
        // Stake instance.
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ,
        poolMint,
        // Member.
        member: initializerMember0,
        beneficiary:initializerPubkey,
        spt: memberAccount.spt,
        lockedSpt: memberAccount.lockedSpt,
        vault: memberAccount.vault,
        vaultStake: memberAccount.vaultStake,
        vaultPw: memberAccount.vaultPw,
        lockedVault: memberAccount.lockedVault,
        lockedVaultStake: memberAccount.lockedVaultStake,
        lockedVaultPw: memberAccount.lockedVaultPw,
        // Program signers.
        memberSigner: memberSigner0,
        registrarSigner,
        // Misc.
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    const txn = new TransactionEnvelope(provider, [ix], [initializer]);

    await expectTX(txn).to.be.fulfilled;

   const vaultAccount = await getTokenAccount(provider,memberAccount.vault);
   const spt = await getTokenAccount(provider, memberAccount.spt);
   const vaultStake = await getTokenAccount(provider, memberAccount.vaultStake);
    assert.ok(vaultAccount.amount.eq(new anchor.BN(0)));
    assert.ok(vaultStake.amount.eq(new anchor.BN(1)));
    assert.ok(spt.amount.eq(new anchor.BN(1)));
  });

  const unlockedVendor = new Keypair();
  let unlockedVendorSigner = null;
  let unlockedVendorVault;


  it("Drops an unlocked reward", async () => {
    const rewardKind = {
      unlocked: {},
    };
    const rewardAmount = new anchor.BN(200);
    const expiry = new anchor.BN(Date.now() / 1000 + 5);
    const [
      _vendorSigner,
      nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
      [registrar.publicKey.toBuffer(), unlockedVendor.publicKey.toBuffer(), anchor.utils.bytes.utf8.encode("signer")],
      staking.programId
    );
    unlockedVendorSigner = _vendorSigner;


    const createVaultTxn = await createTokenAccount({provider: provider, mint: rewardMint, owner: unlockedVendorSigner});
    await expectTX(createVaultTxn.tx).to.be.fulfilled;

    unlockedVendorVault = createVaultTxn.key;

    let ix = await staking.instruction.dropReward(
      rewardKind,
      rewardAmount,
      expiry,
      initializerPubkey,
      {
        accounts: {
          registrar: registrar.publicKey,
          rewardEventQ: rewardQ,
          poolMint,
          vendor: unlockedVendor.publicKey,
          vendorVault: unlockedVendorVault,
          depositor: rewardsVault,
          depositorAuthority: initializerPubkey,
          vendorSigner: unlockedVendorSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        }
      }
    );

    let txn = new TransactionEnvelope(provider, [ix], [initializer, unlockedVendor]);

    await expectTX(txn).to.be.fulfilled;

    const vendorAccount = await staking.account.rewardVendor.fetch(unlockedVendor.publicKey);

    assert.ok(vendorAccount.registrar.equals(registrar.publicKey));
    assert.ok(vendorAccount.vault.equals(unlockedVendorVault));
    assert.ok(vendorAccount.poolTokenSupply.eq(new anchor.BN(1)));
    assert.ok(vendorAccount.expiryTs.eq(expiry));
    assert.ok(vendorAccount.expiryReceiver.equals(initializerPubkey));
    assert.ok(vendorAccount.total.eq(rewardAmount));
    assert.ok(vendorAccount.expired === false);
    assert.ok(vendorAccount.rewardEventQCursor === 0);
    assert.deepEqual(vendorAccount.kind, rewardKind);

    const rewardQAccount = await staking.account.rewardQueue.fetch(rewardQ);
    assert.ok(rewardQAccount.head === 1);
    assert.ok(rewardQAccount.tail === 0);
    const e = rewardQAccount.events[0];
    assert.ok(e.vendor.equals(unlockedVendor.publicKey));
    assert.equal(e.locked, false);
  });

  it("Collects an unlocked reward", async () => {
    let memberAccount = await staking.account.member.fetch(initializerMember0);
    const token = await createTokenAccount({
      provider: provider,
      mint: rewardMint,
      owner: initializerMember0

    });

    await expectTX(token.tx).to.be.fulfilled;
    let ix = await staking.instruction.claimUnlockedReward({
      accounts: {
        to: token.key,
        cmn: {
          registrar: registrar.publicKey,
          member: initializerMember0,
          beneficiary: initializerPubkey,
          spt: memberAccount.spt,
          vault: memberAccount.vault,
          vaultStake: memberAccount.vaultStake,
          vaultPw: memberAccount.vaultPw,
          lockedSpt: memberAccount.lockedSpt,
          lockedVault: memberAccount.lockedVault,
          lockedVaultStake: memberAccount.lockedVaultStake,
          lockedVaultPw: memberAccount.lockedVaultPw,
          vendor: unlockedVendor.publicKey,
          vestingVault: unlockedVendorVault,
          vendorSigner: unlockedVendorSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
      },
    });

    let txn  = new TransactionEnvelope(provider, [ix], [initializer]);
    await expectTX(txn).to.be.fulfilled;

    //reload member account
    memberAccount = await staking.account.member.fetch(initializerMember0);
    let tokenAccount = await getTokenAccount(provider, token.key);
    assert.ok(tokenAccount.amount.eq(new anchor.BN(200)));
    assert.ok(memberAccount.rewardsCursor === 1);
  });

  const lockedVendor = new Keypair();
  let lockedVendorVault;
  let lockedRewardKind;
  let lockedVendorSigner;
  let lockedRewardAmount;

  // it("Drops a locked reward", async () => {
  //   lockedRewardKind = {
  //     locked: {
  //       startTs: new anchor.BN(Date.now() / 1000),
  //       endTs: new anchor.BN(Date.now() / 1000 + 6),
  //       periodCount: new anchor.BN(2),
  //     },
  //   };
  //   lockedRewardAmount = new anchor.BN(200);
  //   const expiry = new anchor.BN(Date.now() / 1000 + 5);
  //   const [
  //     _vendorSigner
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //     [registrar.publicKey.toBytes(), lockedVendor.publicKey.toBytes(), anchor.utils.bytes.utf8.encode("signer")],
  //     staking.programId
  //   );
  //   lockedVendorSigner = _vendorSigner;

  //   const createVaultTxn = await createTokenAccount({provider: provider, mint: rewardMint, owner: lockedVendorSigner});
  //   await expectTX(createVaultTxn.tx).to.be.fulfilled;
  //   lockedVendorVault = createVaultTxn.key;

  //   console.log("accts",{
  //     registrar: registrar.publicKey.toBase58(),
  //     rewardEventQ: rewardQ.toBase58(),
  //     poolMint: poolMint.toBase58(),
  //     vendor: lockedVendor.publicKey.toBase58(),
  //     vendorVault: lockedVendorVault.toBase58(),
  //     vendorSigner: lockedVendorSigner.toBase58(),
  //     depositor: rewardsVault.toBase58(),
  //     depositorAuthority: initializer.publicKey.toBase58(),
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
  //     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     systemProgram: SystemProgram.programId
  //   } )


  //   let ix =  staking.instruction.dropReward(
  //     lockedRewardKind,
  //     lockedRewardAmount,
  //     expiry,
  //     initializer.publicKey,
  //     {
  //       accounts: {
  //         registrar: registrar.publicKey,
  //         rewardEventQ: rewardQ,
  //         poolMint,
  //         vendor: lockedVendor.publicKey,
  //         vendorVault: lockedVendorVault,
  //         vendorSigner: lockedVendorSigner,
  //         depositor: rewardsVault,
  //         depositorAuthority: initializer.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
  //         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId
  //       }
  //     }
  //   );



  //   const txn = new TransactionEnvelope(provider, [ix], [initializer , lockedVendor]);
  //   await expectTX(txn).to.be.fulfilled;


  //   const vendorAccount = await staking.account.rewardVendor.fetch(lockedVendor.publicKey);

  //   assert.ok(vendorAccount.registrar.equals(registrar.publicKey));
  //   assert.ok(vendorAccount.vault.equals(lockedVendorVault));
  //   assert.ok(vendorAccount.poolTokenSupply.eq(new anchor.BN(1)));
  //   assert.ok(vendorAccount.expiryTs.eq(expiry));
  //   assert.ok(vendorAccount.expiryReceiver.equals(initializerPubkey));
  //   assert.ok(vendorAccount.total.eq(lockedRewardAmount));
  //   assert.ok(vendorAccount.expired === false);
  //   assert.ok(vendorAccount.rewardEventQCursor === 1);
  //   assert.equal(
  //     JSON.stringify(vendorAccount.kind),
  //     JSON.stringify(lockedRewardKind)
  //   );

  //   const rewardQAccount = await staking.account.rewardQueue.fetch(
  //     rewardQ
  //   );
  //   assert.ok(rewardQAccount.head === 2);
  //   assert.ok(rewardQAccount.tail === 0);
  //   const e = rewardQAccount.events[1];
  //   assert.ok(e.vendor.equals(lockedVendor.publicKey));
  //   assert.ok(e.locked === true);
  // });

  // let vendoredVesting = null;
  // let vendoredVestingVault = null;
  // let vendoredVestingSigner = null;

  // it("Claims a locked reward", async () => {
  //   vendoredVesting = new anchor.web3.Account();
  //   vendoredVestingVault = new anchor.web3.Account();
  //   let [
  //     _vendoredVestingSigner,
  //     nonce,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //     [vendoredVesting.publicKey.toBuffer()],
  //     lockup.programId
  //   );

  //   vendoredVestingSigner = _vendoredVestingSigner;
  //   let memberAccount = await staking.account.member.fetch(initializerMember0);

    

  //   await staking.instruction.claimLockedReward( 
  //     new Keypair().publicKey,
  //     {
  //     accounts: {
  //       registry: await staking.account.registry.all()[0],
  //       lockupProgram: lockup.programId,
  //       cmn: {
  //         registrar: registrar.publicKey,
  //         member: initializerMember0,
  //         beneficiary: initializerPubkey,
  //         spt: memberAccount.spt,
  //         vault: memberAccount.vault,
  //         vaultStake: memberAccount.vaultStake,
  //         vaultPw: memberAccount.vaultPw,
  //         lockedSpt: memberAccount.lockedSpt,
  //         lockedVault: memberAccount.lockedVault,
  //         lockedVaultStake: memberAccount.lockedVaultStake,
  //         lockedVaultPw: memberAccount.lockedVaultPw,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
  //         vendor: lockedVendor.publicKey,
  //         vestingVault: lockedVendorVault.publicKey,
  //         vendorSigner: lockedVendorSigner,
  //       },
  //     },
  //   }
  // );


  //   const lockupAccount = await lockup.account.vesting.fetch(
  //     vendoredVesting.publicKey
  //   );

  //   assert.ok(lockupAccount.beneficiary.equals(provider.wallet.publicKey));
  //   assert.ok(lockupAccount.mint.equals(mint));
  //   assert.ok(lockupAccount.vault.equals(vendoredVestingVault.publicKey));
  //   assert.ok(lockupAccount.outstanding.eq(lockedRewardAmount));
  //   assert.ok(lockupAccount.startBalance.eq(lockedRewardAmount));
  //   assert.ok(lockupAccount.endTs.eq(lockedRewardKind.locked.endTs));
  //   assert.ok(
  //     lockupAccount.periodCount.eq(lockedRewardKind.locked.periodCount)
  //   );
  //   assert.ok(lockupAccount.whitelistOwned.eq(new anchor.BN(0)));
  //   assert.ok(lockupAccount.realizor.program.equals(staking.programId));
  //   assert.ok(lockupAccount.realizor.metadata.equals(initializerMember0));
  // });

  it("Waits for the lockup period to pass", async () => {
    await sleep(10 * 1000);
  });

  // it("Should fail to unlock an unrealized lockup reward", async () => {
  //   const token = await utils.sreumCmn.createTokenAccount(
  //     provider,
  //     mint,
  //     provider.wallet.publicKey
  //   );
  //   await assert.rejects(
  //     async () => {
  //       const withdrawAmount = new anchor.BN(10);
  //       await lockup.rpc.withdraw(withdrawAmount, {
  //         accounts: {
  //           vesting: vendoredVesting.publicKey,
  //           beneficiary: provider.wallet.publicKey,
  //           token,
  //           vault: vendoredVestingVault.publicKey,
  //           vestingSigner: vendoredVestingSigner,
  //           tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
  //           clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
  //         },
  //         // TODO: trait methods generated on the client. Until then, we need to manually
  //         //       specify the account metas here.
  //         remainingAccounts: [
  //           { pubkey: registry.programId, isWritable: false, isSigner: false },
  //           { pubkey: member.publicKey, isWritable: false, isSigner: false },
  //           { pubkey: balances.spt, isWritable: false, isSigner: false },
  //           { pubkey: balancesLocked.spt, isWritable: false, isSigner: false },
  //         ],
  //       });
  //     },
  //     (err) => {
  //       // Solana doesn't propagate errors across CPI. So we receive the registry's error code,
  //       // not the lockup's.
  //       const errorCode = "custom program error: 0x78";
  //       assert.ok(err.toString().split(errorCode).length === 2);
  //       return true;
  //     }
  //   );
  // });

  const pendingWithdrawal = new Keypair();

  it("Unstakes (unlocked)", async () => {
    const unstakeAmount = new anchor.BN(1);
    let memberAccount = await staking.account.member.fetch(initializerMember0);

    let ix = await staking.instruction.startUnstake(unstakeAmount, false, 
      {
      accounts: {
        registrar: registrar.publicKey,
        rewardEventQ: rewardQ,
        poolMint,
        pendingWithdrawal: pendingWithdrawal.publicKey,
        member: initializerMember0,
        beneficiary: initializer.publicKey,
        spt: memberAccount.spt,
        lockedSpt: memberAccount.lockedSpt,
        vault: memberAccount.vault,
        vaultStake: memberAccount.vaultStake,
        vaultPw: memberAccount.vaultPw,
        lockedVault: memberAccount.lockedVault,
        lockedVaultStake: memberAccount.lockedVaultStake,
        lockedVaultPw: memberAccount.lockedVaultPw,
        // Program signers.
        memberSigner: memberSigner0,
        // Misc.
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      },
    });

    let txn = new TransactionEnvelope(provider, [ix], [initializer, pendingWithdrawal]);
    await expectTX(txn).to.be.fulfilled;

    const vaultPw = await getTokenAccount(
      provider,
      memberAccount.vaultPw
    );
    const vaultStake = await getTokenAccount(
      provider,
      memberAccount.vaultStake
    );
    const spt = await getTokenAccount(
      provider,
      memberAccount.spt
    );

    assert.ok(vaultPw.amount.eq(new anchor.BN(1)));
    assert.ok(vaultStake.amount.eq(new anchor.BN(0)));
    assert.ok(spt.amount.eq(new anchor.BN(0)));
  });

  const getEndUnstakeTxn = async () => {
    let memberAccount = await staking.account.member.fetch(initializerMember0);

    let ix = await staking.instruction.endUnstake({
      accounts: {
        registrar: registrar.publicKey,
        pendingWithdrawal: pendingWithdrawal.publicKey,
        member: initializerMember0,
        beneficiary: initializer.publicKey,
        vault: memberAccount.vault,
        vaultPw: memberAccount.vaultPw,
        // Program signers.
        memberSigner: memberSigner0,
        // Misc.
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    return new TransactionEnvelope(provider, [ix], [initializer]);
  };

  it("Fails to end unstaking before timelock", async () => {

    await expectTX(await getEndUnstakeTxn()).to.be.rejected;
  });

  it("Waits for the unstake period to end", async () => {
    await sleep(5000);
  });

  it("Unstake finalizes (unlocked)", async () => {
    let memberAccount = await staking.account.member.fetch(initializerMember0);
    await expectTX(await getEndUnstakeTxn()).to.be.fulfilled;

    const vault = await getTokenAccount(
      provider,
      memberAccount.vault
    );
    const vaultPw = await getTokenAccount(
      provider,
      memberAccount.vaultPw
    );


    assert.ok(vault.amount.eq(new anchor.BN(1)));
    assert.ok(vaultPw.amount.eq(new anchor.BN(0)));
  });

  it("Withdraws deposits (unlocked)", async () => {
    let memberAccount = await staking.account.member.fetch(initializerMember0);
    const token = await createTokenAccount({
      provider: provider,
      mint: nftMint0,
     owner: initializer.publicKey
    } 
    );
    await expectTX(token.tx).to.be.fulfilled;
    const withdrawAmount = new anchor.BN(1);
    let ix = await staking.instruction.withdraw(withdrawAmount, {
      accounts: {
        registrar: registrar.publicKey,
        member: initializerMember0,
        beneficiary: initializer.publicKey,
        vault: memberAccount.vault,
        memberSigner: memberSigner0,
        depositor: token.key,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });

    await expectTX(new TransactionEnvelope(provider, [ix], [initializer])).to.be.fulfilled;

    const tokenAccount = await getTokenAccount(provider, token.key);
    assert.ok(tokenAccount.amount.eq(withdrawAmount));
  });

  // it("Should succesfully unlock a locked reward after unstaking", async () => {
  //   const token = await utils.sreumCmn.createTokenAccount(
  //     provider,
  //     mint,
  //     provider.wallet.publicKey
  //   );

  //   const withdrawAmount = new anchor.BN(7);
  //   await lockup.rpc.withdraw(withdrawAmount, {
  //     accounts: {
  //       vesting: vendoredVesting.publicKey,
  //       beneficiary: provider.wallet.publicKey,
  //       token,
  //       vault: vendoredVestingVault.publicKey,
  //       vestingSigner: vendoredVestingSigner,
  //       tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
  //       clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
  //     },
  //     // TODO: trait methods generated on the client. Until then, we need to manually
  //     //       specify the account metas here.
  //     remainingAccounts: [
  //       { pubkey: registry.programId, isWritable: false, isSigner: false },
  //       { pubkey: member.publicKey, isWritable: false, isSigner: false },
  //       { pubkey: balances.spt, isWritable: false, isSigner: false },
  //       { pubkey: balancesLocked.spt, isWritable: false, isSigner: false },
  //     ],
  //   });
  //   const tokenAccount = await utils.sreumCmn.getTokenAccount(provider, token);
  //   assert.ok(tokenAccount.amount.eq(withdrawAmount));
  // });
});
