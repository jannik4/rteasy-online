import React from "react";

import { RtEasy } from "../../wasm";
import { GlobalModelEdit } from "../context";
import { State, StateEdit } from "../state";
import { Storage } from "../../storage";

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
    sourceCode: state.sourceCode,
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
    setSourceCode: (sourceCode) => {
      Storage.setSourceCode(sourceCode);
      setState({ ...state, sourceCode });
    },
    build,
  };
}
