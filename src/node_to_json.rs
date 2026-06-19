use serde_json::{Map, Value};
use typst_syntax::{SyntaxDiagnostic, SyntaxNode};

fn diagnostic_to_json(diag: &SyntaxDiagnostic) -> Value {
    let mut map = Map::new();
    map.insert("message".into(), Value::String(diag.message.to_string()));
    let hints: Vec<Value> = diag
        .hints
        .iter()
        .map(|h| Value::String(h.v.to_string()))
        .collect();
    if !hints.is_empty() {
        map.insert("hints".into(), Value::Array(hints));
    }
    Value::Object(map)
}

fn insert_diags(map: &mut Map<String, Value>, key: &str, diags: &[SyntaxDiagnostic]) {
    if !diags.is_empty() {
        map.insert(
            key.into(),
            Value::Array(diags.iter().map(diagnostic_to_json).collect()),
        );
    }
}

pub fn node_to_json(node: &SyntaxNode) -> Value {
    let mut map = Map::new();
    map.insert("kind".into(), Value::String(format!("{:?}", node.kind())));
    map.insert("len".into(), Value::Number(node.len().into()));

    let children: Vec<_> = node.children().collect();

    match children.first() {
        Some(_) => {
            map.insert(
                "children".into(),
                Value::Array(children.iter().map(|c| node_to_json(c)).collect()),
            );
        }
        None => {
            map.insert("text".into(), Value::String(node.leaf_text().to_string()));
        }
    }

    let (errors, warnings) = node.errors_and_warnings();
    insert_diags(&mut map, "errors", &errors);
    insert_diags(&mut map, "warnings", &warnings);

    Value::Object(map)
}
