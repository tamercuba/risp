rust-docs:  ## Opens the documentations of rust std
	@rustup doc --std --

project-docs:  ## Opens the documentations of the project
	@cargo doc --open

docs: rust-docs project-docs ## Opens the documentations of rust std and the project


help:  ## Display this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)