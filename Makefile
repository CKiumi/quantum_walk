doc:
	cargo doc --no-deps --open
test:
	cargo test test::line::tqwo::model2 -- --show-output

plot:
	cargo test --package quantum_walk --lib -- line::test --exact --nocapture