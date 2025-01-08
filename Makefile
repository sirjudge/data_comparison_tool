test:
	make clean-logs
	cargo test

test-full-clean:
	make clean
	make docker-build
	cargo test

binary-release:
	make clean-logs
	cargo build  --release
	./target/release/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -config=src/comparison_default.toml

binary-release-terminal:
	make clean-logs
	cargo build --release
	./target/release/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -config=src/comparison_default.toml -tui

binary-debug:
	make clean-logs
	cargo build
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -config=src/comparison_default.toml

binary-debug-terminal:
	make clean-logs
	cargo build
	RUST_BACKTRACE=1 ./target/debug/data_comparison_tool -t1=test_1 -t2=test_2 -gen=100 -output=test.csv -tui -v

build-debug:
	docker-build
	cargo build

build-release:
	docker-build
	cargo build --release

docker-build:
	docker compose up -d

clean:
	docker compose down
	cargo clean
	rm -f *.sqlite*
	rm -f *.csv
	rm -f *.log

clean-logs:
	rm -f *.log

clean-output:
	rm -f *.csv
	rm -f *.sqlite*

profile-release-build:
	cargo build --release --timings
