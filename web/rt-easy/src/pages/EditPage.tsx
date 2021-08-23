import React from "react";

import Page from "./Page";

interface Props {}

const EditPage: React.FC<Props> = () => {
  return <Page id="edit" defaultModel={defaultModel} />;
};

export default EditPage;

const defaultModel = {
  global: {
    tabEnableClose: false,
    tabEnableRename: false,
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
            type: "tab",
            name: "Log",
            component: "log",
          },
        ],
      },
    ],
  },
};
