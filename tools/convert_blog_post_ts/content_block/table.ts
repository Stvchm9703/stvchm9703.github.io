import type {
  ContentBlock,
  ContentBlockMap,
  FuncContentBlockResolver,
} from "./index";
import type { DataMap, TextMark } from "../common";

export const resolveTableComponent = (
  self: ContentBlock,
  refs: ContentBlockMap,
): [ContentBlock, string[]] => {
  // const tColumnSetting =
  //   refs
  //     .get(self.childrenIds[0])
  //     ?.childrenIds?.map((id) => refs.get(id))
  //     .filter((e) => e !== null && e !== undefined) ?? [];

  // const tColumn = tColumnSetting.map((column) => {
  //   return {
  //     id: column.id,
  //     ...column.fields,
  //   };
  // });
  let toDel: string[] = [];

  const tData =
    refs
      .get(self.childrenIds[1])
      ?.childrenIds?.map((id) => refs.get(id))
      .filter((e) => e !== null && e !== undefined) ?? [];
  const rowData = tData
    ?.map((r, i) => {
      if (!r) return null;
      const [rd, dlList] = resolveTableRowComponent(r, refs, i);
      toDel.push(...dlList);
      return rd;
    })
    .filter((r) => r !== null);

  self.componentAttr = {
    ...self.componentAttr,
    tableData: {
      rowData,
    },
  };

  // self.childrenIds = undefined;
  // self.checked = true;
  return [self, toDel];
};

interface TableRowData {
  id: string;
  rowIdx: number;
  cells: TableCellData[];
  isHeader: boolean;
  [key: string]: any;
}
interface TableCellData {
  id: string;
  cellIdx: number;
  content: string;
  marks: TextMark[];
  [key: string]: any;
}

function resolveTableRowComponent(
  self: ContentBlock,
  refs: ContentBlockMap,
  idx: number,
): [TableRowData, string[]] {
  if (self.componentType !== "TableRow") return null;

  let toDel = [self.id];
  const rowDt: TableRowData = {
    id: self.id,
    rowIdx: idx,
    cells: [],
    isHeader: self.componentAttr?.isHeader ?? false,
    style: self?.style,
    ...self.fields,
    ...self.componentAttr,
  };

  for (const [cellIdx, id] of self.childrenIds.entries()) {
    const child = refs.get(id);
    // if (child === undefined) return;
    // if (child.componentType !== "Text") return;
    const cellDt: TableCellData = {
      id,
      cellIdx,
      content: child?.componentAttr["text"] ?? "",
      marks: child?.componentAttr["marks"]["marks"] ?? [],
      style: child?.style,
      ...child?.fields,
      ...child?.componentAttr,
    };
    rowDt.cells.push(cellDt);
    toDel.push(id);
  }

  return [rowDt, toDel];
}
