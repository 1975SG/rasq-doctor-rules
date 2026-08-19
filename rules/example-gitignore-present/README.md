# example-gitignore-present

I check whether a project has a `.gitignore` at its root and raise a
finding if it doesn't. Small on purpose — this is the canonical
starter rule, meant to be copied as the basis for a new one, not to be
exhaustive on its own.

**Severity:** info.

**Reads:** `.gitignore` at the project root, through `host::read_file`
— nothing else.

The matching repair recipe that proposes a starter `.gitignore` when
this rule fires lives in the sibling `iderm-plugins` repo, under
`repair-recipes/gitignore-scaffold/`.

## Build

```sh
cargo component build --release
```

Produces `target/wasm32-wasip1/release/example_gitignore_present.wasm`.
Copy it into a project's `.iderm/plugins/` to use it.
