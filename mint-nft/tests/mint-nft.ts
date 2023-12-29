import * as anchor from "@coral-xyz/anchor";
import { MintNft } from "../target/types/mint_nft";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("mint-nft", () => {

  const nftName = "minha NFT"
  const nftSymbol = "WONY"
  const nftURI= "https://raw.githubusercontent.com/arthur999999/anchor-solana-studies/main/mint-nft/assets/uri.json";

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as  anchor.Wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.MintNft as anchor.Program<MintNft>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );


  it("Mintando NFT", async () => {

    const MintKeyPair = anchor.web3.Keypair.generate();

    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: MintKeyPair.publicKey,
      owner: wallet.publicKey
    })

    console.log(`Novo Token: ${MintKeyPair.publicKey}`);

    const metadataAddress =  (await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("metada"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      MintKeyPair.publicKey.toBuffer()
    ], TOKEN_METADATA_PROGRAM_ID))[0];

    console.log("Metadata initialized");

    const masterEditionAddress =  ( await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("metada"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      MintKeyPair.publicKey.toBuffer(),
      Buffer.from("edition")
    ], TOKEN_METADATA_PROGRAM_ID))[0];

    console.log("Master edition metadata initialized");

    await program.methods.mintNft(
      nftName, nftSymbol, nftURI
    )
    .accounts({
      masterEdition: masterEditionAddress,
      metadata: metadataAddress,
      mint: MintKeyPair.publicKey,
      tokenAccount: tokenAddress,
      mintAuthority: wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([MintKeyPair])
    .rpc();

  })
  
});
