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
    GraphQl,
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
    #[serde(rename = "java/c/c++/c#")]
    // #[deprecated = "This option exists in the API but not in the UI."]
    JavaCCppCsharp,
}
