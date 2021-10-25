import React, { useContext, useState } from "react";
import { InputGroup } from "@blueprintjs/core";

import { GlobalContext, BaseInherit } from "../global/context";
import { BaseInheritSelect } from "../components";
import { Base } from "../wasm";

interface PropsCommon {
  focused: Focused | null;
  setFocused: (focused: Focused | null) => void;
  inputKey: string;
  highlight: boolean;
}

interface Props extends PropsCommon {
  withBaseSelect?: false;
  value: () => string;
  valueNext: (() => string | null) | null;
  onChanged: (value: string) => void;
}

interface PropsWithBaseSelect extends PropsCommon {
  withBaseSelect: true;
  value: (base: Base) => string;
  valueNext: ((base: Base) => string | null) | null;
  onChanged: (value: string, base: Base) => void;
}

export interface Focused {
  inputKey: string;
  value: string;
}

const InputValue: React.FC<Props | PropsWithBaseSelect> = (props) => {
  const globalModel = useContext(GlobalContext);
  const [baseInherit, setBaseInherit] = useState<BaseInherit>(() => {
    if (globalModel.tag === "Run") {
      return (
        globalModel.inheritBasesStorage.current.get(props.inputKey) ?? "Inherit"
      );
    }
    return "Inherit";
  });
  const base = baseInherit === "Inherit" ? globalModel.base : baseInherit;

  let focused = props.focused;
  let setFocused = props.setFocused;
  let inputKey = props.inputKey;
  let value: string, valueNext: string | null;

  if (props.withBaseSelect) {
    value = props.value(base);
    valueNext = props.valueNext === null ? null : props.valueNext(base);
  } else {
    value = props.value();
    valueNext = props.valueNext === null ? null : props.valueNext();
  }

  let valueDisplay: string;
  if (focused?.inputKey === inputKey) {
    valueDisplay = focused.value;
  } else {
    valueDisplay = value;
    if (valueNext !== null) {
      valueDisplay += ` \u2794 ${valueNext}`;
    }
  }

  return (
    <div style={{ display: "flex", alignItems: "center" }}>
      <div style={{ flex: "1" }}>
        <InputGroup
          small
          style={{ backgroundColor: props.highlight ? "yellow" : undefined }}
          value={valueDisplay}
          onChange={(e) => setFocused({ inputKey, value: e.target.value })}
          onFocus={() => setFocused({ inputKey, value: value })}
          onBlur={() => {
            if (focused !== null) {
              if (focused.value !== value) {
                const valueTrimmed = focused.value.trim();
                if (props.withBaseSelect) {
                  props.onChanged(valueTrimmed, base);
                } else {
                  props.onChanged(valueTrimmed);
                }
              }
              setFocused(null);
            }
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter") e.currentTarget.blur();
          }}
        />
      </div>

      {props.withBaseSelect ? (
        <BaseInheritSelect
          value={baseInherit}
          onChange={(baseInherit) => {
            if (globalModel.tag === "Run") {
              globalModel.inheritBasesStorage.current.set(
                props.inputKey,
                baseInherit
              );
            }
            setBaseInherit(baseInherit);
          }}
        />
      ) : null}
    </div>
  );
};

export default InputValue;
