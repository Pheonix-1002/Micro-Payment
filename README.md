# Micro-Payment

**Charity Donation Tracker**
This is a smart contract written in Rust using #![no_std] and designed to run on the Soroban blockchain platform. The contract enables users to make token transfers while supporting their favorite charities.

**Contract Overview**
The Charity Donation Tracker smart contract is a decentralized application that facilitates charity donations through token transfers. The contract is managed by an admin who has the authority to set the admin address, add or remove charity accounts, and view donation amounts.

**Key Features :**
*--> Admin Control:* The admin address can be set by the contract creator, giving them control over charity account management.

*--> Charity Registration:* Only the admin can add or remove charity accounts, each associated with a unique identifier (id) and address.

*--> Token Transfer with Donation:* Users can initiate token transfers, specifying the recipient's address and the charity account ID. The contract automatically donates 2% of the transferred token amount to the designated charity account.

*--> Donation Analytics:* The contract tracks donation amounts for each user and charity account, providing a transparent and fair donation process.

**Contract Functions**
*--> set_admin:* Sets the admin address.

*--> register_charity:* Registers a new charity account with a specified ID and address.

*--> remove_charity:* Removes a charity account based on its ID.

*--> donate:* Initiates a token transfer from one address to another, automatically donating 2% of the transferred amount to the specified charity account.

*--> get_user_donations:* Retrieves the total donation amount made by a specific user for a particular token.

*--> get_charity_donations:* Retrieves the total donation amount received by a charity account for a particular token.

**Deployment and Usage**
To deploy this smart contract on the Soroban blockchain platform, compile the code and deploy the resulting bytecode. Once deployed, interact with the contract using Soroban-compatible tools or libraries. Smart Contract Address: "TO_BE_SET"

**Project Structure**
This repository follows the recommended structure for a Soroban project:
.
├── contracts
│   └── CharityDonationTracker
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md

The 'contracts' directory contains the smart contract source code, while the 'Cargo.toml' file in the root directory is used for dependency management. The 'README.md' file provides an overview and usage instructions for the smart contract.



