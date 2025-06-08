import * as anchor from "@coral-xyz/anchor";
import { Program, SystemProgram } from "@coral-xyz/anchor";
import { DappsCampaign } from "../target/types/dapps_campaign";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("dapps-campaign", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.dappsCampaign as Program<DappsCampaign>;
  const campaignKeyPair = anchor.web3.Keypair.generate();

  const key = new Keypair();
  const target = new anchor.BN(1 * LAMPORTS_PER_SOL);

  it("Create a campaign", async () => {
    const tx = await program.methods
      .createCampaign("Save the Whales", "Help save ocean wildlife.", target)
      .accounts({
        signer: key.publicKey,
        campaign: campaignKeyPair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([campaignKeyPair])
      .rpc();

    const campaign = await program.account.campaign.fetch(
      campaignKeyPair.publicKey
    );

    console.log("Campaign: ", campaign);
  });
});
