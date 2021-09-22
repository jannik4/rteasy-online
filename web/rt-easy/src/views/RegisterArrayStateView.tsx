import React, { useState, useContext, useMemo } from "react";
import { HTMLTable, Text, InputGroup, Button } from "@blueprintjs/core";

import { GlobalContext } from "../global/context";

interface Props {
  registerArray: string;
}

interface InputValue {
  key: string;
  value: string;
  valueNext: string | null;
  onChanged: (value: string) => void;
}

const RegisterArrayStateView: React.FC<Props> = ({ registerArray }) => {
  // Context and state
  const globalModel = useContext(GlobalContext);
  const [pageNr, setPageNr] = useState(1);
  const [focused, setFocused] = useState<InputValue | null>(null);

  // Page count
  const pageCount = useMemo(() => {
    if (globalModel.tag === "Edit") return "";
    return globalModel.registerArrayPageCount(registerArray);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [globalModel.tag, registerArray]);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const inputValue = (inputValue: InputValue) => {
    let value: string;
    if (focused?.key === inputValue.key) {
      value = focused.value;
    } else {
      value = inputValue.value;
      if (inputValue.valueNext !== null) {
        value += ` \u2794 ${inputValue.valueNext}`;
      }
    }

    return (
      <InputGroup
        small
        value={value}
        onChange={(e) => setFocused({ ...inputValue, value: e.target.value })}
        onFocus={() => setFocused(inputValue)}
        onBlur={() => {
          if (focused?.value !== inputValue.value) {
            focused?.onChanged(focused.value);
          }
          setFocused(null);
        }}
        onKeyDown={(e) => {
          if (e.key === "Enter") e.currentTarget.blur();
        }}
      />
    );
  };

  return (
    <div style={{ height: "100%", padding: "0 8px" /*, overflow: "hidden"*/ }}>
      <div style={{ height: 16 }} />

      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <Button
          icon="arrow-left"
          onClick={() => {
            if (pageNr > 1) setPageNr(pageNr - 1);
          }}
          minimal
          small
        />
        <Text>
          {pageNr} / {pageCount}
        </Text>
        <Button
          icon="arrow-right"
          onClick={() => {
            if (pageNr < pageCount) setPageNr(pageNr + 1);
          }}
          minimal
          small
        />
      </div>

      <div style={{ height: 16 }} />

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Index</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {globalModel
            .registerArrayPage(registerArray, pageNr, globalModel.base)
            .map((row) => (
              <tr key={row.idx}>
                <td>{row.idx}</td>
                <td>
                  {inputValue({
                    key: row.idx.toString(),
                    value: row.value,
                    valueNext: null,
                    onChanged: (value) =>
                      globalModel.writeIntoRegisterArray(
                        registerArray,
                        row.idx,
                        value,
                        globalModel.base
                      ),
                  })}
                </td>
              </tr>
            ))}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default RegisterArrayStateView;
