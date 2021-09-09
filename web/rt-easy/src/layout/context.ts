import React from "react";
import FlexLayout, { Model, Node, DockLocation } from "flexlayout-react";

export class LayoutModel {
  model: Model;

  constructor(model: Model) {
    this.model = model;
  }

  getNodeById(id: string): Node | undefined {
    // "undefined | null" needed because return type of "getNodeById" is incorrect
    const tabNode: Node | undefined | null = this.model.getNodeById(id);
    return tabNode ?? undefined;
  }

  findOne(f: (n: Node) => boolean): Node | undefined {
    let node: Node | undefined = undefined;
    this.model.visitNodes((n, _l) => {
      if (f(n)) node = n;
    });
    return node;
  }

  findAll(f: (n: Node) => boolean): Node[] {
    let nodes: Node[] = [];
    this.model.visitNodes((n, _l) => {
      if (f(n)) nodes.push(n);
    });
    return nodes;
  }

  /**
   *
   * @param tabNodeId
   * @returns true if:
   * - node exists and
   * - is a tab and
   * - was selected
   */
  selectTab(tabNodeId: string): boolean {
    const node = this.getNodeById(tabNodeId);
    if (node === undefined) return false;
    if (node.getType() !== "tab") return false;

    this.model.doAction(FlexLayout.Actions.selectTab(tabNodeId));
    return true;
  }

  createTab(
    id: string,
    name: string,
    component: string,
    toNodeId: string,
    location: DockLocation
  ) {
    this.model.doAction(
      FlexLayout.Actions.addNode(
        {
          type: "tab",
          enableClose: true,
          component,
          name,
          id,
        },
        toNodeId,
        location,
        -1
      )
    );
  }
}

export const LayoutModelContext = React.createContext<LayoutModel>(
  new LayoutModel(new Model())
);
