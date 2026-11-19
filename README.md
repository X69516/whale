
# Whale

**Whale** is a security-oriented HTTP server written in Rust.

The project explores a security-first server architecture based on
Zero Trust principles, process isolation and attack surface minimization.

## Project goals

- Minimize the attack surface.
- Separate control plane and data plane.
- Apply least-privilege principles.
- Isolate request execution.
- Implement explicit identity and authorization.
- Provide auditable security decisions.

## Architecture

Whale is organized as a Cargo workspace.

### Applications

| Application | Responsibility |
|---|---|
| serverd | Network server |
| supervisord | Process and worker supervision |
| whale-cli | Command-line interface |

### Libraries

| Crate | Responsibility |
|---|---|
| whale-core | Shared primitives |
| whale-tcp | TCP transport |
| whale-tls | TLS transport |
| whale-http | HTTP protocol |
| whale-router | Request routing |
| whale-policy | Authorization policies |
| whale-identity | Identity management |
| whale-ipc | Inter-process communication |
| whale-protocol | Internal protocol |
| whale-worker | Worker execution |
| whale-supervisor | Worker lifecycle |
| whale-sandbox | Process isolation |
| whale-config | Configuration |
| whale-audit | Security auditing |

## Current status

**Version: 0.1.0**

The project is currently in its initial foundation stage.

Implemented:
- Cargo workspace.
- Application entrypoints.
- TCP listener.
- Connection limits.
- Basic application lifecycle.
- Graceful shutdown.
- Structured logging.

Not implemented yet:
- HTTP parsing.
- TLS.
- IPC.
- Zero Trust policy engine.
- Identity management.
- OS-level sandboxing.
- Worker execution.

## Requirements

- Rust edition 2024
- Cargo
- Tokio-compatible platform

## Build

```bash
cargo build --workspace
```

## Run

```bash
cargo run -p serverd
```

```bash
cargo run -p supervisord
```

## Test

```bash
cargo test --workspace
```

## Configuration

The current server supports:

| Variable | Default |
|---|---|
| WHALE_LISTEN_ADDR | 127.0.0.1:8080 |
| RUST_LOG | info |

## License

MIT

