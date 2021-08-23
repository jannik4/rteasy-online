export type State = StateEdit | StateRun;

export interface StateEdit {
  tag: "edit";
  sourceCode: string;
}

export interface StateRun {
  tag: "run";
  sourceCode: string;
}
