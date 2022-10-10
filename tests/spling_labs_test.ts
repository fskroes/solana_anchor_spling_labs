import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplingLabsTest } from "../target/types/spling_labs_test";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
import { BN } from "bn.js";

describe("spling_labs_test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplingLabsTest as Program<SplingLabsTest>;

  const fromWalletA = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();
  const tokenA = anchor.web3.Keypair.generate();
  const treasury = anchor.web3.Keypair.generate();

  it("setup", async () => {
    const payer = await provider.connection.requestAirdrop(fromWalletA.publicKey, LAMPORTS_PER_SOL * 5);
    await checkTransactionIsOK(provider, payer);
  });

  it("Mint initialized!", async () => {

    // Initialize mint
    const tx = await program.methods.initializeMint()
    .accounts({
      mint: mint.publicKey,
      payer: fromWalletA.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY
     })
     .signers([fromWalletA, mint])
     .rpc();

     checkTransactionIsOK(provider, tx);
  });

  it("Actually mint the tokens", async () => {

    let [statePubKey, _stateBump] = await PublicKey.findProgramAddress(
      [Buffer.from("state"), fromWalletA.publicKey.toBuffer()], program.programId,
    );
    console.log('State pda', statePubKey);

    await program.methods.mintToken(new BN(1000))
    .accounts({
       mint: mint.publicKey,
       tokenProgram: TOKEN_PROGRAM_ID,
       tokenA: tokenA.publicKey,
       authority: fromWalletA.publicKey,
       treasury: treasury.publicKey,
       state: statePubKey,
       systemProgram: anchor.web3.SystemProgram.programId,
       rent: anchor.web3.SYSVAR_RENT_PUBKEY
    })
    .signers([fromWalletA, tokenA, treasury])
    .rpc();

    let tokenAccountBalance = await getAccountTokenBalance(provider, tokenA.publicKey);
    console.log("Token balance:", tokenAccountBalance);
    console.log("Treasury balance", await provider.connection.getBalance(treasury.publicKey));

    let fetchedStateAccount = await program.account.accountA.fetch(statePubKey);
    console.log("State Counter is:", fetchedStateAccount.counter);
  });
  


  // ---- Helper functions ---- 
  const checkTransactionIsOK = async function(provider: anchor.Provider, transaction: TransactionSignature): Promise<RpcResponseAndContext<SignatureResult>> { 
    const {blockhash, lastValidBlockHeight} = await provider.connection.getLatestBlockhash();
    return await provider.connection.confirmTransaction({
      blockhash,
      lastValidBlockHeight,
      signature: transaction
    });
  }

  const getAccountTokenBalance = async function(provider: anchor.Provider, account_pub_key: PublicKey): Promise<number> {
    return parseInt(
      (await provider.connection.getTokenAccountBalance(account_pub_key)).value.amount
    );
  }

});
