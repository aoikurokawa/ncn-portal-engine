import assert from "assert";
import anchor, { web3 } from "@project-serum/anchor";

const { SystemProgram } = anchor.web3;

describe("mycalculatordapp", () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  // @ts-ignore
  const program = anchor.workspace.Mycalculatordapp;
  
  let _calculator: web3.Keypair;

  it("Creates a calculator", async () => {
    await program.rpc.create("Welcome to Solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator],
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "Welcome to Solana");
    _calculator = calculator;
  });

  it("Adds two numbers", async () => {
    const calculator = _calculator;

    await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
      accouts: {
        calculator: calculator.publicKey
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(5)));
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it("Multiplies two numbers", async () => {});

  it("Subtracts two numbers", async () => {});

  it("Divides two numbers", async () => {});
});
