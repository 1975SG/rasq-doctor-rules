#[allow(warnings)]
mod bindings;

use bindings::exports::iderm::plugin::doctor_rule::Guest;
use bindings::iderm::plugin::host;
use bindings::iderm::plugin::types::{Finding, ProjectInfo, Severity};

struct Component;

impl Guest for Component {
    fn rule_id() -> String {
        "example-gitignore-present".to_string()
    }

    /// Real round trip through `host::read_file`, not just a stub — proves
    /// a rule cloned out of this repo and rebuilt on its own actually
    /// crosses the WASM sandbox boundary correctly, before anyone edits it
    /// into a real check.
    fn check(project: ProjectInfo) -> Vec<Finding> {
        let gitignore_path = format!("{}/.gitignore", project.root_path.trim_end_matches('/'));
        match host::read_file(&gitignore_path) {
            Ok(_) => Vec::new(),
            Err(_) => vec![Finding {
                severity: Severity::Info,
                message: "no .gitignore found at project root".to_string(),
                path: None,
                rule_id: Self::rule_id(),
            }],
        }
    }
}

bindings::export!(Component with_types_in bindings);
