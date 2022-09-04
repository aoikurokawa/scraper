import { mintTo } from "@solana/spl-token";
import { beefMintKeypair, connection, randomPayer } from "./config";
import { TokenHelper } from "../tests/token_helper";
import { User } from "../tests/user";

export const airdropBeefToken = async () => {
  const user = new User();
  await user.getOrCreateBeefTokenBag();

  await mintTo(
    connection,
    await randomPayer(),
    beefMintKeypair.publicKey,
    user.beefTokenBag,
    beefMintKeypair,
    1_000_000_000,
    []
  );

  const balance = await new TokenHelper(beefMintKeypair.publicKey).balance(
    user.beefTokenBag
  );
  console.log(
    `Token Account ${user.beefTokenBag.toString()} balance: ${balance}`
  );
};
