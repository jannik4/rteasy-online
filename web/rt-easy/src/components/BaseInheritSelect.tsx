import React from "react";
import { HTMLSelect } from "@blueprintjs/core";

import {
  BaseInherit,
  baseInheritValues,
  isBaseInherit,
} from "../global/context";

interface Props {
  value: BaseInherit;
  onChange: (baseInherit: BaseInherit) => void;
}

const BaseInheritSelect: React.FC<Props> = ({ value, onChange }) => {
  return (
    <HTMLSelect
      value={value}
      onChange={(e) => {
        if (!isBaseInherit(e.target.value)) throw new Error("invalid value");
        onChange(e.target.value);
      }}
      minimal
    >
      {baseInheritValues.map((baseInherit) => (
        <option key={baseInherit} value={baseInherit}>
          {baseInherit}
        </option>
      ))}
    </HTMLSelect>
  );
};

export default BaseInheritSelect;
