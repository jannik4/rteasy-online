import React from "react";

import { Span } from "../wasm";

export type GlobalModel = GlobalModelEdit | GlobalModelRun;

export interface GlobalModelEdit {
  tag: "Edit";
  sourceCode: string;
  log: string;
  setSourceCode: (sourceCode: string) => void;
  build: () => void;
}

export interface GlobalModelRun {
  tag: "Run";
  sourceCode: string;
  goToEditMode: () => void;
  isFinished: () => boolean;
  state: () => string;
  step: () => void;
  currSpan: () => Span | null;

  runStop: () => void;
  isRunning: () => boolean;
}

export const GlobalContext = React.createContext<GlobalModel>({
  tag: "Edit",
  sourceCode: "",
  log: "",
  setSourceCode: () => {},
  build: () => {},
});
