import { reactive } from "vue";

export const copyStore = reactive({
  sourcePath: "",
  destinationPath: "",
  dryRun: {
    command: "",
    output: "",
  },
});
