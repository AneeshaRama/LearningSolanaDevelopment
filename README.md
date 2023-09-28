# LearningSolanaDevelopment

Welcome to the **LearningSolanaDevelopment** repository 🙂! This is where I document my journey into the world of Solana development. If you're a novice solana developer like me, I invite you to explore this repository, learn with me, and contribute to our collective understanding of Solana.

### About Solana

Solana is a high-performance, next-generation blockchain platform designed to support decentralized applications (DApps) and cryptocurrencies. Known for its exceptional speed and scalability, Solana sets itself apart by offering lightning-fast transaction processing times and low fees, making it an ideal choice for a wide range of applications, from decentralized finance (DeFi) to non-fungible tokens (NFTs).

Key Features:

  - **High Throughput**: Solana can handle thousands of transactions per second, making it one of the fastest blockchain networks in the world.

  - **Low Latency**: Transactions on Solana are confirmed in milliseconds, providing near-instantaneous results.

  - **Scalability**: Solana's unique architecture enables horizontal scaling, ensuring the network can grow as demand increases.

  - **Secure**: Solana utilizes cutting-edge cryptography and security measures to protect user assets and data.

  - **Decentralized**: Like other blockchain networks, Solana is decentralized, meaning it operates without a central authority.

Whether you're interested in developing smart contracts, building decentralized applications, or exploring the potential of blockchain technology, Solana offers a robust and developer-friendly ecosystem to help you bring your ideas to life.



### Solana CLI set up for development
I am using Linux operationg system (Ubuntu 22.04) for the solana development. I have Rust programming language installed in my system. Here are the steps you can follow to set up the Solana CLI:



1. **Run the following command to install the solana CLI**
```
sh -c "$(curl -sSfL https://release.solana.com/v1.16.14/install)"
```

2. **Update the path environment**
```
export PATH="/home/poppy/.local/share/solana/install/active_release/bin:$PATH"
```

3. **Confirm the installation by running:**
```
solana --version
```

4. **Once the CLI is installed, we must create an account. We do this by generating a new *public/private key pair*.**

```shell
solana-keygen new
```

5. **Once we do that, we have to configure our Solana CLI client to use our new account**.

```shell
solana config set --keypair /root/.config/solana/id.json
```

6. **And now we just need to bind our client to a Solana network. Here I have connected it to Solana Devnet.**

```shell
solana config set --url https://api.devnet.solana.com
```


To get started with Solana development, explore the resources and documentation provided in this repository and join me on this journey of discovery and innovation. 🙂 🚀
