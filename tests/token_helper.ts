import {Signer, PublicKey, Keypair} from "@solana/web3.js";
import {
    Account, 
    createMint, 
    getMint, 
    getOrCreateAssociatedTokenAccount
} from "@solana/spl-token";
import {connection, randomPayer} from "../scripts/config";

export class TokenHelper {
    mint: PublicKey;
    
    constructor(mint: PublicKey) {
        this.mint = mint;
    }
    
    getMint = async (): Promise<PublicKey> => {
        return (await getMint(connection, this.mint)).address;
    } 
    
    balance = async (tokenBag: PublicKey) => {
        const balance = (await connection.getTokenAccountBalance(tokenBag)).value.amount;
        return parseInt(balance);
    }
    
    getOrCreateTokenBag = async (owner: PublicKey, isPDA: boolean = false): Promise<Account> => {
        // Get or create the account for token of type mint for owner
        return await getOrCreateAssociatedTokenAccount(
            connection,
            await randomPayer(),
            this.mint,
            owner,
            isPDA
        );
    }
}
