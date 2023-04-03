import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SolPost } from "../target/types/sol_post";
import { assert } from "chai";

describe("sol-post", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolPost as Program<SolPost>;
  const baseAccount = anchor.web3.Keypair.generate();

  before(async () => {
    await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
            baseAccount.publicKey,
            2 * LAMPORTS_PER_SOL,
        ),
        "confirmed"
    );
  });

  it("creates a post", async () => {
    const postText = "Hello, World!";
    const seeds = [
      anchor.utils.bytes.utf8.encode("post_account"),
      baseAccount.publicKey.toBytes()
    ];
    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seeds,
        program.programId
    );

    await program.rpc.createPost(postText, {
      accounts: {
        author: baseAccount.publicKey,
        postAccount: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    const postAccount = await program.account.postAccount.fetch(pda);


    assert.strictEqual(postAccount.author.toString(), baseAccount.publicKey.toString());
    assert.strictEqual(postAccount.text, postText);
    assert.strictEqual(postAccount.bump, bump);
    assert.strictEqual(postAccount.isInitialized, true);
  });

  it("deletes a post", async () => {
    const seeds = [
      anchor.utils.bytes.utf8.encode("post_account"),
      baseAccount.publicKey.toBytes()
    ];
    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seeds,
        program.programId
    );

    await program.rpc.deletePost({
      accounts: {
        author: baseAccount.publicKey,
        postAccount: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    const postAccount = await program.account.postAccount.fetch(pda);
    assert.strictEqual(postAccount.author.toString(), anchor.web3.PublicKey.default.toString());
    assert.strictEqual(postAccount.text, "");
    assert.strictEqual(postAccount.bump, 0);
    assert.strictEqual(postAccount.isInitialized, false);
  });
});
