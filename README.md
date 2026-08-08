# iderm-doctor-rules

Community-proposed `doctor-rule` plugins for [iderm](https://github.com/1975SG/iderm) —
a WASM component per rule, built against iderm's real `doctor-rule` WIT
interface (`iderm:plugin`). Core ships a small default ruleset built in;
this repo is where checks beyond that live, versioned and reviewable on
their own, per `docs/modules/Doctor.md`'s own "public, versioned doctor
rules repo" line in the iderm project.

**Status: private, pre-OSS staging.** This repo exists ahead of iderm's
own public release so the scaffold, the ABI-pinning convention, and the
contribution shape below are real and tested before either goes public
together.

## Layout

```
rules/
  example-gitignore-present/   # canonical starter — copy this to add a rule
    Cargo.toml
    src/lib.rs
    wit/deps/iderm-plugin/iderm-plugin.wit
```

Each rule is its own crate under `rules/`, not a workspace member sharing
one `Cargo.toml` — a rule should build and version independently of every
other rule in this repo, the same way iderm's own `plugins/example-*`
directories do.

## ABI versioning

Every rule crate vendors its own copy of `wit/deps/iderm-plugin/
iderm-plugin.wit`, pinned to a specific `package iderm:plugin@X.Y.Z`
line at the top of that file — the same MAJOR.MINOR.PATCH compatibility
policy iderm's own ADR-013 established for the plugin ABI generally:
- **MAJOR** — breaking change to `types`/`host`/`doctor-rule`. A rule
  built against an older MAJOR will not load.
- **MINOR** — additive only (a new optional field, a new interface). A
  rule built against an older MINOR still loads against a newer host.
- **PATCH** — no interface change.

A rule's own `Cargo.toml` doesn't declare a compatible range — the
vendored `.wit` file's version *is* the declaration. Bumping to a new
iderm ABI means re-copying the `.wit` file from the iderm repo it's
being proposed against and rebuilding.

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
