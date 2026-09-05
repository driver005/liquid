.PHONY: widgetbook check clean

# Run the visual component browser
widgetbook:
	cargo run --example widgetbook


# Check the library and examples for errors
check:
	cargo check
	cargo check --examples

# Clean the target directory
clean:
	cargo clean
