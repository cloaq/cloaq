# Cloaq

Cloaq is the first blockchain network built on Substrate that leverages Fully Homomorphic Encryption (FHE) to revolutionize data privacy and security in decentralized environments. As the first of its kind, Cloaq allows users to perform computations on encrypted data without ever exposing the underlying information, ensuring unparalleled confidentiality. By integrating FHE directly into the blockchain infrastructure, Cloaq empowers developers and enterprises to create privacy-preserving applications that can securely handle sensitive data, from financial transactions to personal health information. This innovative approach not only enhances user privacy but also opens new possibilities for secure data sharing and collaboration in a decentralized world, setting a new standard for blockchain networks.

## Current Status of FHE in Polkadot

As Fully Homomorphic Encryption (FHE) gains increasing attention for its critical role in privacy-preserving computation, more blockchain ecosystems are beginning to adopt this groundbreaking technology. However, despite its growing importance, there remains a significant gap in the Polkadot ecosystem: there are no FHE-focused projects or pallets specifically designed for this environment. Furthermore, there is a lack of an FHE Rust library with no-std support, which is essential for enabling FHE capabilities on Substrate-based blockchains.


## Architecture
![Cloaq Architecture](https://raw.githubusercontent.com/h4n0/polkadot-hackathon-2024/47e96067a13aba304f1b4e5fe1f1f762ea01b195/singapore/23-Cloak/doc/cloak_arch.jpg))


## Build

Use the following command to build the node without launching it:

```sh
cargo build --package cloaq-node --release
```


## Demo and Testnet
1. Demo Video: https://youtu.be/q3UpOFLfBNQ
2. Cloaq Testnet: comming soon
