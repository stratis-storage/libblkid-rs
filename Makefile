ifeq ($(origin MANIFEST_PATH), undefined)
else
  MANIFEST_PATH_ARGS = --manifest-path=${MANIFEST_PATH}
endif

ifeq ($(origin FEDORA_RELEASE), undefined)
else
  FEDORA_RELEASE_ARGS = --release=${FEDORA_RELEASE}
endif

RUST_2018_IDIOMS = -D bare-trait-objects \
                   -D ellipsis-inclusive-range-patterns \
                   -D unused-extern-crates

DENY = -D warnings -D future-incompatible -D unused ${RUST_2018_IDIOMS}

${HOME}/.cargo/bin/cargo-audit:
	cargo install cargo-audit

audit: ${HOME}/.cargo/bin/cargo-audit
	PATH=${HOME}/.cargo/bin:${PATH} cargo audit -D warnings

build:
	RUSTFLAGS="${DENY}" cargo build

build-deprecated:
	RUSTFLAGS="${DENY}" cargo build --features=deprecated

verify-dependency-bounds:
	RUSTFLAGS="${DENY}" cargo build ${MANIFEST_PATH_ARGS} --all-features
	${SET_LOWER_BOUNDS} ${MANIFEST_PATH_ARGS}
	RUSTFLAGS="${DENY}" cargo build ${MANIFEST_PATH_ARGS} --all-features

verify-dependency-bounds-sys:
	RUSTFLAGS="${DENY}" cargo build ${MANIFEST_PATH_ARGS} --all-features
	${SET_LOWER_BOUNDS} ${MANIFEST_PATH_ARGS}
	RUSTFLAGS="${DENY}" cargo build ${MANIFEST_PATH_ARGS} --all-features

test-compare-fedora-versions:
	echo "Testing that COMPARE_FEDORA_VERSIONS environment variable is set to a valid path"
	test -e "${COMPARE_FEDORA_VERSIONS}"

check-fedora-versions:
	${COMPARE_FEDORA_VERSIONS} ${MANIFEST_PATH_ARGS} ${FEDORA_RELEASE_ARGS} \
	--ignore-missing libblkid-rs-sys

check-fedora-versions-sys:
	${COMPARE_FEDORA_VERSIONS} ${MANIFEST_PATH_ARGS} ${FEDORA_RELEASE_ARGS} \
	--ignore-low bindgen

clippy:
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::needless_borrow -A clippy::upper_case_acronyms -A clippy::from_over_into

docs-rust:
	cargo doc --no-deps --package libblkid-rs --package libblkid-rs-sys

docs-travis: docs-rust

yamllint:
	yamllint --strict .github/workflows/*.yml

fmt:
	cargo fmt

fmt-travis:
	cargo fmt -- --check

release:
	RUSTFLAGS="${DENY}" cargo build --release

test:
	RUSTFLAGS="${DENY}" RUST_BACKTRACE=1 cargo test

.PHONY:
	build
	check-fedora-versions
	check-fedora-versions-sys
	clippy
	docs-rust
	docs-travis
	fmt
	fmt-travis
	release
	test
	test-compare-fedora-versions
	verify-dependency-bounds
	verify-dependency-bounds-sys
	yamllint
