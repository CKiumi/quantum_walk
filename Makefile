doc:
	cargo doc --no-deps --open
test:
	cargo test test::line::test -- --show-output
tree:
	cargo tree -i getrandom:0.2.2
plot:
	cargo test --package quantum_walk --lib -- line::test --exact --nocapture