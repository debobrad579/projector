import path from "path"
import { Config } from "./config"
import fs from "fs"

export type Data = {
  projector: {
    [key: string]: {
      [key: string]: string
    }
  }
}

const defaultData: Data = {
  projector: {},
}

export class Projector {
  constructor(
    private config: Config,
    private data: Data,
  ) {}

  getValueAll() {
    let curr = this.config.pwd
    let prev = ""
    const paths = []

    do {
      prev = curr
      paths.push(curr)
      curr = path.dirname(curr)
    } while (curr != prev)

    return paths.reverse().reduce((acc, dirPath) => {
      const value = this.data.projector[dirPath]

      if (value != null) {
        Object.assign(acc, value)
      }

      return acc
    }, {})
  }

  getValue(key: string) {
    let curr = this.config.pwd
    let prev = ""

    do {
      const value = this.data.projector[curr]?.[key]
      if (value != null) {
        return value
      }
      prev = curr
      curr = path.dirname(curr)
    } while (curr != prev)
  }

  setValue(key: string, value: string) {
    if (this.data.projector[this.config.pwd] == null) {
      this.data.projector[this.config.pwd] = {}
    }

    this.data.projector[this.config.pwd]![key] = value
  }

  removeValue(key: string) {
    if (this.data.projector[this.config.pwd]?.[key] != null) {
      delete this.data.projector[this.config.pwd]![key]
    }
  }

  save() {
    const configPath = path.dirname(this.config.config)
    if (!fs.existsSync(configPath)) {
      fs.mkdirSync(configPath, { recursive: true })
    }
    fs.writeFileSync(this.config.config, JSON.stringify(this.data))
  }

  static fromConfig(config: Config): Projector {
    if (fs.existsSync(config.config)) {
      try {
        return new Projector(
          config,
          JSON.parse(fs.readFileSync(config.config).toString()),
        )
      } catch (_) {
        return new Projector(config, defaultData)
      }
    }

    return new Projector(config, defaultData)
  }
}
