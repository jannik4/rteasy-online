import { useCallback } from "react";

export type Accept = "*" | string[];
export interface Options {
  accept?: "*" | string[];
  onChange: (name: string, content: string) => void;
}
export type FilePicker = () => void;

export function useFilePicker({ accept, onChange }: Options): FilePicker {
  const openFilePicker = useCallback(() => {
    const inputElement = document.createElement("input");

    inputElement.type = "file";
    inputElement.multiple = false;
    inputElement.accept =
      accept === undefined || accept === "*" ? "*" : accept.join(",");

    inputElement.addEventListener("change", () => {
      const files = inputElement.files ? Array.from(inputElement.files) : [];
      if (files.length === 1) {
        const file = files[0];
        const reader = new FileReader();
        reader.onload = () => {
          const content = reader.result as string;
          onChange(file.name, content);
        };
        reader.readAsText(file);
      }
    });

    inputElement.click();
  }, [accept, onChange]);

  return openFilePicker;
}
