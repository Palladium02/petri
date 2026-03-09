build-spec:
    typst c spec.typ spec.pdf

lint:
    cargo clippy -- -D clippy::pedantic -D clippy::nursery
