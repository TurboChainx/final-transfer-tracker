import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { TransferTracker } from "../target/types/transfer_tracker"; // Adjust the import as per your program's name

const { SystemProgram } = web3;

describe("transfer-tracker", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.transferTracker as Program<TransferTracker>;

  let ownerAccount: web3.Keypair;
  let transferRecord: web3.Keypair;

  before(async () => {
    // Create the owner account (this will be the account for testing ownership)
    ownerAccount = web3.Keypair.generate();
    transferRecord = web3.Keypair.generate();
  });

  it("Initializes the owner", async () => {
    // Call the `initializeOwner` method.
    const tx = await program.methods
      .initializeOwner()
      .accounts({
        ownerAccount: ownerAccount.publicKey, // Pass publicKey as expected
        signer: ownerAccount.publicKey, // Signer is also the owner
        systemProgram: SystemProgram.programId,
      })
      .signers([ownerAccount])
      .rpc();

    console.log("Your transaction signature", tx);

    // Assert the owner account is initialized properly
    const accountInfo = await program.account.ownerAccount.fetch(ownerAccount.publicKey);
    expect(accountInfo.owner.toString()).toBe(ownerAccount.publicKey.toString());
  });

  it("Adds a new transfer", async () => {
    // Create fake transfer data
    const signature_1 = "signature_part_1";
    const signature_2 = "signature_part_2";
    const signature_3 = "signature_part_3";
    const from = web3.Keypair.generate().publicKey;
    const to = web3.Keypair.generate().publicKey;
    const amount = 100.23; // Ensure it's a BigNumber (BN) for large numbers
    const timestamp = Math.floor(Date.now() / 1000); // Current time in seconds
    const wallet_balance = 5000.0;
    const sol_price = 0.02;
    const token_price = 1.5;

    // Generate the PDA for the transfer record using signature parts
    const [transferPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("transfer"), Buffer.from(signature_1), Buffer.from(signature_2), Buffer.from(signature_3)],
      program.programId
    );

    // Call the `addTransfer` method
    const tx = await program.methods
      .addTransfer(
        signature_1,
        signature_2,
        signature_3,
        from,
        to,
        amount.toString(), // Convert BN to string for passing as f64 in the program
        timestamp,
        wallet_balance,
        sol_price,
        token_price
      )
      .accounts({
        ownerAccount: ownerAccount.publicKey,
        transferRecord: transferPda, // Use the generated PDA for transferRecord
        signer: ownerAccount.publicKey, // The signer should be the owner
        systemProgram: SystemProgram.programId,
      })
      .signers([ownerAccount, transferRecord])
      .rpc();

    console.log("Your transaction signature", tx);

    // Add assertions to check if the transfer was successfully added
    const transferInfo = await program.account.transferRecord.fetch(transferPda);
    expect(transferInfo.signature_1).toBe(signature_1);
    expect(transferInfo.signature_2).toBe(signature_2);
    expect(transferInfo.signature_3).toBe(signature_3);
    expect(transferInfo.from.toString()).toBe(from.toString());
    expect(transferInfo.to.toString()).toBe(to.toString());
    expect(transferInfo.amount.toString()).toBe(amount.toString());
  });

  it("Updates an existing transfer", async () => {
    // New values for the update
    const new_token_price = 1.8;
    const new_sol_price = 0.025;
    const new_wallet_balance = 6000.0;

    // Call the `updateTransfer` method
    const tx = await program.methods
      .updateTransfer(new_token_price, new_sol_price, new_wallet_balance)
      .accounts({
        ownerAccount: ownerAccount.publicKey,
        transferRecord: transferRecord.publicKey,
        signer: ownerAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([ownerAccount])
      .rpc();

    console.log("Your transaction signature", tx);

    // Fetch and assert the updated transfer record
    const transferInfo = await program.account.transferRecord.fetch(transferRecord.publicKey);
    expect(transferInfo.wallet_balance).toBe(new_wallet_balance);
    expect(transferInfo.sol_price).toBe(new_sol_price);
    expect(transferInfo.token_price).toBe(new_token_price);
  });

  it("Transfers ownership", async () => {
    // Generate a new owner
    const newOwner = web3.Keypair.generate().publicKey;

    // Call the `transferOwnership` method
    const tx = await program.methods
      .transferOwnership(newOwner)
      .accounts({
        ownerAccount: ownerAccount.publicKey,
        signer: ownerAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([ownerAccount])
      .rpc();

    console.log("Your transaction signature", tx);

    // Assert the new owner
    const accountInfo = await program.account.ownerAccount.fetch(ownerAccount.publicKey);
    expect(accountInfo.owner.toString()).toBe(newOwner.toString());
  });
});
