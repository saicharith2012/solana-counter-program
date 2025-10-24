import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { expect, test } from "bun:test";
import { COUNTER_SIZE, CounterAccount, schema } from "./types";
import * as borsh from "borsh";

const adminAccount = Keypair.generate();
const dataAccount = Keypair.generate();

const connection = new Connection("http://localhost:8899");
const PROGRAM_ID = new PublicKey(
  "BW1fFqWb6pE6G3xPhRj5UgNRHXmA3hM8D2peLfuHZNxh"
);

test("Account Initialized", async () => {
  // airdrop sol
  const airdropTxn = await connection.requestAirdrop(
    adminAccount.publicKey,
    1 * LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(airdropTxn);

  const data = await connection.getAccountInfo(adminAccount.publicKey);
  console.log(data?.lamports);

  // create new data account
  const lamports = await connection.getMinimumBalanceForRentExemption(
    COUNTER_SIZE
  );

  const createAccountTxn = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: adminAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      space: COUNTER_SIZE,
      lamports,
      programId: PROGRAM_ID,
    })
  );

  await sendAndConfirmTransaction(connection, createAccountTxn, [
    adminAccount,
    dataAccount,
  ]);

  console.log(dataAccount.publicKey.toBase58());

  // fetch data from the data account
  const counterAccount = await connection.getAccountInfo(dataAccount.publicKey);
  const counter = borsh.deserialize(
    schema,
    counterAccount?.data!
  ) as CounterAccount;

  console.log("Counter value is at: ", counter.count);
  expect(counter.count).toBe(0);
});
