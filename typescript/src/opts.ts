import cli from "command-line-args"

export default function getOptions() {
  return cli([
    {
      name: "args",
      defaultOption: true,
      multiple: true,
      type: String,
    },
    {
      name: "config",
      alias: "c",
      type: String,
    },
    {
      name: "pwd",
      alias: "p",
      type: String,
    },
  ])
}
