default:
    @just --list

alias d := deploy
alias r := redeploy

# Run some pre-commit checks.
precommit:
    cargo clippy
    cargo machete
    nix run nixpkgs#typos

# Redeploy all services, rebuilding `server` and `sqlrunner`.
deploy:
    docker compose down
    docker compose up --detach --build server postgresql pgadmin

# Redeploy a new version of a container.
redeploy CONTAINER:
    docker compose up --detach --build {{CONTAINER}}
