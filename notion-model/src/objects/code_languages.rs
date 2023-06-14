use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CodeLanguage {
    Abap,
    Arduino,
    Bash,
    Basic,
    C,
    Clojure,
    CoffeeScript,
    #[serde(rename = "c++")]
    Cpp,
    #[serde(rename = "c#")]
    CSharp,
    Css,
    Dart,
    Diff,
    Docker,
    Erlang,
    Flow,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    Glsl,
    Go,
    Graphql,
    Groovy,
    Haskell,
    Html,
    Java,
    JavaScript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    LiveScript,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    OCaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    #[default]
    PlainText,
    PowerShell,
    Prolog,
    Protobuf,
    Python,
    R,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Sql,
    Swift,
    TypeScript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    VHDL,
    #[serde(rename = "visual basic")]
    VisualBasic,
    WebAssembly,
    Xml,
    Yaml,
    /// Java, C, C++, C#
    ///
    /// Probably is deprecated now, as it's not in the UI. It still works
    /// though.
    #[serde(rename = "java/c/c++/c#")]
    JavaCCppCsharp,
}
