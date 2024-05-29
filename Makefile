build_eval:
	@cargo build -p risp_eval

test-eval: build_eval
	@cargo test --lib -p risp_eval

run-repl:
	@cargo run -p repl
