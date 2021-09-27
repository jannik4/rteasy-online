import React from "react";
import { InputGroup } from "@blueprintjs/core";

interface Props {
  focused: Focused | null;
  setFocused: (focused: Focused | null) => void;
  name: string;
  value: string;
  valueNext: string | null;
  onChanged: (value: string) => void;
}

export interface Focused {
  name: string;
  value: string;
}

const InputValue: React.FC<Props> = ({
  focused,
  setFocused,
  name,
  value,
  valueNext,
  onChanged,
}) => {
  let valueDisplay: string;

  if (focused?.name === name) {
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
      onChange={(e) => setFocused({ name, value: e.target.value })}
      onFocus={() => setFocused({ name, value })}
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
