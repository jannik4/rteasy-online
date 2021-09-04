import React, { useState, useContext } from "react";
import { HTMLTable, HTMLSelect, Text, H5, InputGroup } from "@blueprintjs/core";

import { GlobalContext } from "../context";

interface Props {}

interface InputValue {
  key: string;
  value: string;
  onChanged: (value: string) => void;
}

const StateView: React.FC<Props> = () => {
  const [focused, setFocused] = useState<InputValue | null>(null);
  const globalModel = useContext(GlobalContext);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const inputValue = (inputValue: InputValue) => (
    <InputGroup
      small
      value={focused?.key === inputValue.key ? focused.value : inputValue.value}
      onChange={(e) => setFocused({ ...inputValue, value: e.target.value })}
      onFocus={() => setFocused(inputValue)}
      onBlur={() => {
        focused?.onChanged(focused.value);
        setFocused(null);
      }}
      onKeyDown={(e) => {
        if (e.key === "Enter") e.currentTarget.blur();
      }}
    />
  );

  return (
    <div style={{ height: "100%", padding: "0 8px" /*, overflow: "hidden"*/ }}>
      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <HTMLSelect
          value={globalModel.base}
          onChange={(e) => {
            if (
              e.target.value !== "BIN" &&
              e.target.value !== "DEC" &&
              e.target.value !== "HEX"
            )
              throw new Error("invalid value");
            globalModel.setBase(e.target.value);
          }}
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
                <td>
                  {inputValue({
                    key: register,
                    value: globalModel.registerValue(
                      register,
                      globalModel.base
                    ),
                    onChanged: (value) =>
                      console.log(`TODO: set ${register} = ${value}`),
                  })}
                </td>
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
                <td>
                  {inputValue({
                    key: bus,
                    value: globalModel.busValue(bus, globalModel.base),
                    onChanged: (value) =>
                      globalModel.writeIntoBus(bus, value, globalModel.base),
                  })}
                </td>
              </tr>
            ))
          }
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default StateView;
