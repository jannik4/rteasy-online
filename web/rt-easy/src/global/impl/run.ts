import React from "react";
import { editor } from "monaco-editor";

import { RtEasy } from "../../wasm";
import { GlobalModelRun, BaseInherit } from "../context";
import { model as modelCommon } from "./common";
import { State, StateRun } from "../state";

export function model(
  _rtEasy: RtEasy,
  state: StateRun,
  setState: React.Dispatch<React.SetStateAction<State>>,
  editorModel: editor.IModel,
  inheritBasesStorage: React.MutableRefObject<Map<string, BaseInherit>>
): GlobalModelRun {
  const goToEditMode = () => {
    state.simulator.free();
    setState({
      tag: "Edit",
      editor: state.editor,
      base: state.base,
      clockRate: state.clockRate,
    });
  };

  return {
    ...modelCommon(state, setState, editorModel, goToEditMode),
    tag: "Run",
    goToEditMode,

    simulator: state.simulator,
    toggleRun: () => {
      const intervalMsOrMax =
        state.clockRate === "Max" ? "Max" : 1000 / state.clockRate;
      state.simulator.toggleRun(intervalMsOrMax);
    },
    inheritBasesStorage,
  };
}
