# AI Talker API

A RESTful API server for managing PJSIP (Asterisk) realtime accounts, built with Rust and Axum framework.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [Clone the Repository](#clone-the-repository)
  - [Environment Setup](#environment-setup)
  - [Database Setup](#database-setup)
  - [Build and Run](#build-and-run)
- [API Endpoints](#api-endpoints)
- [Testing](#testing)
  - [Unit Tests](#unit-tests)
  - [Integration Tests](#integration-tests)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Overview

This API provides RESTful endpoints to manage PJSIP realtime accounts for Asterisk PBX systems. It supports creating, retrieving, and deleting SIP accounts with various transport protocols (UDP, TCP, TLS, WebSocket).

### Key Features

- Create SIP accounts with auto-generated or custom IDs
- Support for multiple transport protocols.  
  current UDP only. TCP, TLS, WS, WSS will be support.
- PostgreSQL-based persistence
- Comprehensive test coverage
- Docker/Podman support for PostgreSQL

## Architecture

```
ai-talker-api/
├── src/
│   ├── main.rs              # Entry point
│   ├── application/         # Business logic layer
│   │   └── repository/      # Repository interfaces
│   ├── infrastructure/      # Data access layer
│   │   ├── models/          # Database models
│   │   └── repository/      # Repository implementations
│   ├── restapi/             # REST API layer
│   │   ├── handlers/        # Request handlers
│   │   └── routes/          # Route definitions
│   └── tests/               # Integration tests
├── setup/              # Database initial setup script(s)
├── migrations/              # Database migrations
├── simple-test/             # Manual API test scripts
└── container_volumes/       # Docker/Podman volumes (auto-generated)
```

## Prerequisites

### Required Software
- **git**: 2.25 or higher
- **Rust**: 1.90 or higher
  - [Installation guide](https://www.rust-lang.org/tools/install)
- **Cargo**: Included with Rust installation
- **PostgreSQL**: 13 or higher (via Docker/Podman)
- **Podman** or **Docker**: For running PostgreSQL container
- **HTTPie**: For API testing (optional but recommended)
  - [Installation guide](https://httpie.io/docs/cli/installation)
- **uv**: An extremely fast Python package and project manager, written in Rust.
  - [Installation guide](https://docs.astral.sh/uv/getting-started/installation/)
- **python3**: 3.10 or higher
### Visual Studio Code Extensions (Recommended)

For development in VS Code, see [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

Required extensions for debugging:
- `rust-analyzer` - Rust language support
- `CodeLLDB` (Linux/macOS) or `Microsoft C++` (Windows) - Debugging support

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/hfujishige/ai-talker-api.git
cd ai-talker-api
```

### Environment Setup

1. **Copy the environment template** (if `.env` doesn't exist):

```bash
cp .env.example .env  # Create from example if needed
```

2. **Configure `.env` file**:

The `.env` file contains all necessary configuration. Key variables:

```bash
# Logging level
RUST_LOG=debug

# HTTP Server Configuration
LISTEN_IPV4=0.0.0.0
LISTEN_PORT_V4=3000

# PostgreSQL Database Configuration
PJSIP_DB_SCHEME=postgres
PJSIP_DB_HOST=127.0.0.1
PJSIP_DB_PORT=5432
PJSIP_DB_USER=api_user_rw
PJSIP_DB_PWD=q4p05yOt9V9g
PJSIP_DB_CATALOG=asterisk
PJSIP_DB_SSL_MODE=prefer

# Connection Pool Settings
PJSIP_DB_POOL_SIZE=3
PJSIP_DB_MAX_LIFETIME=1800
PJSIP_DB_MAX_IDLE=600
PJSIP_DB_TIMEOUT=10
```

**Note**: For production, use strong passwords and enable SSL connections.

### Database Setup
1. **Install SQLx CLI** (one-time setup):

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

2. **Create data volume directory** (one-time setup):

```bash
mkdir -p container_volumes/postgres_data
```

3. **Start PostgreSQL container**:

If you use PostgreSQL container. follow this procedure.
```bash
./pgsql_container.sh start
```

Check container status:

```bash
./pgsql_container.sh status
```

Stop container:

```bash
./pgsql_container.sh stop
```

4. **Create Database user**(one-time setup):

Note: this script need `postgres` user password.
run `run_setup.sh` script.  
```bash
./setup/run_setup.sh
```

5. **Run database migrations**:

```bash
# Set DATABASE_URL (or use the one in .env)
DATABASE_USER=api_user_rw
DATABASE_PASSWD=q4p05yOt9V9g
DATABASE_CATALOG=asterisk
DATABASE_HOST=127.0.0.1
DATABASE_PORT=5432
export DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASSWD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_CATALOG}"

# Check DATABASE_URL
echo $DATABASE_URL

# Create database (if database is not exists)
sqlx database create

# Run migrations
sqlx migrate run
```

Verify migrations:

```bash
# List applied migrations
sqlx migrate info
```

6. **Prepare SQLx for offline compilation** (optional):

```bash
cargo sqlx prepare
```

This generates `.sqlx/` directory for compile-time query verification without a live database.

7. **Sparse-checkout Asterisk PJSIP Realtime database**

```bash
git clone --no-checkout git@github.com:asterisk/asterisk.git
cd asterisk
git sparse-checkout init --cone
git sparse-checkout set contrib/ast-db-manage/
git checkout master
cd contrib/ast-db-manage/
cp config.ini.sample config.ini
vim config.ini
```

`config.ini`
Change `sqlalchemy.url`.Disable mysql url. Enable mysql url
The values for user, pass, and database are specified in `pgsql_container.sh`.
```conf
sqlalchemy.url = postgresql://api_user_rw:HE4ycm8uCER3@localhost/asterisk
```

create pjsip realtime tables
```sh
# Install python3 if you did not install python3.x
# e.g, install 3.13.x
uv python list
uv python install 3.13

# migrate pjsip realtime tables.
uv init
uv venv
source .venv/bin/activate
uv pip install alembic psycopg2-binary sqlalchemy
alembic -c config.ini upgrade head
```

### Build and Run

1. **Build the project**:

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

Or use VS Code task: `Ctrl+Shift+B` → `rust: cargo build`

2. **Run the server**:

```bash
# Development mode
cargo run

# Or run the compiled binary
./target/debug/ai-talker-api
```

The server will start on `http://0.0.0.0:3000` (configurable via `.env`).

3. **Verify the server is running**:

```bash
# Health check
curl http://127.0.0.1:3000/

# Or using HTTPie
http GET http://127.0.0.1:3000/
```

## API Endpoints

Base URL: `http://127.0.0.1:3000/api/v1/pjsip_realtime`

### Get All Accounts

```bash
GET /accounts
```

Example:
```bash
http GET http://127.0.0.1:3000/api/v1/pjsip_realtime/accounts
```

### Create Account (Auto-generated ID)

```bash
POST /accounts
Content-Type: application/json
```

Request body:
```json
{
  "username": "john_doe",
  "password": "123456",
  "transport": "udp",
  "context": "default",
  "from_domain": "example.com",
  "from_user": "1001"
}
```

Example:
```bash
http POST http://127.0.0.1:3000/api/v1/pjsip_realtime/accounts \
  username="john_doe" \
  password="123456" \
  transport="udp" \
  context="default" \
  from_domain="example.com" \
  from_user="1001"
```

### Create Account (Custom ID)

```bash
POST /accounts_with_id
Content-Type: application/json
```

Request body:
```json
{
  "id": "1001",
  "username": "jane_doe",
  "password": "654321",
  "transport": "udp",
  "context": "default",
  "from_domain": "example.com",
  "from_user": "1002"
}
```

Example:
```bash
http POST http://127.0.0.1:3000/api/v1/pjsip_realtime/accounts_with_id \
  id="1001" \
  username="jane_doe" \
  password="654321" \
  transport="udp" \
  context="default" \
  from_domain="example.com" \
  from_user="1002"
```

### Delete Account

```bash
DELETE /accounts/{account_id}
```

Example:
```bash
http DELETE http://127.0.0.1:3000/api/v1/pjsip_realtime/accounts/1001
```

### Supported Transport Types

- `udp` - UDP transport (fully implemented)
- `tcp` - TCP transport (not yet implemented)
- `tls` - TLS transport (not yet implemented)
- `ws` - WebSocket transport (not yet implemented)
- `wss` - Secure WebSocket transport (not yet implemented)

## Testing

### Unit Tests

1. **Install cargo-llvm-cov** (one-time setup):

```bash
cargo install cargo-llvm-cov
```

2. **Ensure PostgreSQL is running**:

```bash
./pgsql_container.sh start
```

3. **Run all tests with coverage**:

```bash
# Basic coverage report
cargo llvm-cov --workspace --html

# With backtrace for debugging
RUST_BACKTRACE=1 cargo llvm-cov --workspace --html
```

Coverage report will be generated in `target/llvm-cov/html/index.html`.

4. **Run specific tests**:

```bash
# List all available tests
cargo test -- --list

# Run a specific test
cargo llvm-cov -- --test-threads=1 --include-ignored --exact \
  tests::restapi::api::v1::pjsip_realtime::create_account::test_create_pjsip_realtime_account

# Run tests with pattern matching
cargo test create_account
```

5. **Run tests without coverage**:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test module
cargo test tests::restapi
```

### Integration Tests

Manual integration tests are provided in the `simple-test/` directory:

1. **Create account with auto-generated ID**:

```bash
./simple-test/api_v1_pjsip_realtime_create.sh
```

2. **Create account with custom ID**:

```bash
./simple-test/api_v1_pjsip_realtime_create_with_id.sh
```

3. **Delete account**:

```bash
./simple-test/api_v1_pjsip_realtime_delete.sh
```

These scripts use HTTPie and can be customized by editing the test data at the top of each file.

## Development

### Project Structure

- **`src/main.rs`**: Application entry point, server initialization
- **`src/application/`**: Business logic and use cases
- **`src/infrastructure/`**: Database access, models, and persistence
- **`src/restapi/`**: HTTP handlers and routing
- **`migrations/`**: Database migration files
- **`simple-test/`**: Manual API test scripts

### Adding a New Migration

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Example
sqlx migrate add add_user_email_field
```

This creates a new file in `migrations/` where you can write your SQL statements.

### Database Connection Management

The application uses SQLx with connection pooling. Configuration is done via environment variables in `.env`:

- `PJSIP_DB_POOL_SIZE`: Maximum number of connections
- `PJSIP_DB_MAX_LIFETIME`: Maximum lifetime of a connection (seconds)
- `PJSIP_DB_MAX_IDLE`: Maximum idle time before closing (seconds)
- `PJSIP_DB_TIMEOUT`: Connection timeout (seconds)

### Logging

Logging is configured via the `RUST_LOG` environment variable:

```bash
# Debug level (default)
RUST_LOG=debug cargo run

# Info level
RUST_LOG=info cargo run

# Trace level (verbose)
RUST_LOG=trace cargo run
```

## Troubleshooting

### Database Connection Issues

1. **Check PostgreSQL container is running**:

```bash
./pgsql_container.sh status
```

2. **Verify database credentials** in `.env` match those in `pgsql_container.sh`.

3. **Test database connection**:

```bash
psql -h 127.0.0.1 -p 5432 -U api_user_rw -d asterisk
```

### Migration Errors

1. **Check migration status**:

```bash
sqlx migrate info
```

2. **Revert last migration** (if needed):

```bash
sqlx migrate revert
```

3. **Rebuild database** (development only):

```bash
sqlx database drop
sqlx database create
sqlx migrate run
```

### Compilation Errors

1. **Clean and rebuild**:

```bash
cargo clean
cargo build
```

2. **Update dependencies**:

```bash
cargo update
```

### Port Already in Use

If port 3000 is already in use, change `LISTEN_PORT_V4` in `.env`:

```bash
LISTEN_PORT_V4=8080
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

Copyright 2025 AI Talker API Contributors

---

For more details on database migrations, see [migrations/README.md](migrations/README.md).

