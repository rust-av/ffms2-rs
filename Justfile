lcov:
    cargo llvm-cov --lcov --output-path=lcov.info --ignore-filename-regex tests\.rs
    genhtml lcov.info --dark-mode --flat --missed --output-directory target/coverage_html

precommit:
    cargo fmt
    cargo clippy
    just lcov
