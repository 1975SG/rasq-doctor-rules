# iderm-doctor-rules

I keep the doctor-rule plugins for [iderm](https://github.com/1975SG/iderm)
here — a WASM component per rule, built against iderm's real
`doctor-rule` WIT interface (`iderm:plugin`). Core ships a small
default ruleset built in; this repo is where I put checks beyond
that, versioned and reviewable on their own.

View plugins, repair recipes, and niche language manifests live in
the sibling repo, `iderm-plugins`, instead — I keep this one separate
since it existed first and I saw no reason to fold real, working
history into a rename.

**Status: private, pre-OSS staging.** This repo exists ahead of
iderm's own public release, so I can get the scaffold, the ABI-pinning
convention, and the contribution shape right while nobody but me is
depending on any of it yet.

## Layout

```
rules/
  example-gitignore-present/   # canonical starter — copy this to add a rule
    Cargo.toml
    src/lib.rs
    wit/deps/iderm-plugin/iderm-plugin.wit
```

Each rule is its own crate under `rules/`, not a workspace member
sharing one `Cargo.toml` — I want a rule to build and version
independently of every other rule here, the same way iderm's own
example plugins do.

## ABI versioning

Every rule crate vendors its own copy of `wit/deps/iderm-plugin/
iderm-plugin.wit`, pinned to a specific `package iderm:plugin@X.Y.Z`
line at the top of that file. I follow the same MAJOR.MINOR.PATCH
compatibility policy iderm itself commits to for the plugin ABI:

- **MAJOR** — breaking change to `types`/`host`/`doctor-rule`. A rule
  built against an older MAJOR will not load.
- **MINOR** — additive only (a new optional field, a new interface). A
  rule built against an older MINOR still loads against a newer host.
- **PATCH** — no interface change.

A rule's own `Cargo.toml` doesn't declare a compatible range — the
vendored `.wit` file's version *is* the declaration. Bumping to a new
iderm ABI means re-copying the `.wit` file from the iderm repo I'm
proposing against and rebuilding.

## Adding a rule

1. Copy `rules/example-gitignore-present/` to `rules/<your-rule-name>/`.
2. Rename the package in `Cargo.toml` (`name`,
   `package.metadata.component.package`) and update `rule_id()` in
   `src/lib.rs` to match.
3. Implement `check(project) -> Vec<Finding>` — pure, read-only, same
   contract every doctor-rule plugin holds to (no writes, no side
   effects; that's what keeps `doctor` safe to run automatically).
4. `cargo component build --release` — produces a real `.wasm` under
   `target/wasm32-wasip1/release/`.
5. Open a PR. See `CONTRIBUTING.md`.

## Building

Requires [`cargo-component`](https://github.com/bytecodealliance/cargo-component):

```sh
cargo install cargo-component
cd rules/<rule-name>
cargo component build --release
```

## License

Dual-licensed under MIT or Apache-2.0, matching iderm itself — see
`LICENSE-MIT` / `LICENSE-APACHE`.
