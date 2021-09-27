import React from "react";
import { InputGroup } from "@blueprintjs/core";

interface Props {
  focused: Focused | null;
  setFocused: (focused: Focused | null) => void;
  inputKey: string;
  value: string;
  valueNext: string | null;
  onChanged: (value: string) => void;
}

export interface Focused {
  inputKey: string;
  value: string;
}

const InputValue: React.FC<Props> = ({
  focused,
  setFocused,
  inputKey,
  value,
  valueNext,
  onChanged,
}) => {
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
    <InputGroup
      small
      value={valueDisplay}
      onChange={(e) => setFocused({ inputKey, value: e.target.value })}
      onFocus={() => setFocused({ inputKey, value })}
      onBlur={() => {
        if (focused !== null) {
          if (focused.value !== value) onChanged(focused.value);
          setFocused(null);
        }
      }}
      onKeyDown={(e) => {
        if (e.key === "Enter") e.currentTarget.blur();
      }}
    />
  );
};

export default InputValue;
