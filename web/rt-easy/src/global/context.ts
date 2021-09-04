import React from "react";
import { GlobalModel } from "./model";

export const GlobalContext = React.createContext<GlobalModel>({
  tag: "Edit",
  sourceCode: "",
  base: "DEC",
  setBase: () => {},
  log: "",
  setSourceCode: () => {},
  build: () => {},
});
