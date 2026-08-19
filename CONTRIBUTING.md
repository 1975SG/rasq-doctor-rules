# Contributing a rule

1. A rule proposes one specific, real check — not a bundle of unrelated
   checks in one crate. If a PR adds two independent findings, it
   should be two rule crates.
2. `check()` must stay pure: read project state via the `host` import
   only (`read-file`/`list-files`), never assume filesystem access
   outside the sandbox, never write anything. This is the same
   contract iderm's own default ruleset holds to — a rule that
   violates it will be rejected regardless of what it detects.
3. Include a short doc comment on `rule_id()` or above `impl Guest`
   explaining *why* the check exists, not just what it does. A finding
   message a reader can't act on without more context is a weaker rule
   than one that names the actual problem.
4. Test the rule against a real project exhibiting both the pass and
   fail case before opening the PR — not just that it compiles.
5. State which `iderm:plugin` ABI version (from your vendored `.wit`
   file's `package` line) the rule was built and tested against, in
   the PR description.

## License and the sign-off line

Everything here is dual-licensed under MIT or Apache-2.0, matching
iderm (`LICENSE-MIT` / `LICENSE-APACHE`). By opening a PR you're
confirming you have the right to offer your contribution under that
same license — add a `Signed-off-by: Your Name <email>` line to each
commit (`git commit -s` adds it for you) as that confirmation. No
separate paperwork, just that one line.
