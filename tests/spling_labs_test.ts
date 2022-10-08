import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplingLabsTest } from "../target/types/spling_labs_test";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "bn.js";

describe("spling_labs_test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplingLabsTest as Program<SplingLabsTest>;

  const fromWallet = anchor.web3.Keypair.generate()
  const mint = anchor.web3.Keypair.generate()

  it("setup", async () => {
    const payer = await provider.connection.requestAirdrop(fromWallet.publicKey, LAMPORTS_PER_SOL * 5);
    await checkTransactionIsOK(provider, payer);
  });

  it("Mint initialized!", async () => {

    // Initialize mint
    const tx = await program.methods.initializeMint()
    .accounts({
      mint: mint.publicKey,
      payer: fromWallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY
     })
     .signers([fromWallet, mint])
     .rpc();

     checkTransactionIsOK(provider, tx);
  });

  it("Actually mint the tokens", async () => {

    let associatedTokenAccount = await getAssociatedTokenAddress(
      mint.publicKey,
      fromWallet.publicKey,
    );

    await program.methods.mintToken(new BN(100))
    .accounts({
      mint: mint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenAccount: associatedTokenAccount,
      authority: fromWallet.publicKey,
    })
    .signers([fromWallet])
    .rpc();

  });
  

  const checkTransactionIsOK = async function(provider: anchor.Provider, transaction: TransactionSignature): Promise<RpcResponseAndContext<SignatureResult>> { 
    const {blockhash, lastValidBlockHeight} = await provider.connection.getLatestBlockhash();
    return await provider.connection.confirmTransaction({
      blockhash,
      lastValidBlockHeight,
      signature: transaction
    });
  }

});
