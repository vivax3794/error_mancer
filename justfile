test:
    cargo nextest run
    cargo test --doc

update_ui:
    TRYBUILD=overwrite cargo nextest run ui

docs:
    cargo doc --open
