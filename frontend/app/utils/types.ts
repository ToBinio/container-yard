export type Project = {
  name: string;
  status: "running" | "stopped";
};

export type ProjectDetails = Project & {
  files: string[];
};
