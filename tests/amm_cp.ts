import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AmmCp } from "../target/types/amm_cp";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { assert } from "chai";

describe("amm_cp — constant-product AMM", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ammCp as Program<AmmCp>;
  const payer = (provider.wallet as anchor.Wallet).payer;

  const FEE_BPS = 30; // 0.3%

  let mintA: PublicKey;
  let mintB: PublicKey;
  let pool: PublicKey;
  let vaultA: PublicKey;
  let vaultB: PublicKey;
  let lpMint: PublicKey;
  let userTokenA: PublicKey;
  let userTokenB: PublicKey;
  let userLp: PublicKey;

  before(async () => {
    // Create two mints and order them canonically (mint_a < mint_b).
    const m1 = await createMint(provider.connection, payer, payer.publicKey, null, 6);
    const m2 = await createMint(provider.connection, payer, payer.publicKey, null, 6);
    [mintA, mintB] = Buffer.compare(m1.toBuffer(), m2.toBuffer()) < 0 ? [m1, m2] : [m2, m1];

    [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), mintA.toBuffer(), mintB.toBuffer()],
      program.programId
    );
    [vaultA] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_a"), pool.toBuffer()],
      program.programId
    );
    [vaultB] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_b"), pool.toBuffer()],
      program.programId
    );
    [lpMint] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp_mint"), pool.toBuffer()],
      program.programId
    );

    // User token accounts, funded with both mints.
    userTokenA = (
      await getOrCreateAssociatedTokenAccount(provider.connection, payer, mintA, payer.publicKey)
    ).address;
    userTokenB = (
      await getOrCreateAssociatedTokenAccount(provider.connection, payer, mintB, payer.publicKey)
    ).address;

    await mintTo(provider.connection, payer, mintA, userTokenA, payer, 1_000_000_000);
    await mintTo(provider.connection, payer, mintB, userTokenB, payer, 1_000_000_000);
  });

  it("initializes a pool", async () => {
    await program.methods
      .initializePool(FEE_BPS)
      .accountsPartial({
        payer: payer.publicKey,
        mintA,
        mintB,
        pool,
        vaultA,
        vaultB,
        lpMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    const p = await program.account.pool.fetch(pool);
    assert.equal(p.feeBps, FEE_BPS);
    assert.ok(p.mintA.equals(mintA));
    assert.ok(p.mintB.equals(mintB));

    // Create the user's LP token account now that the LP mint exists.
    userLp = (
      await getOrCreateAssociatedTokenAccount(provider.connection, payer, lpMint, payer.publicKey)
    ).address;
  });

  it("adds initial liquidity (LP = sqrt(a*b))", async () => {
    const amountA = new anchor.BN(1_000_000);
    const amountB = new anchor.BN(4_000_000);

    await program.methods
      .addLiquidity(amountA, amountB, new anchor.BN(0))
      .accountsPartial({
        user: payer.publicKey,
        pool,
        vaultA,
        vaultB,
        lpMint,
        userTokenA,
        userTokenB,
        userLp,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const va = await getAccount(provider.connection, vaultA);
    const vb = await getAccount(provider.connection, vaultB);
    const lp = await getAccount(provider.connection, userLp);

    assert.equal(va.amount.toString(), "1000000");
    assert.equal(vb.amount.toString(), "4000000");
    // sqrt(1_000_000 * 4_000_000) = sqrt(4e12) = 2_000_000
    assert.equal(lp.amount.toString(), "2000000");
  });

  it("swaps A for B along the constant product", async () => {
    const vbBefore = (await getAccount(provider.connection, vaultB)).amount;
    const userBBefore = (await getAccount(provider.connection, userTokenB)).amount;

    const amountIn = new anchor.BN(100_000);
    await program.methods
      .swap(amountIn, new anchor.BN(1), true) // a_to_b
      .accountsPartial({
        user: payer.publicKey,
        pool,
        vaultA,
        vaultB,
        userTokenA,
        userTokenB,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const vaAfter = (await getAccount(provider.connection, vaultA)).amount;
    const userBAfter = (await getAccount(provider.connection, userTokenB)).amount;

    // Vault A grew by the input.
    assert.equal(vaAfter.toString(), "1100000");
    // User received some B, and the pool paid out of vault B.
    const received = userBAfter - userBBefore;
    assert.ok(received > 0n, "user should receive token B");
    assert.equal((vbBefore - received).toString(), (await getAccount(provider.connection, vaultB)).amount.toString());

    // Expected out (constant product, 0.3% fee):
    // in_after_fee = 100000 * 9970 / 10000 = 99700
    // out = 4000000 * 99700 / (1000000 + 99700) = 362_644 (floor)
    assert.equal(received.toString(), "362644");
  });

  it("removes liquidity proportionally", async () => {
    const lpBefore = (await getAccount(provider.connection, userLp)).amount;
    const burn = new anchor.BN(500_000); // a quarter of the 2_000_000 LP supply

    const aBefore = (await getAccount(provider.connection, userTokenA)).amount;
    const bBefore = (await getAccount(provider.connection, userTokenB)).amount;

    await program.methods
      .removeLiquidity(burn, new anchor.BN(0), new anchor.BN(0))
      .accountsPartial({
        user: payer.publicKey,
        pool,
        vaultA,
        vaultB,
        lpMint,
        userTokenA,
        userTokenB,
        userLp,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const lpAfter = (await getAccount(provider.connection, userLp)).amount;
    const aAfter = (await getAccount(provider.connection, userTokenA)).amount;
    const bAfter = (await getAccount(provider.connection, userTokenB)).amount;

    assert.equal((lpBefore - lpAfter).toString(), "500000");
    // Burning 25% of supply returns ~25% of each reserve.
    assert.ok(aAfter > aBefore, "should receive token A back");
    assert.ok(bAfter > bBefore, "should receive token B back");
  });

  it("rejects a swap that violates slippage", async () => {
    try {
      await program.methods
        .swap(new anchor.BN(1000), new anchor.BN(10_000_000), true) // absurd min_out
        .accountsPartial({
          user: payer.publicKey,
          pool,
          vaultA,
          vaultB,
          userTokenA,
          userTokenB,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      assert.fail("swap should have failed on slippage");
    } catch (err: any) {
      assert.include(err.toString(), "SlippageExceeded");
    }
  });
});
