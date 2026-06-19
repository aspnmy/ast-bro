use crate::core::{Declaration, DeclarationKind, ParseResult};
use std::path::Path;

pub fn parse_toml(path: &Path, source: &[u8]) -> ParseResult {
    let line_count = source.iter().filter(|&&b| b == b'\n').count() + 1;
    let mut decls = Vec::new();

    if let Ok(text) = std::str::from_utf8(source) {
        if let Ok(doc) = text.parse::<toml_edit::ImDocument<String>>() {
            // Walk top-level keys
            for (key, value) in doc.iter() {
                let key_str = key.to_string();
                let sig = _value_summary(value);
                let decl = Declaration {
                    kind: DeclarationKind::Field,
                    name: key_str,
                    signature: sig,
                    bases: Vec::new(),
                    attrs: Vec::new(),
                    docs: Vec::new(),
                    docs_inside: false,
                    visibility: String::new(),
                    start_line: 1,
                    end_line: line_count,
                    start_byte: 0,
                    end_byte: source.len(),
                    doc_start_byte: 0,
                    native_kind: None,
                    modifiers: Vec::new(),
                    deprecated: false,
                    children: _table_children(value),
                    calls: Vec::new(),
                };
                decls.push(decl);
            }
        }
    }

    ParseResult {
        path: path.to_path_buf(),
        language: "toml",
        source: source.to_vec(),
        line_count,
        declarations: decls,
        error_count: 0,
        imports: Vec::new(),
    }
}

fn _table_children(value: &toml_edit::Item) -> Vec<Declaration> {
    match value {
        toml_edit::Item::Table(table) => {
            let mut children = Vec::new();
            for (key, val) in table.iter() {
                children.push(Declaration {
                    kind: DeclarationKind::Field,
                    name: key.to_string(),
                    signature: _value_summary(val),
                    bases: Vec::new(),
                    attrs: Vec::new(),
                    docs: Vec::new(),
                    docs_inside: false,
                    visibility: String::new(),
                    start_line: 1,
                    end_line: 1,
                    start_byte: 0,
                    end_byte: 0,
                    doc_start_byte: 0,
                    native_kind: None,
                    modifiers: Vec::new(),
                    deprecated: false,
                    children: Vec::new(),
                    calls: Vec::new(),
                });
            }
            children
        }
        _ => Vec::new(),
    }
}

fn _value_summary(value: &toml_edit::Item) -> String {
    match value {
        toml_edit::Item::None => "none".to_string(),
        toml_edit::Item::Value(v) => _scalar_summary(v),
        toml_edit::Item::Table(_) => "table".to_string(),
        toml_edit::Item::ArrayOfTables(_) => "array of tables".to_string(),
    }
}

fn _scalar_summary(value: &toml_edit::Value) -> String {
    match value {
        toml_edit::Value::String(s) => {
            let v = s.value();
            if v.len() > 40 {
                format!("\"...{}...\"", &v[..40])
            } else {
                format!("\"{}\"", v)
            }
        }
        toml_edit::Value::Integer(i) => format!("{}", i.value()),
        toml_edit::Value::Float(f) => format!("{}", f.value()),
        toml_edit::Value::Boolean(b) => format!("{}", b.value()),
        toml_edit::Value::Datetime(d) => d.to_string(),
        toml_edit::Value::Array(a) => format!("array[{}]", a.len()),
        toml_edit::Value::InlineTable(t) => format!("inline table[{}]", t.len()),
    }
}
