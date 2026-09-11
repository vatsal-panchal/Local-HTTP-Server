# Local HTTP Server 🦀

A simple, modular local HTTP server built with **Rust** and **Axum**.

## Project Structure

```
src/
├── main.rs              # Entry point — starts the server
├── lib.rs               # App builder — wires routes & middleware
├── config.rs            # Server configuration (host, port)
├── errors.rs            # Custom error types & responses
├── routes/
│   ├── mod.rs           # Route module declarations
│   ├── home.rs          # GET /  and  GET /about
│   ├── greet.rs         # GET /hello/:name
│   ├── health.rs        # GET /health  and  GET /status
│   └── echo.rs          # POST /echo
└── models/
    ├── mod.rs           # Model module declarations
    ├── request.rs       # Request body structs
    └── response.rs      # Response body structs
```

## Routes

| Method | Path           | Description              | Response Type |
| ------ | -------------- | ------------------------ | ------------- |
| GET    | `/`            | Home page                | Plain text    |
| GET    | `/about`       | About this server        | Plain text    |
| GET    | `/hello/:name` | Greet by name            | JSON          |
| GET    | `/health`      | Health check             | JSON          |
| GET    | `/status`      | Server status & version  | JSON          |
| POST   | `/echo`        | Echo back your message   | JSON          |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)

### Run the Server

```bash
cargo run
```

The server will start on **http://localhost:3000**.

### Test the Routes

```bash
# Home
curl http://localhost:3000/

# About
curl http://localhost:3000/about

# Greet
curl http://localhost:3000/hello/Vatsal

# Health check
curl http://localhost:3000/health

# Server status
curl http://localhost:3000/status

# Echo (POST)
curl -X POST http://localhost:3000/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from curl!"}'
```

## Tech Stack

- **Rust** — Systems programming language
- **Axum** — Web framework
- **Tokio** — Async runtime
- **Serde** — Serialization / Deserialization
- **Tower-HTTP** — CORS & request tracing middleware
- **Tracing** — Structured logging

## License

This project is for learning purposes.
