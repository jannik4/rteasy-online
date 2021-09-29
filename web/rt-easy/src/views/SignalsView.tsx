import React, { useMemo, useContext } from "react";
import { HTMLTable } from "@blueprintjs/core";

import { GlobalContext, Signals } from "../global/context";

interface Props {}

const StateView: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);
  const signals = useMemo<Signals>(() => {
    return globalModel.tag === "Run"
      ? globalModel.signals()
      : { conditionSignals: [], controlSignals: [] };

    // Signals are only dependent on tag and source code
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [globalModel.tag, globalModel.editorModel.getValue()]);

  const headerRow = (title: string) => (
    <tr style={{ backgroundColor: "#f2f2f2" }}>
      <td colSpan={2}>{title}:</td>
    </tr>
  );
  const dividerRow = (
    <tr key="dividerRow">
      <td colSpan={2}>&nbsp;</td>
    </tr>
  );

  return (
    <div style={{ padding: "16px 8px" /*, overflow: "hidden"*/ }}>
      <HTMLTable width="100%" bordered condensed>
        <thead></thead>
        <tbody>
          {headerRow("Condition Signals")}
          {signals.conditionSignals.map((value, idx) => (
            <tr key={idx}>
              <td>{"k" + idx}</td>
              <td>{value}</td>
            </tr>
          ))}
          {dividerRow}

          {headerRow("Control Signals")}
          {signals.controlSignals.map((value, idx) => (
            <tr key={idx}>
              <td>{"c" + idx}</td>
              <td>{value}</td>
            </tr>
          ))}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default StateView;
