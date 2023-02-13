import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';
import { Account, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { Spok } from '../target/types/spok';
import createKeccakHash from 'keccak';

describe('spok', () => {
    beforeEach(() => {
        global.console = require('console');
    });

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

    for (let i = 0; i < 10; i++) {
        it(`mines ${i}`, async () => {
            const s = await program.account.spok.fetch(spok);
            const target = Buffer.from(s.target);
            const targetValue = parseInt(target.toString('hex'), 16);
            const mints = s.mints.toBuffer('be', 8);

            console.log('target is', target.toString('hex'));
            console.log('last minted slot is', s.lastTargetSlot.toNumber());

            console.log('sleeping');
            await new Promise((r) => setTimeout(r, 682));

            for (let j = 0; j < 10000; j++) {
                const nonce = Buffer.from(j.toString(2), 'binary');
                const inputHash = createKeccakHash('keccak256')
                    .update(Buffer.concat([target, nonce, tokenAccount.address.toBuffer(), mints]))
                    .digest('hex');

                const inputValue = parseInt(inputHash, 16);

                if (inputValue < targetValue) {
                    console.log('found input with nonce', j);
                    console.log('input hash', inputHash);

                    await program.methods
                        .mine(nonce)
                        .accounts({
                            mint: mint.publicKey,
                            payerTa: tokenAccount.address,
                            payer: userKp.publicKey,
                            spok,
                            tokenProgram: TOKEN_PROGRAM_ID,
                        })
                        .signers([userKp])
                        .rpc();

                    break;
                }

                if (j === 9999) {
                    console.log('Need more bytes!');
                }
            }
        });
    }
});

// console.log('expecting hash', targetHash);

// const b1 = createKeccakHash('keccak256').update('123').digest();
// const b2 = Buffer.from('ffffffffffffffffffffffffffffffff', 'hex');
// const b3 = Buffer.concat([b1, b2]);

// const h1 = createKeccakHash('keccak256').update(b1).update(b2).digest('hex');
// const h2 = createKeccakHash('keccak256').update(b3).digest('hex');
// const h3 = createKeccakHash('keccak256').update('123').update(b2).digest('hex');
