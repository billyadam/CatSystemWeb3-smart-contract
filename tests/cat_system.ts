import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { CatSystem } from "../target/types/cat_system";
import { assert } from "chai";

describe("cat_system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CatSystem as Program<CatSystem>;

  it("creates a cat", async () => {
    const owner = provider.wallet.publicKey;

    const [catPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("cat"), owner.toBuffer()],
      program.programId
    );

    await program.methods
      .createCat(
        "Whiskers",
        { female: {} },
        "A fluffy white cat who loves napping in sunbeams."
      )
      .accounts({ owner })
      .rpc();

    const cat = await program.account.cat.fetch(catPda);
    assert.equal(cat.name, "Whiskers");
    assert.deepEqual(cat.gender, { female: {} });
    assert.equal(cat.description, "A fluffy white cat who loves napping in sunbeams.");
    assert.ok(cat.owner.equals(owner));
  });
});
