import { defineStore } from "pinia";

export const useProjectsStore = defineStore("projects", {
  state: (): { data: ProjectDetails[] } => ({
    data: [],
  }),
  actions: {
    async fetch() {
      const { $api } = useNuxtApp();

      const data = await $api<Project[]>("/projects").catch(async (err) => {
        //todo - show in toast
        console.warn(err);
        return [];
      });

      const newData = data.map((project) => {
        return {
          name: project.name,
          status: project.status,
          files: [],
        } as ProjectDetails;
      });

      this.data = newData;
    },

    async fetchProject(name: string) {
      const { $api } = useNuxtApp();

      try {
        const data = await $api<ProjectDetails>(`/projects/${name}`);
        this.setProjectDetails(data);
      } catch (e) {
        console.error(e);
      }
    },

    setProjectDetails(projectDetails: ProjectDetails) {
      const index = this.data.findIndex(
        (project) => project.name == projectDetails.name,
      );
      this.data[index] = projectDetails;
    },
  },
  getters: {
    getByName(state) {
      return (name: string) => state.data.find((todo) => todo.name === name);
    },
  },
});
