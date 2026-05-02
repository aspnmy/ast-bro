use std::path::Path;

use ast_grep_core::Language;
use ast_grep_language::{LanguageExt, SupportLang};

use crate::adapters::base::LanguageAdapter;
use crate::adapters::csharp::CSharpAdapter;
use crate::adapters::go::GoAdapter;
use crate::adapters::java::JavaAdapter;
use crate::adapters::kotlin::KotlinAdapter;
use crate::adapters::python::PythonAdapter;
use crate::adapters::rust::RustAdapter;
use crate::adapters::scala::ScalaAdapter;
use crate::adapters::typescript::TypeScriptAdapter;
use crate::core::ParseResult;

pub fn parse_file_for_hook(path: &Path) -> Option<ParseResult> {
    let lang = SupportLang::from_path(path);
    let source = std::fs::read_to_string(path).ok()?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if matches!(ext, "md" | "markdown" | "mdx" | "mdown") {
        let mut r = crate::adapters::markdown::parse_markdown(path, source.as_bytes());
        crate::core::populate_markers(&mut r.declarations, r.language);
        return Some(r);
    }
    let lang = lang?;
    let mut result = match lang {
        SupportLang::Rust => RustAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        SupportLang::Python => PythonAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        SupportLang::TypeScript | SupportLang::Tsx | SupportLang::JavaScript => {
            TypeScriptAdapter.parse(path, source.as_bytes(), lang.ast_grep(source.clone()).root())
        }
        SupportLang::CSharp => CSharpAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        SupportLang::Go => GoAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        SupportLang::Java => JavaAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        SupportLang::Kotlin => KotlinAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        SupportLang::Scala => ScalaAdapter.parse(
            path,
            source.as_bytes(),
            lang.ast_grep(source.clone()).root(),
        ),
        _ => return None,
    };
    // One central pass enriches every adapter's output with `native_kind`,
    // `modifiers`, and `deprecated` derived from the language's source
    // conventions. Adapters stay focused on tree traversal.
    crate::core::populate_markers(&mut result.declarations, result.language);
    Some(result)
}
