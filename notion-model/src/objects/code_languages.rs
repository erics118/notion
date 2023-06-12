use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CodeLanguage {
    ABAP,
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
    CSS,
    Dart,
    Diff,
    Docker,
    Erlang,
    Flow,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    GLSL,
    Go,
    Graphql,
    Groovy,
    Haskell,
    HTML,
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
    MATLAB,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    OCaml,
    Pascal,
    Perl,
    PHP,
    #[serde(rename = "plain text")]
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
    VBNet,
    Verilog,
    VHDL,
    #[serde(rename = "visual basic")]
    VisualBasic,
    WebAssembly,
    XML,
    YAML,
    #[serde(rename = "java/c/c++/c#")]
    JavaCCppCSharp,
}
