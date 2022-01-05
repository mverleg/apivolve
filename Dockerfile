
FROM mverleg/rust_nightly_musl_base:nodeps_2022-01-01

# Copy Cargo files to compile dependencies
COPY ./Cargo.toml ./Cargo.lock ./deny.toml ./

# Build dependencies (debug)
RUN sh ./build_dependencies_only.sh --features=jemalloc

# Check dependencies
RUN cargo --offline audit --deny warnings
RUN cargo --offline deny check advisories
RUN cargo --offline deny check licenses
RUN cargo --offline deny check bans
#RUN cargo --offline outdated --exit-code 1
#TODO @mark: ^turn back on when 2021 works

# Copy the actual code.
COPY ./build.rs ./grammar.lalrpop ./
COPY ./src ./src
RUN find . -name target -prune -o -type f &&\
    touch -c build.rs src/main.rs src/lib.rs

# Build
RUN cargo build --features=jemalloc

# Test
RUN cargo --offline test --features=jemalloc

# Lint
RUN cargo --offline clippy --features=jemalloc --tests -- -D warnings

# Style
RUN cargo --offline fmt --all -- --check


