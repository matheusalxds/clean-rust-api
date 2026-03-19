# Clean Rust API

A web API built with Rust following Clean Architecture principles.

## Table of Contents

- [Requirements](#requirements)
- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Available Routes](#available-routes)
- [Tech Stack](#tech-stack)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [bacon](https://crates.io/crates/bacon) (optional — for development with auto-reload)

## Getting Started

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install bacon for development (optional)
cargo install bacon
```

### Environment Variables

Copy the sample environment file and adjust as needed:

```bash
cp .env.sample .env
```

### Running

```bash
# Standard mode
cargo run

# Development mode (auto-reload on file changes)
bacon run
```

The server starts at `http://localhost:3033`.

## Architecture

This project follows **Clean Architecture**, organizing code into layers with strict dependency rules — inner layers never depend on outer layers.

```
Domain  ←  Application  ←  Infrastructure  ←  Presentation  ←  Main
```

### Layers

| Layer | Responsibility | Example |
|---|---|---|
| **Domain** | Entities, repository traits (interfaces), DTOs | `entity.rs`, `repo.rs`, `dto.rs` |
| **Application** | Use cases / business logic (services) | `user_service.rs` |
| **Infrastructure** | Concrete implementations (storage, external APIs) | `repo.rs` (in-memory impl) |
| **Presentation** | HTTP handlers, request/response DTOs, routing | `controller.rs`, `router.rs` |
| **Main** | Composition root — wires all layers together | `factory.rs` |

### Dependency Flow

```
Request → Presentation (controller) → Application (service) → Domain (trait)
                                                                    ↑
                                                          Infrastructure (impl)
```

- **Services** depend on **traits** (defined in Domain), not concrete implementations.
- **Factory** is the composition root — it instantiates repositories, services, and routers, then returns a ready-to-use `Router`.
- This enables easy testing: inject a mock that implements the trait without touching the real database.

### Example: Factory (Composition Root)

```rust
pub fn make_user_router() -> Router {
    let repository = Arc::new(UserRepository::new());
    let service = Arc::new(UserService::new(repository));
    user_router(service)
}
```

The `main.rs` simply merges all module routers:

```rust
let app = Router::new()
    .merge(health_router().with_state(health_service))
    .merge(make_user_router());
```

## Project Structure

```
src/
├── main.rs                          # Application entry point
└── modules/
    ├── mod.rs                       # Module declarations
    ├── health/                      # Health check module
    │   ├── health_service.rs
    │   └── router.rs
    ├── shared/                      # Shared utilities
    │   ├── http_helper.rs
    │   └── logger.rs
    └── user/                        # User module (Clean Architecture)
        ├── domain/                  # Entities, traits, DTOs
        │   ├── entity.rs
        │   ├── repo.rs             # Repository trait (interface)
        │   └── dto.rs
        ├── application/             # Use cases / business logic
        │   └── user_service.rs
        ├── infra/                   # Concrete implementations
        │   └── repo.rs             # In-memory repository
        ├── presentation/            # HTTP layer
        │   ├── controller.rs       # Request handlers
        │   ├── dto.rs              # Request/Response DTOs
        │   └── router.rs           # Route definitions
        └── main/                    # Composition root
            └── factory.rs          # Wires all layers together
```

## Available Routes

| Method | Route   | Description       |
|--------|---------|-------------------|
| GET    | /health | API health check  |
| GET    | /users  | List all users    |
| POST   | /users  | Create a new user |

## Tech Stack

### Core

| Crate | Purpose |
|---|---|
| [axum](https://crates.io/crates/axum) | Web framework — routing, extractors, middleware |
| [tokio](https://crates.io/crates/tokio) | Async runtime — powers the entire async ecosystem |

### Serialization

| Crate | Purpose |
|---|---|
| [serde](https://crates.io/crates/serde) | Serialization/deserialization framework |

### Observability

| Crate | Purpose |
|---|---|
| [tracing](https://crates.io/crates/tracing) | Structured, event-based diagnostic logging |
| [tracing-subscriber](https://crates.io/crates/tracing-subscriber) | Log formatting and filtering (env-based) |

### Configuration

| Crate | Purpose |
|---|---|
| [dotenvy](https://crates.io/crates/dotenvy) | Load environment variables from `.env` files |

## Author

<a href="https://github.com/matheusalxds">
  <img src="https://github.com/matheusalxds.png" width="80" style="border-radius: 50%;" alt="matheusalxds" />
  <br />
  <sub><b>Matheus</b></sub>
  <br />
  <sub>@matheusalxds</sub>
</a>
