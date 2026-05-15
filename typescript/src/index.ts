import { getConfig } from "./config"
import { getOptions } from "./opts"

const opts = getOptions()
const config = getConfig(opts)

console.log(config)
