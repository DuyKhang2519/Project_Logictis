/*# Project Title

Global Logistics Product Hub

# Project Description

Global Logistics Product Hub is a decentralized product and inventory management platform built on the Stellar Soroban blockchain. It enables multinational logistics companies to securely manage products, warehouses, inventory levels, and shipment records across multiple countries through transparent and immutable smart contracts.

By leveraging blockchain technology, every product movement, inventory update, and shipment status change is permanently recorded on-chain, ensuring trust, traceability, and operational efficiency throughout the global supply chain.

# Project Vision

Our vision is to create a transparent and decentralized logistics ecosystem where manufacturers, warehouses, transportation providers, and customers can access trusted product information without relying on centralized databases.

Global Logistics Product Hub helps organizations reduce fraud, improve inventory visibility, and streamline international logistics operations while maintaining a single source of truth across all participating parties.

# Key Features

### Product Management

* Create and manage products.
* Store product details including SKU, name, description, and weight.
* Update product information securely.

### Inventory Management

* Track inventory quantities.
* Manage stock across multiple warehouses.
* Record inventory updates on-chain.

### Shipment Tracking

* Create shipment records.
* Track shipment status from origin to destination.
* Maintain immutable shipment history.

### Multi-Country Operations

* Support warehouses and products across different countries.
* Enable global inventory visibility.

### Access Control

* Admin-controlled product creation.
* Authorized staff can update inventory.
* Public product verification.

### Transparency & Auditability

* Immutable blockchain records.
* Full audit trail for inventory and logistics operations.

# Usage Instructions

1. Deploy the smart contract.
2. Initialize the contract with an administrator.
3. Create products.
4. Register inventory quantities.
5. Update inventory as products move.
6. Query products and inventory status.
7. Track logistics operations through blockchain records.

# Future Scope

* Integration with IoT sensors.
* GPS shipment tracking.
* NFT-based product certificates.
* Automated customs documentation.
* AI-powered demand forecasting.
* Cross-chain logistics integration.
* Carbon footprint monitoring.

# Technology Stack

* Rust
* Soroban SDK
* Stellar Blockchain
* Smart Contracts
* Decentralized Storage Concepts

# Contribution

Contributions are welcome from blockchain developers, logistics professionals, and supply chain experts. Fork the repository and submit pull requests to help improve the platform.

# License

MIT License
*/
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String,
};

#[derive(Clone)]
#[contracttype]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub sku: String,
    pub quantity: u32,
    pub country: String,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    ProductCount,
    Product(u64),
}

#[contract]
pub struct GlobalLogisticsProductHub;

#[contractimpl]
impl GlobalLogisticsProductHub {

    // Initialize contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ProductCount, &0u64);
    }

    // Create product
    pub fn create_product(
        env: Env,
        admin: Address,
        name: String,
        sku: String,
        quantity: u32,
        country: String,
    ) -> u64 {

        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::ProductCount)
            .unwrap_or(0);

        count += 1;

        let product = Product {
            id: count,
            name,
            sku,
            quantity,
            country,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Product(count), &product);

        env.storage()
            .instance()
            .set(&DataKey::ProductCount, &count);

        count
    }

    // Update inventory
    pub fn update_quantity(
        env: Env,
        admin: Address,
        product_id: u64,
        new_quantity: u32,
    ) {

        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let key = DataKey::Product(product_id);

        let mut product: Product = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Product not found");

        product.quantity = new_quantity;

        env.storage()
            .persistent()
            .set(&key, &product);
    }

    // Get product details
    pub fn get_product(
        env: Env,
        product_id: u64,
    ) -> Product {

        env.storage()
            .persistent()
            .get(&DataKey::Product(product_id))
            .expect("Product not found")
    }

    // Total products
    pub fn total_products(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ProductCount)
            .unwrap_or(0)
    }
}