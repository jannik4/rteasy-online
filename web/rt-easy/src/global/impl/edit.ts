import React from "react";
import { editor } from "monaco-editor";

import { RtEasy } from "../../wasm";
import { GlobalModelEdit } from "../context";
import { State, StateEdit } from "../state";
import { Storage } from "../../storage";

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
    tag: "Edit",
    editorRef,
    editorModel,
    toggleMode: () => build(),
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
    build,
  };
}
