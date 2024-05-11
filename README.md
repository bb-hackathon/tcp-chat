# Архитектура

![Архитектура](.github/assets/architecture.svg)

# Локальная сборка и развертывание сервера

### С помощью Docker

> [!TIP]
> Это наиболее разумный способ развертывания. Остальные приведены для справки.

```bash
docker build -t tcp-chat                      # Собрать только контейнер с сервером.
docker compose up --detach --build tcp-server # Поднять всю серверную часть. (БД, сервер, pgAdmin)
docker compose down                           # Shutdown серверной части.
```

### Без контейнеризации

Установить `cargo` - официальную систему сборки Rust через [`rustup`](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Установка Rust.

cargo build --release # Сборка сервера.
cargo run --release   # Запуск сервера.
cargo clippy          # Линтер.
```

> [!IMPORTANT]
> В системе необходим [`protoc`](https://grpc.io/docs/protoc-installation/)!

### С помощью Nix

```bash
nix build . # Ага, вот так просто.
```
