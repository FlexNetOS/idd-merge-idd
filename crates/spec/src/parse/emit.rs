//! `SpecDoc` -> well-formed Markdown (rebuild strategy, design §5 emit phase).
//!
//! Emit strategy chosen: **rebuild from the model** (string assembly of the
//! stored CommonMark block spans), not arena splicing. The block spans were
//! produced by comrak's `format_commonmark` at parse time, so they are already
//! normalized CommonMark; re-emitting them verbatim makes `parse(emit(x))`
//! stable. Byte-exact parity with the oracle's quirky blank-line tightening is
//! an explicit NON-goal (the golden test compares parsed models); this emitter
//! produces clean, well-formed Markdown with one blank line between blocks.

use crate::model::SpecDoc;

/// Render a [`SpecDoc`] to Markdown ending in a single trailing newline.
pub fn emit_spec(doc: &SpecDoc) -> String {
    let mut sections: Vec<String> = Vec::new();

    if let Some(title) = &doc.title {
        sections.push(format!("# {title}"));
    }

    if let Some(purpose) = &doc.purpose {
        let mut s = String::from("## Purpose");
        for b in purpose {
            s.push_str("\n\n");
            s.push_str(b.text.trim_end());
        }
        sections.push(s);
    }

    // The `## Requirements` heading, then each requirement block.
    sections.push("## Requirements".to_string());

    for req in &doc.requirements {
        let mut s = format!("### Requirement: {}", req.name);
        for b in &req.body {
            s.push_str("\n\n");
            s.push_str(b.text.trim_end());
        }
        for scen in &req.scenarios {
            s.push_str(&format!("\n\n#### Scenario: {}", scen.name));
            for step in &scen.steps {
                s.push_str("\n\n");
                s.push_str(step.text.trim_end());
            }
        }
        sections.push(s);
    }

    let mut out = sections.join("\n\n");
    out.push('\n');
    out
}
