/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
  Account,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  SystemInstruction,
  SYSVAR_CLOCK_PUBKEY,
  Keypair,
  
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {
  getPayer,
  getRpcUrl,
  newAccountWithLamports,
  readAccountFromFile,
} from './utils';

/**
 * Connection to the network
 */
let connection: Connection;
let systemprogram : SystemProgram;
/**
 * Connection to the network
 */
let payerAccount: Account;
let programAccount: Account;
let kittyprogramId: PublicKey;

let TransferOne: Account;
let TransferTwo: Account;
/**
 * Hello world's program id
 */
let programId: PublicKey;
let crossprogramId: PublicKey;
let crossprogramAddress :[PublicKey, number];
/**
 * The public key of the account we are saying hello to
 */

/**
 * Path to program files
 */
//const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');
const PROGRAM_PATH = path.resolve(__dirname, '../../example-helloworld/src/transfer-lamports/target/deploy');
const TRANSFER_ONE = ('/root/ji/transfer_one.json');
const TRANSFER_TWO = ('/root/ji/transfer_two.json');
/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
//const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'helloworld.so');
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'spl_example_transfer_lamports.so');
const CROSS_PROGRAM_SO_PATH = ('/root/ji/example-helloworld/src/cross-program-invocation/target/deploy/spl_example_cross_program_invocation.so');
const KITTY_PROGRAM_SO_PATH = ('/root/ji/example-helloworld/src/kitty/target/deploy/solana_escrow.so');
/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/helloworld.so`
 */
const KITTY_KEYPAIR_PATH = ('/root/ji/example-helloworld/src/kitty/target/deploy/solana_escrow-keypair.json');
const PROGRAM_KEYPAIR_PATH = ('/root/ji/example-helloworld/src/transfer-lamports/target/deploy/spl_example_transfer_lamports-keypair.json');
const CROSS_PROGRAM_KEYPAIR_PATH = ('/root/ji/example-helloworld/src/cross-program-invocation/target/deploy/spl_example_cross_program_invocation-keypair.json');
/**
 * The state of a greeting account managed by the hello world program
 */
class GreetingAccount {
  counter = 0;
  constructor(fields: {counter: number} | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}

/**
 * Borsh schema definition for greeting accounts
 */
const GreetingSchema = new Map([
  [GreetingAccount, {kind: 'struct', fields: [['counter', 'u32']]}],
]);

/**
 * The expected size of each greeting account.
 */
const GREETING_SIZE = borsh.serialize(
  GreetingSchema,
  new GreetingAccount(),
).length;

/**
 * Establish a connection to the cluster
 */
export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establishPayer(): Promise<void> {
  let fees = 0;
  if (!payerAccount) {
    const {feeCalculator} = await connection.getRecentBlockhash();
    // Calculate the cost to fund the greeter account
    fees += await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

    // Calculate the cost of sending transactions
    fees += feeCalculator.lamportsPerSignature * 100; // wag

    try {
      // Get payer from cli config
      payerAccount = await getPayer();
    } catch (err) {
      // Fund a new payer via airdrop
      payerAccount = await newAccountWithLamports(connection, fees);
    }
  }

  const lamports = await connection.getBalance(payerAccount.publicKey);
  if (lamports < fees) {
    // This should only happen when using cli config keypair
    const sig = await connection.requestAirdrop(
      payerAccount.publicKey,
      fees - lamports,
    );
    await connection.confirmTransaction(sig);
  }

  console.log(
    'Using account',
    payerAccount.publicKey.toBase58(),
    'containing',
    lamports / LAMPORTS_PER_SOL,
    'SOL to pay for fees',
  );
}
/**
 * Check if the hello world BPF program has been deployed
 */

 export async function checkkittyProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const kittyprogramAccount = await readAccountFromFile(KITTY_KEYPAIR_PATH);
    kittyprogramId = kittyprogramAccount.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
    );
  }
  const kittyprogramInfo = await connection.getAccountInfo(kittyprogramId);
  if (kittyprogramInfo === null) {
    if (fs.existsSync(KITTY_PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy /root/ji/example-helloworld/src/cross-program-invocation/target/deploy/spl_example_cross_program_invocation.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!kittyprogramInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  else{
    console.log('programInfo',kittyprogramInfo.owner.toBase58());
  }
  console.log("111111");

  

  
}


export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programAccount = await readAccountFromFile(PROGRAM_KEYPAIR_PATH);
    const crossprogramAccount = await readAccountFromFile(CROSS_PROGRAM_KEYPAIR_PATH);
    const kittyprogramAccount = await readAccountFromFile(KITTY_KEYPAIR_PATH);
    programId = programAccount.publicKey;
    crossprogramId = crossprogramAccount.publicKey;
    kittyprogramId = kittyprogramAccount.publicKey;
    console.log('crossprogramId',crossprogramId.toBase58());
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
    );
  }

  const kittyprogramInfo = await connection.getAccountInfo(kittyprogramId);
  if (kittyprogramInfo === null) {
    if (fs.existsSync(KITTY_PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy /root/ji/example-helloworld/src/cross-program-invocation/target/deploy/spl_example_cross_program_invocation.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!kittyprogramInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  else{
    console.log('programInfo',kittyprogramInfo.owner.toBase58());
  }

  const crossprogramInfo = await connection.getAccountInfo(crossprogramId);
  if (crossprogramInfo === null) {
    if (fs.existsSync(CROSS_PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy /root/ji/example-helloworld/src/cross-program-invocation/target/deploy/spl_example_cross_program_invocation.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!crossprogramInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  else{
    console.log('programInfo',crossprogramInfo.owner.toBase58());
  }
  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  console.log('programInfo' ,programInfo);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/helloworld.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  else{
    console.log('programInfo',programInfo.owner.toBase58());
    console.log('programId',programId.toBase58());
  }
  console.log(`Using program ${programId.toBase58()}`);

  TransferOne = await readAccountFromFile(TRANSFER_ONE);
  console.log('TransferOne',TransferOne);
  
  const transferoneaccount = await connection.getAccountInfo(TransferOne.publicKey);
  console.log('transferoneaccount',transferoneaccount);
  const transaction =  new Transaction().add(
  SystemProgram.assign({accountPubkey: TransferOne.publicKey, programId}) ,);
  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount,TransferOne],
    
  );
  console.log('transferoneaccount',transferoneaccount);
  if(transferoneaccount != null)
  console.log('TransferOne owner',transferoneaccount.owner.toBase58());
  TransferTwo = await readAccountFromFile(TRANSFER_TWO);
  console.log('TransferOne',TransferTwo);
  const transfertwoaccount = await connection.getAccountInfo(TransferTwo.publicKey);
  if(transfertwoaccount != null)
  console.log('transfertwoaccount',transfertwoaccount.owner.toBase58());

  const payeraaccount = await connection.getAccountInfo(payerAccount.publicKey);
  if(payeraaccount!=null)
  console.log('payerAccount ',payeraaccount.owner.toBase58());
  
}

export async function createkitty(): Promise<void> {
  console.log("111111");
  let new_key = Keypair.generate();
  // let abc = Keypair.fromSeed(TransferTwo.publicKey.toBytes());
  console.log(new_key);
  console.log(kittyprogramId);
  console.log('111111111');
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: payerAccount.publicKey, isSigner: false, isWritable: true},
      {pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true},
      {pubkey: SystemProgram.programId,isSigner: false, isWritable: true},
      {pubkey: new_key.publicKey ,isSigner: true, isWritable: true}
    ],
    programId: kittyprogramId,
    // data: Buffer.from([nonce]), 
    data: Buffer.from(Uint8Array.of(3))
  });

  console.log('success');
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payerAccount,new_key],
  );
  console.log('success2');

}


export async function brithkitty(): Promise<void> {
  console.log("111111");
  TransferOne = await readAccountFromFile(TRANSFER_ONE);
  TransferTwo = await readAccountFromFile(TRANSFER_TWO);
  let abc = Keypair.fromSeed(TransferTwo.publicKey.toBytes());
  console.log(abc);
  console.log(kittyprogramId);
  console.log('111111111');
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: TransferOne.publicKey, isSigner: false, isWritable: true},
      {pubkey: TransferTwo.publicKey, isSigner: false, isWritable: true},
      {pubkey: payerAccount.publicKey, isSigner: false, isWritable: true},
      {pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true},
      {pubkey: SystemProgram.programId,isSigner: false, isWritable: true},
      {pubkey: abc.publicKey ,isSigner: true, isWritable: true}
    ],
    programId: kittyprogramId,
    // data: Buffer.from([nonce]), 
    data: Buffer.from(Uint8Array.of(2))
  });

  console.log('success');
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payerAccount,abc],
  );
  console.log('success2');

}


export async function transfer(): Promise<void> {
  crossprogramAddress = await PublicKey.findProgramAddress(
    [Buffer.from('You pass butter', 'utf8')],
    crossprogramId);
  console.log('crossprogramAddress',crossprogramAddress[1]);
  const crossprogramAccount = await connection.getAccountInfo(crossprogramAddress[0]);
  const nonce = crossprogramAddress[1];
  if(crossprogramAccount!=null)
  console.log('crossprogramAccount ',crossprogramAccount);
  let abc = Keypair.fromSeed(TransferTwo.publicKey.toBytes());
  const instruction = new TransactionInstruction({
    keys: [{pubkey: programId, isSigner: false, isWritable: true},
      {pubkey: crossprogramAddress[0], isSigner: false, isWritable: true},
      {pubkey: TransferOne.publicKey, isSigner: false, isWritable: true},
      {pubkey: TransferTwo.publicKey, isSigner: false, isWritable: true},
      {pubkey: payerAccount.publicKey, isSigner: false, isWritable: true},
      {pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true},
      {pubkey: SystemProgram.programId,isSigner: false, isWritable: true},
      {pubkey: abc.publicKey ,isSigner: true, isWritable: true}
    ],
    programId: crossprogramId,
    // data: Buffer.from([nonce]), 
    data: Buffer.from(Uint8Array.of(2))
  });

  console.log('success');
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payerAccount,abc],
  );
  console.log('success2');
  // const instruction2 = new TransactionInstruction({
  //     keys: [{pubkey: TransferOne.publicKey, isSigner: false, isWritable: true},
  //       {pubkey: TransferTwo.publicKey, isSigner: false, isWritable: true}],
  //     programId,
  //     data: Buffer.alloc(0), 
  //   });
  //   console.log('success');
  //   await sendAndConfirmTransaction(
  //     connection,
  //     new Transaction().add(instruction2),
  //     [payerAccount],
  //   );
  //   console.log('success2');
}

/**
 * Say hello
 */


/**
 * Report the number of times the greeted account has been said hello to
 */
export async function reportGreetings(): Promise<void> {
  const oneaccountInfo = await connection.getAccountInfo(TransferOne.publicKey);
  const twoaccountInfo = await connection.getAccountInfo(TransferTwo.publicKey);
  if (oneaccountInfo === null && twoaccountInfo === null) {
    throw 'Error: cannot find the greeted account';
  }
  else if( (oneaccountInfo != null && twoaccountInfo != null) ){
    console.log(
      TransferOne.publicKey.toBase58(),
    'balance is',
    oneaccountInfo.lamports,
    );
    console.log(
      TransferTwo.publicKey.toBase58(),
        'balance is',
        twoaccountInfo.lamports,
      );
  }
  else{}
  
}
