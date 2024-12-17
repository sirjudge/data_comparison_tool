test-release:
	cargo build  --release
	RUST_BACKTRACE=1 ./target/release/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv

test-release-terminal:
	cargo build --release
	RUST_BACKTRACE=1 ./target/release/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -tui

test-debug:
	cargo build
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv

test-debug-terminal:
	cargo build
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -tui

build-debug:
	cargo build

build-release:
	cargo build --release

docker-build:
	docker compose up -d

clean-docker:
	docker compose down

clean-files:
	cargo clean
	rm -f *.sqlite*
	rm -f *.csv
	rm -f *.log

