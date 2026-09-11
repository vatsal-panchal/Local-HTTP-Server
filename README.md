# Local HTTP Server

A simple HTTP server made with Rust and Axum.

## Folder Structure

```
Local HTTP Server/
├── Cargo.toml
├── .gitignore
├── README.md
└── src/
    ├── main.rs
    ├── lib.rs
    ├── config.rs
    ├── errors.rs
    ├── routes/
    │   ├── mod.rs
    │   ├── home.rs
    │   ├── greet.rs
    │   ├── health.rs
    │   └── echo.rs
    └── models/
        ├── mod.rs
        ├── request.rs
        └── response.rs
```

## How to Run

```bash
cargo run
```

Server will start on http://localhost:3000

## Routes

- `GET /` — Hello from Rust!
- `GET /about` — About message
- `GET /hello/:name` — Hello, {name}!
- `GET /health` — Health check
- `GET /status` — Server status
- `POST /echo` — Echoes back your message
