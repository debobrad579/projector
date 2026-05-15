import path from "path"
import { Opts } from "./opts"

export type Operation = "print" | "add" | "remove"
export type Config = {
  args: string[]
  operation: Operation
  config: string
  pwd: string
}

function getConfigLocation(opts: Opts) {
  if (opts.config != null) {
    return opts.config
  }

  return path.join(
    process.env["XDG_CONFIG_HOME"] ??
      path.join(process.env["HOME"]!, ".projector.json"),
    "projector",
    "projector.json",
  )
}

function getOperation(opts: Opts): Operation {
  if (opts.args[0] === "add" || opts.args[0] === "remove") {
    return opts.args[0]
  } else {
    return "print"
  }
}

function getArgs(opts: Opts): string[] {
  if (opts.args.length < 0) {
    return []
  }

  const operation = getOperation(opts)
  switch (operation) {
    case "add":
      if (opts.args.length !== 3) {
        throw new Error(`expected 2 arguments but got ${opts.args.length - 1}`)
      }
      return opts.args.slice(1)
    case "remove":
      if (opts.args.length !== 2) {
        throw new Error(`expected 1 argument but got ${opts.args.length - 1}`)
      }
      return opts.args.slice(1)
    case "print":
      if (opts.args.length > 1) {
        throw new Error(`expected 0 or 1 arguments but got ${opts.args.length}`)
      }
      return opts.args
  }
}

export function getConfig(opts: Opts): Config {
  return {
    pwd: opts.pwd ?? process.cwd(),
    config: getConfigLocation(opts),
    args: getArgs(opts),
    operation: getOperation(opts),
  }
}
