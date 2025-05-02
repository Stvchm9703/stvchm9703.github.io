export interface JupyterNotebookRoot {
  cells: Cell[];
  metadata: TopLevelMetadata;
  nbformat: number;
  nbformat_minor: number;
  fileUrl?: string;
}

export type Cell = CodeCell | MarkdownCell;

interface BaseCell {
  cell_type: CellType;
  metadata: CellMetadata;
  source: string[];
}

type CellMetadata = {
  [key: string]: any;
};

type Attachments = {
  [key: string]: any;
};
export interface CodeCell extends BaseCell {
  cell_type: CellType.Code;
  execution_count: number | null;
  outputs?: Output[];
}

export interface MarkdownCell extends BaseCell {
  cell_type: CellType.Markdown;
  attachments?: Attachments;
}

export enum CellType {
  Code = "code",
  Markdown = "markdown",
}

export interface Output {
  ename?: string;
  evalue?: string;
  output_type: OutputType;
  traceback?: string[];
  data?: Data;
  execution_count?: number;
  metadata?: OutputMetadata;
  name?: string;
  text?: string[];
}

export interface Data {
  "text/plain": string[];
  "text/html"?: string[];
  "image/png"?: string;
}

export interface OutputMetadata {
  needs_background?: NeedsBackground;
}

export enum NeedsBackground {
  Light = "light",
}

export enum OutputType {
  DisplayData = "display_data",
  Error = "error",
  ExecuteResult = "execute_result",
  Stream = "stream",
}

export interface TopLevelMetadata {
  kernelspec: Kernelspec;
  language_info: LanguageInfo;
  orig_nbformat: number;
}

export interface Kernelspec {
  display_name: string;
  language: string;
  name: string;
}

export interface LanguageInfo {
  codemirror_mode: CodemirrorMode;
  file_extension: string;
  mimetype: string;
  name: string;
  nbconvert_exporter: string;
  pygments_lexer: string;
  version: string;
}

export interface CodemirrorMode {
  name: string;
  version: number;
}

export function readJupyterNotebook(fileContent: string): JupyterNotebookRoot {
  const parsedContent = JSON.parse(fileContent);
  return parsedContent as JupyterNotebookRoot;
}

export function resolveJupyterCellOutput(lines: string[]): string[] {
  let result: string[] = [];
  let inSkipCount = false;

  for (let line of lines) {
    // remove style
    if (line.startsWith("<style")) {
      inSkipCount = true;
      continue;
    }

    if (inSkipCount && line.startsWith("</style>")) {
      inSkipCount = false;
      continue;
    }

    if (inSkipCount) {
      continue;
    }

    result.push(line);
  }
  if (inSkipCount) {
    throw new Error("Unbalanced <style> tag");
  }
  return result;
}
