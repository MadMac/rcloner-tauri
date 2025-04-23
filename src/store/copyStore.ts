import { reactive } from "vue";

export const copyStore = reactive({
  sourcePath: "",
  destinationPath: "",
  sourceError: false,
  destinationError: false,
  dryRun: {
    command: "",
    output: "" as string,
    started: false,
    finished: false,
  },
  startRun: {
    command: "",
    output: "" as string,
    started: false,
    finished: false,
  },
  buildCopyDryRunCommand() {
    return 'rclone copy "{sourcePath}" "{destinationPath}" --update --progress --dry-run'
      .replace("{sourcePath}", this.sourcePath)
      .replace("{destinationPath}", this.destinationPath);
  },
  buildCopyRunCommand() {
    return 'rclone copy "{sourcePath}" "{destinationPath}" --update --progress'
      .replace("{sourcePath}", this.sourcePath)
      .replace("{destinationPath}", this.destinationPath);
  },
});
