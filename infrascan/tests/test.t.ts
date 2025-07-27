import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConstructionTransparency } from "../target/types/construction_transparency";
import { assert } from "chai";

describe("construction_transparency - create_project", () => {
  // Set provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Fetch the program
  const program = anchor.workspace.ConstructionTransparency as Program<ConstructionTransparency>;

  // Generate a new keypair for the contractor (just needs to be a pubkey, not necessarily a wallet)
  const contractor = anchor.web3.Keypair.generate();

  // Sample project data
  const projectId = new anchor.BN(1);
  const name = "Smart Highway Project";
  const location = "Mumbai";

  // Compute PDA
  let projectPda: anchor.web3.PublicKey;
  let bump: number;

  before(async () => {
    [projectPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project"),
        provider.wallet.publicKey.toBuffer(),
        projectId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
  });

  it("should create a project successfully", async () => {
    // Send transaction
    await program.methods
      .createProject(projectId, name, location)
      .accounts({
        project: projectPda,
        authority: provider.wallet.publicKey,
        contractor: contractor.publicKey,
      })
      .signers([])
      .rpc();

    // Fetch the on-chain account
    const projectAccount = await program.account.constructionProject.fetch(projectPda);

    // Validate fields
    assert.equal(projectAccount.projectId.toString(), projectId.toString());
    assert.equal(projectAccount.name, name);
    assert.equal(projectAccount.location, location);
    assert.equal(projectAccount.authority.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(projectAccount.contractor.toBase58(), contractor.publicKey.toBase58());
    assert.deepEqual(projectAccount.materials, []);
    assert.deepEqual(projectAccount.docHashes, []);
    assert.equal(projectAccount.verified, false);
    assert.ok(typeof projectAccount.timestamp === "number" || typeof projectAccount.timestamp === "bigint");
  });
});
