## Архитектура

![Архитектура](.github/assets/architecture.svg)

## Локальная сборка и развертывание

#### Сервер

Установить `cargo` - официальную систему сборки Rust через [`rustup`](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Установка Rust.

cargo build --release # Сборка сервера.
cargo run --release   # Запуск сервера.
cargo clippy          # Линтер.
```

> [!TIP]
> В системе необходим [`protoc`](https://grpc.io/docs/protoc-installation/)
