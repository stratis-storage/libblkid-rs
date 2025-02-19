ifeq ($(origin PROFILE), undefined)
else
  PROFILE_FLAGS = -C instrument-coverage
endif

ifeq ($(origin MANIFEST_PATH), undefined)
else
  MANIFEST_PATH_ARGS = --manifest-path=${MANIFEST_PATH}
endif

ifeq ($(origin FEDORA_RELEASE), undefined)
else
  FEDORA_RELEASE_ARGS = --release=${FEDORA_RELEASE}
endif

ifeq ($(origin CLIPPY_FIX), undefined)
  CLIPPY_OPTS = --all-targets --no-deps
else
  CLIPPY_OPTS = --fix
endif

IGNORE_ARGS ?=

${HOME}/.cargo/bin/cargo-audit:
	cargo install cargo-audit

audit: ${HOME}/.cargo/bin/cargo-audit
	PATH=${HOME}/.cargo/bin:${PATH} cargo audit -D warnings

build:
	RUSTFLAGS="${DENY} ${PROFILE_FLAGS}" cargo build

build-deprecated:
	RUSTFLAGS="${DENY} ${PROFILE_FLAGS}" cargo build --features=deprecated

check-typos:
	typos

SET_LOWER_BOUNDS ?=
test-set-lower-bounds:
	echo "Testing that SET_LOWER_BOUNDS environment variable is set to a valid path"
	test -e "${SET_LOWER_BOUNDS}"

verify-dependency-bounds: test-set-lower-bounds
	cargo build ${MANIFEST_PATH_ARGS} --all-features
	${SET_LOWER_BOUNDS} ${MANIFEST_PATH_ARGS}
	cargo build ${MANIFEST_PATH_ARGS} --all-features

test-compare-fedora-versions:
	echo "Testing that COMPARE_FEDORA_VERSIONS environment variable is set to a valid path"
	test -e "${COMPARE_FEDORA_VERSIONS}"

check-fedora-versions: test-compare-fedora-versions
	${COMPARE_FEDORA_VERSIONS} ${MANIFEST_PATH_ARGS} ${FEDORA_RELEASE_ARGS} ${IGNORE_ARGS}

clippy:
	cargo clippy --all-features ${CLIPPY_OPTS}
	(cd libblkid-rs-sys && \
        cargo clippy --all-features ${CLIPPY_OPTS})

docs-rust:
	cargo doc --no-deps --package libblkid-rs --package libblkid-rs-sys

docs-ci: docs-rust

yamllint:
	yamllint --strict .github/workflows/*.yml

fmt:
	cargo fmt

fmt-ci:
	cargo fmt -- --check

release:
	RUSTFLAGS="${DENY} ${PROFILE_FLAGS}" cargo build --release

test:
	RUSTFLAGS="${DENY} ${PROFILE_FLAGS}" RUST_BACKTRACE=1 cargo test

.PHONY:
	build
	check-fedora-versions
	check-typos
	clippy
	docs-rust
	docs-ci
	fmt
	fmt-ci
	release
	test
	test-compare-fedora-versions
	test-set-lower-bounds
	verify-dependency-bounds
	yamllint
