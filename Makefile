all:
	cargo build --release
	install -m 755 target/release/kairo_status tao/bin/core/kairo_status
	install -m 755 target/release/kairo_diff tao/bin/core/kairo_diff