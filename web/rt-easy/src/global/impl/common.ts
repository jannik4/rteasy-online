import React from "react";
import { editor } from "monaco-editor";

import { GlobalModelCommon } from "../context";
import { State, StateEdit, StateRun } from "../state";
import { Storage } from "../../storage";

export function model(
  state: StateEdit | StateRun,
  setState: React.Dispatch<React.SetStateAction<State>>,
  editorModel: editor.IModel,
  toggleMode: () => void
): GlobalModelCommon {
  return {
    editor: state.editor,
    setEditor: (editor) => {
      editor?.setModel(editorModel);
      setState({ ...state, editor });
    },
    editorModel,
    toggleMode,
    base: state.base,
    setBase: (base) => {
      Storage.setBase(base);
      setState({ ...state, base });
    },
    clockRate: state.clockRate,
    setClockRate: (clockRate) => {
      Storage.setClockRate(clockRate);
      setState({ ...state, clockRate });
    },
  };
}
