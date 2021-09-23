import React from "react";
import { editor } from "monaco-editor";

import { RtEasy } from "../../wasm";
import { GlobalModelEdit } from "../context";
import { model as modelCommon } from "./common";
import { State, StateEdit } from "../state";

export function model(
  rtEasy: RtEasy,
  state: StateEdit,
  setState: React.Dispatch<React.SetStateAction<State>>,
  editorRef: React.MutableRefObject<editor.IStandaloneCodeEditor | null>,
  editorModel: editor.IModel
): GlobalModelEdit {
  const build = () => {
    try {
      const simulator = rtEasy.build(editorModel.getValue());
      setState({
        tag: "Run",
        base: state.base,
        clockRate: state.clockRate,
        simulator,
        simState: null,
        timerId: null,
      });
    } catch (_e) {
      alert("Code has errors");
    }
  };

  return {
    ...modelCommon(state, setState, editorRef, editorModel, build),
    tag: "Edit",
    build,
  };
}
