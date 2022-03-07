import assert from "assert";
import anchor from "@project-serum/anchor";

const { SystemProgram } = anchor.web3;

describe("mycalculatordapp", () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  // @ts-ignore
  const program = anchor.workspace.Mycalculatordapp;

  it("Creates a calculator", async () => {});

  it("Adds two numbers", async () => {});

  it("Multiplies two numbers", async () => {});

  it("Subtracts two numbers", async () => {});

  it("Divides two numbers", async () => {});
});
