build_eval:  ## Build the risp_eval library
	@cargo build -p risp_eval

test-eval: build_eval  ## Run tests for the risp_eval library
	@cargo test --lib -p risp_eval

run-repl:  ## Run the Risp REPL
	@cargo run -p repl

docs:  ## Opens the documentations of rust std and the project
	@cargo doc --open && rustup doc --std

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)