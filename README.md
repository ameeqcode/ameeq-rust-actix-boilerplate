# Rust Actix Boilerplate

Welcome to the Rust Actix Boilerplate! This repository provides a starting point for building a web application using Rust and the Actix framework. It includes configurations for Diesel as the ORM, handling database migrations, and instructions to get the project up and running.

## Getting Started

These instructions will help you set up the project on your local machine for development and testing purposes.

## Prerequisites

Ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Diesel CLI](https://diesel.rs/guides/getting-started/)
- [PostgreSQL](https://www.postgresql.org/download/) (or any other supported database)

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/rust-actix-boilerplate.git
    cd rust-actix-boilerplate
    ```

2. Set up the environment variables:

    Create a `.env` file in the project root and add the following:

    ```env
    DATABASE_URL=postgres://username:password@localhost/database_name
    ```

3. Install Diesel CLI:

    ```sh
    cargo install diesel_cli --no-default-features --features postgres
    ```

4. Set up the database:

    ```sh
    diesel setup
    ```

## Running the Project

To run the project in development mode, use:

    ```sh
    cargo run
    ```
# Database Migrations

### Creating a Migration

To create a new migration, use the following command:

    ```sh
    diesel migration generate create_users_table
    ```
### Running a Migration

To run all pending migrations, use:

    ```sh
    diesel migration run
    ```
### Reverting Migrations

To revert the last migration, use:

    ```sh
    diesel migration revert
    ```
