import React, { useState, useContext } from "react";
import { HTMLTable, HTMLSelect, Text, H5 } from "@blueprintjs/core";

import { GlobalContext } from "../context";

interface Props {}

const StateView: React.FC<Props> = () => {
  const [base, setBase] = useState("DEC");
  const globalModel = useContext(GlobalContext);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  return (
    <div style={{ height: "100%" /*, overflow: "hidden"*/ }}>
      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <HTMLSelect
          value={base}
          onChange={(e) => setBase(e.target.value)}
          minimal
        >
          <option value="BIN">BIN</option>
          <option value="DEC">DEC</option>
          <option value="HEX">HEX</option>
        </HTMLSelect>
        <Text>Cycle count: {globalModel.cycleCount()}</Text>
      </div>

      <div style={{ height: 16 }} />
      <H5>Registers</H5>

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Identifier</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {
            // TODO: Only get names one time!!! (on simulator create)
            globalModel.registers().map((register) => (
              <tr key={register}>
                <td>{register}</td>
                <td>{globalModel.registerValue(register, base)}</td>
              </tr>
            ))
          }
        </tbody>
      </HTMLTable>

      <div style={{ height: 16 }} />
      <H5>Buses</H5>

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Identifier</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {
            // TODO: Only get names one time!!! (on simulator create)
            globalModel.buses().map((bus) => (
              <tr key={bus}>
                <td>{bus}</td>
                <td>{globalModel.busValue(bus, base)}</td>
              </tr>
            ))
          }
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default StateView;
