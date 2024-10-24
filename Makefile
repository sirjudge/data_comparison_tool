test:
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv

build-debug:
	cargo build

build-release:
	cargo build --release
