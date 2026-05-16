import { getConfig } from "./config"
import { getOptions } from "./opts"
import { Projector } from "./projector"

const opts = getOptions()
const config = getConfig(opts)
const projector = Projector.fromConfig(config)

switch (config.operation) {
  case "print":
    if (config.args.length === 0) {
      console.log(JSON.stringify(projector.getValueAll()))
    } else {
      const value = projector.getValue(config.args[0]!)
      if (value) {
        console.log(value)
      }
    }
    break
  case "add":
    projector.setValue(config.args[0]!, config.args[1]!)
    projector.save()
    break
  case "remove":
    projector.removeValue(config.args[0]!)
    projector.save()
    break
}
