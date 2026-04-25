use super::base::{collapse_ws, count_parse_errors, field_text, LanguageAdapter};
use crate::core::{Declaration, DeclarationKind, ParseResult};
use ast_grep_core::{Doc, Node};
use std::path::Path;

pub struct RustAdapter;

impl LanguageAdapter for RustAdapter {
    fn language_name(&self) -> &'static str {
        "rust"
    }

    fn parse<'a, D: Doc>(&self, path: &Path, source: &[u8], root: Node<'a, D>) -> ParseResult {
        let mut decls = Vec::new();
        _walk_mod(&root, source, &mut decls);
        ParseResult {
            path: path.to_path_buf(),
            language: self.language_name(),
            source: source.to_vec(),
            line_count: source.iter().filter(|&&b| b == b'\n').count() + 1,
            declarations: decls,
            error_count: count_parse_errors(root.clone()),
        }
    }
}

fn _walk_mod<'a, D: Doc>(node: &Node<'a, D>, src: &[u8], out: &mut Vec<Declaration>) {
    for child in node.children() {
        if !child.is_named() {
            continue;
        }
        if let Some(decl) = _node_to_decl(&child, src) {
            out.push(decl);
        }
    }
}

fn _node_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Option<Declaration> {
    let kind = node.kind();

    if kind == "struct_item" {
        return Some(_struct_to_decl(node, src));
    }
    if kind == "enum_item" {
        return Some(_enum_to_decl(node, src));
    }
    if kind == "trait_item" {
        return Some(_trait_to_decl(node, src));
    }
    if kind == "impl_item" {
        return Some(_impl_to_decl(node, src));
    }
    if kind == "function_item" {
        return Some(_function_to_decl(node, src, false));
    }
    if kind == "mod_item" {
        return Some(_mod_to_decl(node, src));
    }

    None
}

fn _struct_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Declaration {
    let name = field_text(node, "name").unwrap_or_else(|| "?".to_string());

    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let mut children = Vec::new();
    if let Some(body) = node.field("body") {
        if body.kind() == "field_declaration_list" {
            for field in body.children() {
                if field.kind() == "field_declaration" {
                    if let Some(fd) = _field_to_decl(&field, src) {
                        children.push(fd);
                    }
                }
            }
        }
    }

    let sig_end = node
        .field("body")
        .map(|b| b.range().start)
        .unwrap_or(node.range().end);
    let sig = collapse_ws(&String::from_utf8_lossy(&src[node.range().start..sig_end]))
        .trim_end_matches(&[' ', '{', ';'][..])
        .to_string();

    Declaration {
        kind: DeclarationKind::Struct,
        name,
        signature: sig,
        bases: Vec::new(),
        attrs,
        docs,
        docs_inside: false,
        visibility: _visibility(node, src),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children,
    }
}

fn _enum_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Declaration {
    let name = field_text(node, "name").unwrap_or_else(|| "?".to_string());

    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let mut children = Vec::new();
    if let Some(body) = node.field("body") {
        for variant in body.children() {
            if variant.kind() == "enum_variant" {
                let vname = field_text(&variant, "name").unwrap_or_else(|| "?".to_string());
                let vr = variant.range();
                children.push(Declaration {
                    kind: DeclarationKind::EnumMember,
                    name: vname.clone(),
                    signature: vname,
                    bases: Vec::new(),
                    attrs: Vec::new(),
                    docs: Vec::new(),
                    docs_inside: false,
                    visibility: String::new(),
                    start_line: variant.start_pos().line() + 1,
                    end_line: variant.end_pos().line() + 1,
                    start_byte: vr.start,
                    end_byte: vr.end,
                    doc_start_byte: vr.start,
                    children: Vec::new(),
                });
            }
        }
    }

    let sig_end = node
        .field("body")
        .map(|b| b.range().start)
        .unwrap_or(node.range().end);
    let sig = collapse_ws(&String::from_utf8_lossy(&src[node.range().start..sig_end]))
        .trim_end_matches(&[' ', '{'][..])
        .to_string();

    Declaration {
        kind: DeclarationKind::Enum,
        name,
        signature: sig,
        bases: Vec::new(),
        attrs,
        docs,
        docs_inside: false,
        visibility: _visibility(node, src),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children,
    }
}

fn _trait_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Declaration {
    let name = field_text(node, "name").unwrap_or_else(|| "?".to_string());

    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let mut children = Vec::new();
    if let Some(body) = node.field("body") {
        for item in body.children() {
            if item.kind() == "function_signature_item" || item.kind() == "function_item" {
                children.push(_function_to_decl(&item, src, true));
            }
        }
    }

    let sig_end = node
        .field("body")
        .map(|b| b.range().start)
        .unwrap_or(node.range().end);
    let sig = collapse_ws(&String::from_utf8_lossy(&src[node.range().start..sig_end]))
        .trim_end_matches(&[' ', '{'][..])
        .to_string();

    Declaration {
        kind: DeclarationKind::Interface,
        name,
        signature: sig,
        bases: Vec::new(),
        attrs,
        docs,
        docs_inside: false,
        visibility: _visibility(node, src),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children,
    }
}

fn _impl_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Declaration {
    let name = field_text(node, "type").unwrap_or_else(|| "?".to_string());
    let trait_node = node.field("trait");
    let trait_name = trait_node.map(|t| collapse_ws(&t.text()));

    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let mut children = Vec::new();
    if let Some(body) = node.field("body") {
        for item in body.children() {
            if item.kind() == "function_item" {
                children.push(_function_to_decl(&item, src, true));
            }
        }
    }

    let mut sig = "impl ".to_string();
    if let Some(t) = &trait_name {
        sig.push_str(t);
        sig.push_str(" for ");
    }
    sig.push_str(&name);

    Declaration {
        kind: DeclarationKind::Class,
        name: format!("impl_{}", name),
        signature: sig,
        bases: trait_name.into_iter().collect(),
        attrs,
        docs,
        docs_inside: false,
        visibility: String::new(),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children,
    }
}

fn _function_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8], is_method: bool) -> Declaration {
    let name = field_text(node, "name").unwrap_or_else(|| "?".to_string());

    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let sig_end = node
        .field("body")
        .map(|b| b.range().start)
        .unwrap_or(node.range().end);
    let sig = collapse_ws(&String::from_utf8_lossy(&src[node.range().start..sig_end]))
        .trim_end_matches(&[' ', '{', ';'][..])
        .to_string();

    Declaration {
        kind: if is_method {
            DeclarationKind::Method
        } else {
            DeclarationKind::Function
        },
        name,
        signature: sig,
        bases: Vec::new(),
        attrs,
        docs,
        docs_inside: false,
        visibility: _visibility(node, src),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children: Vec::new(),
    }
}

fn _mod_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Declaration {
    let name = field_text(node, "name").unwrap_or_else(|| "?".to_string());

    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let mut children = Vec::new();
    if let Some(body) = node.field("body") {
        _walk_mod(&body, src, &mut children);
    }

    let sig_end = node
        .field("body")
        .map(|b| b.range().start)
        .unwrap_or(node.range().end);
    let sig = collapse_ws(&String::from_utf8_lossy(&src[node.range().start..sig_end]))
        .trim_end_matches(&[' ', '{', ';'][..])
        .to_string();

    Declaration {
        kind: DeclarationKind::Namespace,
        name: name.clone(),
        signature: sig,
        bases: Vec::new(),
        attrs,
        docs,
        docs_inside: false,
        visibility: _visibility(node, src),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children,
    }
}

fn _field_to_decl<'a, D: Doc>(node: &Node<'a, D>, src: &[u8]) -> Option<Declaration> {
    let name = field_text(node, "name")?;
    let mut attrs = Vec::new();
    let mut docs = Vec::new();
    _extract_attrs_and_docs(node, src, &mut attrs, &mut docs);

    let sig = collapse_ws(&String::from_utf8_lossy(
        &src[node.range().start..node.range().end],
    ))
    .trim_end_matches(',')
    .to_string();

    Some(Declaration {
        kind: DeclarationKind::Field,
        name,
        signature: sig,
        bases: Vec::new(),
        attrs,
        docs,
        docs_inside: false,
        visibility: _visibility(node, src),
        start_line: node.start_pos().line() + 1,
        end_line: node.end_pos().line() + 1,
        start_byte: node.range().start,
        end_byte: node.range().end,
        doc_start_byte: _doc_start(node),
        children: Vec::new(),
    })
}

fn _extract_attrs_and_docs<'a, D: Doc>(
    node: &Node<'a, D>,
    _src: &[u8],
    attrs: &mut Vec<String>,
    docs: &mut Vec<String>,
) {
    let mut current = node.prev();
    let mut nodes = Vec::new();
    while let Some(prev) = current {
        if prev.kind() == "line_comment"
            || prev.kind() == "block_comment"
            || prev.kind() == "attribute_item"
        {
            nodes.push(prev.clone());
            current = prev.prev();
        } else {
            break;
        }
    }
    nodes.reverse();
    for n in nodes {
        if n.kind() == "attribute_item" {
            attrs.push(collapse_ws(&n.text()));
        } else {
            let t = n.text().into_owned();
            if t.starts_with("///") || t.starts_with("/**") {
                docs.push(t);
            }
        }
    }
}

fn _doc_start<'a, D: Doc>(node: &Node<'a, D>) -> usize {
    let mut start = node.range().start;
    let mut current = node.prev();
    while let Some(prev) = current {
        if prev.kind() == "line_comment"
            || prev.kind() == "block_comment"
            || prev.kind() == "attribute_item"
        {
            start = prev.range().start;
            current = prev.prev();
        } else {
            break;
        }
    }
    start
}

fn _visibility<'a, D: Doc>(node: &Node<'a, D>, _src: &[u8]) -> String {
    for c in node.children() {
        if c.kind() == "visibility_modifier" {
            return collapse_ws(&c.text());
        }
    }
    String::new() // Rust default is private
}
