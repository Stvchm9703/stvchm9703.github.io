use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JupyterNotebookRoot {
    pub cells: Vec<Cell>,
    pub metadata: TopLevelMetadata,
    pub nbformat: usize,
    pub nbformat_minor: usize,
    pub file_url: Option<String>,
}

// type Cell = CodeCell | MarkdownCell;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "cell_type")]
pub enum Cell {
    Unknown,
    Code(CodeCell),
    Markdown(MarkdownCell),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Unknown
    }
}

// pub struct BaseCell {
//   pub cell_type : CellType,
//   pub metadata : CellMetadata,
//   pub source : Vec<String>,
// }

type CellMetadata = serde_json::Map<String, serde_json::Value>;

type Attachments = serde_json::Map<String, serde_json::Value>;

// pub enum CellType {
//     Code = "code",
//     Markdown = "markdown",
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CodeCell {
    // pub cell_type : CellType.Code,
    pub execution_count: Option<usize>,
    pub metadata: CellMetadata,
    pub source: Vec<String>,
    pub outputs: Option<Vec<Output>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownCell {
    // pub cell_type : CellType.Markdown,
    pub metadata: CellMetadata,
    pub source: Vec<String>,
    pub attachments: Option<Attachments>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Output {
    pub ename: Option<String>,
    pub evalue: Option<String>,
    pub output_type: OutputType,
    pub traceback: Option<Vec<String>>,
    pub data: Option<Data>,
    pub execution_count: Option<usize>,
    pub metadata: Option<OutputMetadata>,
    pub name: Option<String>,
    pub text: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Data {
    #[serde(rename = "text/plain")]
    pub text_plain: Vec<String>,
    #[serde(rename = "text/html", skip_serializing_if = "Option::is_none")]
    pub text_html: Option<Vec<String>>,
    #[serde(rename = "image/png", skip_serializing_if = "Option::is_none")]
    pub image_png: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OutputMetadata {
    pub needs_background: Option<NeedsBackground>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NeedsBackground {
    Light,
}
impl Default for NeedsBackground {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputType {
    None,
    DisplayData,
    Error,
    ExecuteResult,
    Stream,
}

impl Default for OutputType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TopLevelMetadata {
    pub kernelspec: Kernelspec,
    pub language_info: LanguageInfo,
    pub orig_nbformat: usize,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Kernelspec {
    pub display_name: String,
    pub language: String,
    pub name: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LanguageInfo {
    pub codemirror_mode: CodemirrorMode,
    pub file_extension: String,
    pub mimetype: String,
    pub name: String,
    pub nbconvert_exporter: String,
    pub pygments_lexer: String,
    pub version: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CodemirrorMode {
    pub name: String,
    pub version: usize,
}
