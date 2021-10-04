import React, { useContext } from "react";
import { HTMLTable } from "@blueprintjs/core";

import { GlobalContext } from "../global/context";

interface Props {}

const StateView: React.FC<Props> = () => {
  const globalModel = useContext(GlobalContext);
  const signals =
    globalModel.tag === "Run"
      ? globalModel.simulator.getSignals()
      : { conditionSignals: [], controlSignals: [] };

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
              <td style={{ fontFamily: "monospace" }}>{value}</td>
            </tr>
          ))}
          {dividerRow}

          {headerRow("Control Signals")}
          {signals.controlSignals.map((value, idx) => (
            <tr key={idx}>
              <td>{"c" + idx}</td>
              <td style={{ fontFamily: "monospace" }}>{value}</td>
            </tr>
          ))}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default StateView;
