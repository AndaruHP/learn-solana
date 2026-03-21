import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DeployTutorial } from "../target/types/deploy_tutorial";

import fs from 'fs'
let idl = JSON.parse(fs.readFileSync('target/idl/deployed.json', 'utf-8'))

describe("deployed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "A7DDqXWWR8nDXQEfm7nViBX84N8RFQmrRa3hnDrkqBge";
  const program = new Program(idl, programID, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
