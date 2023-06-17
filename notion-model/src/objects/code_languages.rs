use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CodeLanguage {
    Abap,
    Agda,
    Arduino,
    Assembly,
    Bash,
    Basic,
    Bnf,
    C,
    #[serde(rename = "c#")]
    CSharp,
    #[serde(rename = "c++")]
    Cpp,
    Clojure,
    CoffeeScript,
    Coq,
    Css,
    Dart,
    Dhall,
    Diff,
    Docker,
    Ebnf,
    Elixir,
    Elm,
    Erlang,
    #[serde(rename = "f#")]
    FSharp,
    Flow,
    Fortran,
    Gherkin,
    Glsl,
    Go,
    #[serde(rename = "graphql")]
    GraphQl,
    Groovy,
    Haskell,
    Html,
    Idris,
    Java,
    #[serde(rename = "javascript")]
    JavaScript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    LiveScript,
    #[serde(rename = "llvm ir")]
    LlvmIr,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mathematica,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    #[serde(rename = "ocaml")]
    OCaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    #[default]
    PlainText,
    #[serde(rename = "powershell")]
    PowerShell,
    Prolog,
    Protobuf,
    #[serde(rename = "purescript")]
    PureScript,
    Python,
    R,
    Racket,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Solidity,
    Sql,
    Swift,
    Toml,
    TypeScript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    #[serde(rename = "webassembly")]
    WebAssembly,
    Xml,
    Yaml,
    // /// Java, C, C++, C#
    // #[serde(rename = "java/c/c++/c#")]
    // #[deprecated = "This option exists in the API but not in the UI."]
    // JavaCCppCsharp,
}
