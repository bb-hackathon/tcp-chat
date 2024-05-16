# Run some pre-commit checks.
precommit:
    cargo clippy
    nix run nixpkgs#typos

# Redeploy all services, rebuilding `server` and `sqlrunner`.
redeploy:
    docker compose down
    docker compose up --detach --build server sqlrunner postgresql pgadmin
