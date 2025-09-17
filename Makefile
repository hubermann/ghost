format:; cargo fmt --all
lint:; cargo clippy --workspace --all-targets -- -D warnings
test:; cargo test --workspace
run-backend:; cd backend && cargo run
run-frontend:; cd frontend && trunk serve --open
