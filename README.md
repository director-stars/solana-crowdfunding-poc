### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```
=========================================
### Scope
1. Wallet Integration (Sollet Web , Sollet Extension and Phantom Wallet) with Website. 
Blank webpage with "Connect Wallet" and "Mint" buttons.

2. On clicking "Connect Wallet" button, wallet should be connected and corresponding wallet address shown on webpage.

3. Create a smart contract which when invoked by the user from the web page using the "Send" button, 
should transfer 2 SOLs from user wallet to the contract.

The smart contract should not accept more than 20 SOLs which mean only 10 successful transactions(2 SOLs each) 
can be made and any transaction after the contract collected 20 SOLs should fail and 
appropriate error message should be shown on web page.

4. Write a CLI command to dump all the successful transactions happened on the smart contract,
 with ability to see the user wallet address.

5. Deployment steps & Script

6. Fund transafer shall be allowed from Smart contrat to external Wallet :)