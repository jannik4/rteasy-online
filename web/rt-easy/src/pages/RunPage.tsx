import React from "react";

import Page from "./Page";

interface Props {}

const RunPage: React.FC<Props> = () => {
  return <Page id="run" defaultLayout={defaultLayout} />;
};

export default RunPage;

const defaultLayout = {
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
          name: "State",
          component: "state",
        },
      ],
    },
  ],
};
