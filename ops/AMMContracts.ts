import { ScrtEnsemble } from '@fadroma/scrt-ops'
import { abs } from './lib/index.js'

export default class SiennaSwap extends ScrtEnsemble {

  workspace = abs()

  prefix = `${new Date().toISOString()} `

  contracts = {
    FACTORY: {
      crate: 'factory'
    },
    SNIP20: {
      crate: 'amm-snip20'
    },
    EXCHANGE: {
      crate: 'exchange'
    },
    LP_TOKEN: {
      crate: 'lp-token'
    },
    IDO: {
      crate: 'ido'
    }
  }

}
