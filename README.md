# Turbin

## Getting started

### Generate new keypair

```bash
cargo t test_keygen -- --nocapture
```

#### Create a new json file to store private key

```bash
touch dev-wallet.json
```

### Airdrop devnet SOL

```bash
cargo t test_airdrop -- --nocapture
```

### Interact with program

Copy IDL

![image](https://github.com/user-attachments/assets/9c9b4962-ec82-4e9b-a2b3-5dade6d16a39)


### Create a new json file to store private key

```bash
touch dev1-wallet.json
```

```bash
cargo t test_wba_prereq_program -- --nocapture
```

## Useful crate
- https://github.com/deanmlittle/solana-idlgen
