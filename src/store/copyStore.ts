import { reactive } from "vue";

export const copyStore = reactive({
  sourcePath: "",
  destinationPath: "",
  dryRun: {
    command: "",
    output: "",
  },
  buildCopyCommand() {
    return 'rclone copy "{sourcePath}" "{destinationPath}" --update --progress --dry-run'
      .replace("{sourcePath}", this.sourcePath)
      .replace("{destinationPath}", this.destinationPath);
  },
});
