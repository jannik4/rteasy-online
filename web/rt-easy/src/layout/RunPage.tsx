import React from "react";
import { IJsonModel } from "flexlayout-react";

import Page from "./Page";
import * as consts from "./consts";

interface Props {}

const RunPage: React.FC<Props> = () => {
  return <Page id="run" defaultModel={defaultModel} />;
};

export default RunPage;

const defaultModel: IJsonModel = {
  global: {
    tabEnableClose: false,
    tabEnableRename: false,
    tabSetEnableMaximize: false,
  },
  borders: [],
  layout: {
    type: "row",
    weight: 100,
    children: [
      {
        type: "tabset",
        weight: 50,
        selected: 0,
        children: [
          {
            type: "tab",
            name: "Code",
            component: "editor",
          },
        ],
      },
      {
        type: "tabset",
        weight: 50,
        selected: 0,
        children: [
          {
            id: consts.ID_TAB_STATE,
            type: "tab",
            name: "State",
            component: "state",
          },
          {
            type: "tab",
            name: "Signals",
            component: "signals",
          },
        ],
      },
    ],
  },
};
