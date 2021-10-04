import React, { useState, useContext, useMemo } from "react";
import { HTMLTable, Text, Button } from "@blueprintjs/core";

import { InputValue, Focused, BaseInheritSelect } from "../components";
import { GlobalContext, BaseInherit } from "../global/context";

interface Props {
  registerArray: string;
}

const RegisterArrayStateView: React.FC<Props> = ({ registerArray }) => {
  // Context and state
  const globalModel = useContext(GlobalContext);
  const [pageNr, setPageNr] = useState(1);
  const [focused, setFocused] = useState<Focused | null>(null);
  const [baseInherit, setBaseInherit] = useState<BaseInherit>(() => {
    if (globalModel.tag === "Run") {
      return (
        globalModel.inheritBasesStorage.current.get(registerArray) ?? "Inherit"
      );
    }
    return "Inherit";
  });
  const base = baseInherit === "Inherit" ? globalModel.base : baseInherit;

  // Page count
  const pageCount = useMemo(() => {
    if (globalModel.tag === "Edit") return "";
    return globalModel.simulator.registerArrayPageCount(registerArray);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [globalModel.tag, registerArray]);

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  const valueNext = globalModel.simulator.registerArrayValueNext(
    registerArray,
    base
  );

  return (
    <div style={{ height: "100%", padding: "0 8px" /*, overflow: "hidden"*/ }}>
      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          alignItems: "center",
        }}
      >
        <BaseInheritSelect
          value={baseInherit}
          onChange={(baseInherit) => {
            if (globalModel.tag === "Run") {
              globalModel.inheritBasesStorage.current.set(
                registerArray,
                baseInherit
              );
            }
            setBaseInherit(baseInherit);
          }}
        />
      </div>

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
          {globalModel.simulator
            .registerArrayPage(registerArray, pageNr, base)
            .map((row) => (
              <tr key={row.idx}>
                <td>{row.idx}</td>
                <td>
                  <InputValue
                    focused={focused}
                    setFocused={setFocused}
                    inputKey={row.idx.toString()}
                    highlight={
                      globalModel.simulator
                        .getSimState()
                        ?.changed?.registerArrays.get(registerArray) === row.idx
                    }
                    value={() => row.value}
                    valueNext={() =>
                      row.idx === valueNext?.idx ? valueNext.value : null
                    }
                    onChanged={(value: string) =>
                      globalModel.simulator.writeRegisterArray(
                        registerArray,
                        row.idx,
                        value,
                        base
                      )
                    }
                  />
                </td>
              </tr>
            ))}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default RegisterArrayStateView;
