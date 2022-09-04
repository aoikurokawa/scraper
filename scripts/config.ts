import { LAMPORTS_PER_SOL, PublicKey, Keypair } from "@solana/web3.js";
import fs from "fs";
import * as anchor from "@project-serum/anchor";
import { Staking } from "../target/types/staking";

anchor.setProvider(anchor.AnchorProvider.env());
export const program = anchor.workspace.Staking as anchor.Program<Staking>;
export const connection = anchor.getProvider().connection;
export const userWallet = anchor.workspace.Staking.provider.wallet;

export const randomPayer = async (lamports = LAMPORTS_PER_SOL) => {
  const wallet = Keypair.generate();
  const signature = await connection.requestAirdrop(wallet.publicKey, lamports);
  await connection.confirmTransaction(signature, "finalized");
  return wallet;
};

export const findBeefMintAuthorityPDA = async (): Promise<
  [PublicKey, number]
> => {
  return await getProgramDerivedAddress(beefMintAddress);
};

export const findStakeMintAuthorityPDA = async (): Promise<
  [PublicKey, number]
> => {
  return await getProgramDerivedAddress(stakeMintAddress);
};

export const getProgramDerivedAddress = async (
  seed: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [seed.toBuffer()],
    program.programId
  );
};

// @ts-ignore
const beefData = JSON.parse(fs.readFileSync(".keys/beef_mint.json"));
export const beefMintKeypair = Keypair.fromSecretKey(new Uint8Array(beefData));
export const beefMintAddress = beefMintKeypair.publicKey;

// @ts-ignore
const stakeData = JSON.parse(fs.readFileSync(".keys/stake_mint.json"));
export const stakeMintKeypair = Keypair.fromSecretKey(
  new Uint8Array(stakeData)
);
export const stakeMintAddress = stakeMintKeypair.publicKey;
