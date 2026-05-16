import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { CatSystem } from "../target/types/cat_system";
import { assert } from "chai";

describe("cat_system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CatSystem as Program<CatSystem>;

  it("creates a cat and emits CatCreated", async () => {
    const owner = provider.wallet.publicKey;

    const [userCounterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_counter"), owner.toBuffer()],
      program.programId
    );

    // First registration: cat_count is 0, so cat seed uses 0_u64 little-endian.
    const catIndex = new anchor.BN(0);
    const [catPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("cat"), owner.toBuffer(), catIndex.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const imageUrl1 = "/uploads/cats/owner-uuid-1.jpg";
    const imageUrl2 = "/uploads/cats/owner-uuid-2.jpg";

    const eventPromise = new Promise<any>((resolve) => {
      const listener = program.addEventListener("CatCreated", (event) => {
        program.removeEventListener(listener);
        resolve(event);
      });
    });

    await program.methods
      .createCat(
        "Whiskers",
        { female: {} },
        "Persian",
        "White",
        { long: {} },
        "Blue",
        { rounded: {} },
        { medium: {} },
        "A fluffy white cat who loves napping in sunbeams.",
        imageUrl1,
        imageUrl2
      )
      .accounts({
        owner,
        userCounter: userCounterPda,
        cat: catPda,
      })
      .rpc();

    const cat = await program.account.cat.fetch(catPda);
    assert.equal(cat.name, "Whiskers");
    assert.deepEqual(cat.gender, { female: {} });
    assert.equal(cat.description, "A fluffy white cat who loves napping in sunbeams.");
    assert.equal(cat.imageUrl1, imageUrl1);
    assert.equal(cat.imageUrl2, imageUrl2);
    assert.ok(cat.owner.equals(owner));

    const event = await eventPromise;
    assert.ok(event.cat.equals(catPda));
    assert.ok(event.owner.equals(owner));
    assert.equal(event.name, "Whiskers");
    assert.equal(event.imageUrl1, imageUrl1);
    assert.equal(event.imageUrl2, imageUrl2);
    assert.equal(event.catIndex.toNumber(), 0);

    const counter = await program.account.userCounter.fetch(userCounterPda);
    assert.equal(counter.catCount.toNumber(), 1);
  });
});
