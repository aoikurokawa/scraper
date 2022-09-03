import {Keypair, PublicKey} from "@solana/web3.js";
import {createMint} from "@solana/spl-token";
import {beefMintKeypair, stakeMintKeypair, connection, randomPayer, findBeefMintAuthorityPDA} from "./config";

const createMintAcct = async (keypairToAssign: Keypair, authorityToAssign: PublicKey): Promise<PublicKey> => {
  return await createMint(
    connection,
    await randomPayer(),
    authorityToAssign,
    null,
    8,
    keypairToAssign,
  );
}

export const createMints = async() => {
  const beefMintAddress = await createMintAcct(beefMintKeypair, beefMintKeypair.publicKey);
  
  const [stakePDA, _] = await findBeefMintAuthorityPDA();
  
  const stakeMintAddress = await createMintAcct(stakeMintKeypair, stakePDA);
  
  console.log(`beef Mint Address: ${beefMintAddress}`);
  console.log(`stake Mint Address: ${stakeMintAddress}`);
}
