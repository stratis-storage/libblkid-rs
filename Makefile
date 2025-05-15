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

ifeq ($(origin MINIMAL), undefined)
  BUILD = build
else
  BUILD = minimal-versions build --direct
endif

IGNORE_ARGS ?=

${HOME}/.cargo/bin/cargo-audit:
	cargo install cargo-audit

audit: ${HOME}/.cargo/bin/cargo-audit
	PATH=${HOME}/.cargo/bin:${PATH} cargo audit -D warnings

build:
	RUSTFLAGS="${PROFILE_FLAGS}" cargo ${BUILD}

build-deprecated:
	RUSTFLAGS="${PROFILE_FLAGS}" cargo ${BUILD} --features=deprecated

check-typos:
	typos

test-compare-fedora-versions:
	echo "Testing that COMPARE_FEDORA_VERSIONS environment variable is set to a valid path"
	test -e "${COMPARE_FEDORA_VERSIONS}"

check-fedora-versions: test-compare-fedora-versions
	${COMPARE_FEDORA_VERSIONS} ${MANIFEST_PATH_ARGS} ${FEDORA_RELEASE_ARGS} ${IGNORE_ARGS}

clippy:
	cargo clippy ${CLIPPY_OPTS}
	(cd libblkid-rs-sys && \
        cargo clippy ${CLIPPY_OPTS})

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
	RUSTFLAGS="${PROFILE_FLAGS}" cargo ${BUILD} --release

test:
	RUSTFLAGS="${PROFILE_FLAGS}" RUST_BACKTRACE=1 cargo test

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
	yamllint
