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

    /// full host round trip, not a stub
    /// read `.gitignore` through `host::read_file` to prove a standalone
    /// rule crosses the WASM sandbox boundary
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
