all:
	@cargo build --release
	@install -m 755 target/release/kairo tao/bin/core/kairo
	@install -m 755 target/release/kairo-status tao/bin/core/kairo-status
	@install -m 755 target/release/kairo-commit tao/bin/core/kairo-commit
	@install -m 755 target/release/kairo-diff tao/bin/core/kairo-diff
	@install -m 755 target/release/kairo-tree tao/bin/core/kairo-tree
	@install -m 755 target/release/kairo-web tao/bin/core/kairo-web
archive:
	tar cf chronos.tar.gz src
