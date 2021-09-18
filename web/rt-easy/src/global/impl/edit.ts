import React from "react";

import { RtEasy } from "../../wasm";
import { GlobalModelEdit } from "../context";
import { State, StateEdit } from "../state";

export function model(
  rtEasy: RtEasy,
  state: StateEdit,
  setState: React.Dispatch<React.SetStateAction<State>>
): GlobalModelEdit {
  const build = () => {
    try {
      const simulator = rtEasy.build(state.sourceCode);
      setState({
        tag: "Run",
        sourceCode: state.sourceCode,
        base: state.base,
        simulator,
        simState: null,
        timerId: null,
      });
    } catch (e) {
      alert(e);
    }
  };

  return {
    tag: "Edit",
    sourceCode: state.sourceCode,
    toggleMode: () => build(),
    base: state.base,
    setBase: (base) => setState({ ...state, base }),
    log: state.log,
    setSourceCode: (sourceCode) => {
      let log: string;
      try {
        rtEasy.check(sourceCode);
        log = "--- ok ---";
      } catch (e) {
        log = e as string;
      }

      localStorage.setItem("source-code", sourceCode);
      setState({ ...state, sourceCode, log });
    },
    build,
  };
}
