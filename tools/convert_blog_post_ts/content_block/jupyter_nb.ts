import type { Page } from "../page/index";
import * as Jupyter from "../jupyter";

export function resolveJupyterComponent(
  self: Page,
  jupyters: Jupyter.JupyterNotebookRoot[],
) {
  let toDelete: string[] = [];
  let list_buffer = self.reformedContents
    .map((elm, idx) => {
      if (
        !(elm.componentType === "Text" && elm.componentAttr["style"] === "Code")
      )
        return elm;

      const { text } = elm.componentAttr;
      if (!text.match(_jupyter_component_regex)) return elm;

      let next_item = self.reformedContents[idx + 1];
      if (next_item.componentType !== "File") return null;
      // console.log({ next_item });
      // if (!next_item.componentAttr["fileName"] ) return null;

      const { fileName, cellNumber } = text.match(
        _jupyter_component_regex,
      ).groups;
      // console.log({ fileName, cellNumber });

      const targetJupyter = jupyters.find((jupyter) =>
        jupyter.fileUrl?.includes(fileName),
      );
      if (!targetJupyter) return elm;

      const cell = targetJupyter.cells[cellNumber];
      if (!cell) return elm;
      // console.log({ cell });
      if (cell.cell_type === "code") {
        const { outputs } = cell as Jupyter.CodeCell;
        for (const output of outputs) {
          if (output.data["text/html"]) {
            const html_str = output.data["text/html"];
            let result = Jupyter.resolveJupyterCellOutput(html_str);
            output.data["text/html"] = result;
          }
          if (output.data["text/plain"]) {
            output.data["text/plain"] = undefined;
          }
        }
        (targetJupyter.cells[cellNumber] as Jupyter.CodeCell).outputs = outputs;
      }

      elm.componentType = "JupyterComponent";
      elm.componentAttr = {
        ...elm.componentAttr,
        fileName,
        cellNumber,
        cell,
      };
      toDelete.push(next_item.id);

      // const code = cell.source.join("");
      // const output = cell.outputs.map((output) => output.text).join("\n");

      return elm;
    })
    .filter((p) => p !== null);
  self.reformedContents = list_buffer.filter(
    (p) => toDelete.includes(p.id) === false,
  );

  return self;
}
