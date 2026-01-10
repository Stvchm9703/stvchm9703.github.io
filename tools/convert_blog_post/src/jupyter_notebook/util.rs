use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Error, anyhow};
use serde_json;

use super::model::JupyterNotebookRoot;
// export function readJupyterNotebook(fileContent: string): JupyterNotebookRoot {
//   const parsedContent = JSON.parse(fileContent);
//   return parsedContent as JupyterNotebookRoot;
// }

pub fn read_jupyter_notebook(file_path: &Path) -> Result<JupyterNotebookRoot, Error> {
    // let file = file!(file_path);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}

// export function resolveJupyterCellOutput(lines: string[]): string[] {
//   let result: string[] = [];
//   let inSkipCount = false;

//   for (let line of lines) {
//     // remove style
//     if (line.startsWith("<style")) {
//       inSkipCount = true;
//       continue;
//     }

//     if (inSkipCount && line.startsWith("</style>")) {
//       inSkipCount = false;
//       continue;
//     }

//     if (inSkipCount) {
//       continue;
//     }

//     result.push(line);
//   }
//   if (inSkipCount) {
//     throw new Error("Unbalanced <style> tag");
//   }
//   return result;
// }
// #[warn(unsafe_code)]
pub fn resolve_jupyter_cell_output(lines: &Vec<String>) -> Result<Vec<String>, Error> {
    let mut result = vec![];
    let mut in_skip_count = false;

    // for (let line of lines) {
    for line in lines.iter() {
        // remove style
        if line.starts_with("<style") {
            in_skip_count = true;
            continue;
        }

        if in_skip_count && line.starts_with("</style>") {
            in_skip_count = false;
            continue;
        }

        if in_skip_count {
            continue;
        }

        result.push(line.to_owned());
    }
    if in_skip_count {
        // throw new Error("Unbalanced <style> tag");
        return Err(anyhow!("Unbalanced <style> tag"));
    }
    return Ok(result);
}
