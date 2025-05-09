import { exportForMetaData, type FileObject } from "../file_object";
import { type Page } from "../page/index";
import type { ContentBlock, ContentBlockMap } from "./index";

export function resolveFileComponent(self: Page, extFileList: FileObject[]) {
  let included: FileObject[] = [];
  for (const block of self.reformedContents) {
    if (block.componentType === "File") {
      const targetFile = extFileList.find(
        (p) => p.id === block.componentAttr["targetObjectId"],
      );
      if (targetFile) {
        block.componentAttr = {
          ...block.componentAttr,
          fileUrl: targetFile.fileUrl,
        };
        included.push(targetFile);
      }
    }
  }
  self.meta = exportForMetaData(included);
  return self;
}

export const resolveFileComponentV2 = (
  self: ContentBlock,
  refs: ContentBlockMap,
  extFileList: FileObject[],
): [ContentBlock, string[]] => {
  const toDel: string[] = [];
  const targetFile = extFileList.find(
    (p) => p.id === self.componentAttr["targetObjectId"],
  );

  if (targetFile) {
    self.componentAttr["fileUrl"] = targetFile.fileUrl;
    included.push(targetFile);
  }

  return [self, toDel];
};
