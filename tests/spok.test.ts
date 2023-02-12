import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';
import { Account, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { Spok } from '../target/types/spok';

describe('spok', () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Spok as Program<Spok>;

    const conn = program.provider.connection;

    const mint = anchor.web3.Keypair.generate();
    const [spok] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('spok')], program.programId);

    const userKp = anchor.web3.Keypair.generate();
    let tokenAccount: Account;

    beforeAll(async () => {
        const sig = await conn.requestAirdrop(userKp.publicKey, 1e12);
        await conn.confirmTransaction(sig);
    });

    it('initialize', async () => {
        await program.methods
            .initialize()
            .accounts({
                mint: mint.publicKey,
                payer: userKp.publicKey,
                spok,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([userKp, mint])
            .rpc();

        tokenAccount = await getOrCreateAssociatedTokenAccount(conn, userKp, mint.publicKey, userKp.publicKey);
    });

    for (let i = 0; i < 50; i++) {
        it(`mines ${i}`, async () => {
            await program.methods
                .mine(Buffer.from([]))
                .accounts({
                    mint: mint.publicKey,
                    payerTa: tokenAccount.address,
                    payer: userKp.publicKey,
                    spok,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([userKp])
                .rpc();
        });
    }
});
