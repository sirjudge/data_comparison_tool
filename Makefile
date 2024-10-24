test:
	cargo build
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv

test-terminal:
	cargo build
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -tui

build-debug:
	cargo build

build-release:
	cargo build --release

clean:
	cargo clean

