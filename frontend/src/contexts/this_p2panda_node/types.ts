export type NodeAddr = {
  node_id: string
  info: {
    relay_url: string
    direct_addresses: string[]
  }
}

export type BootstrapPeer = {
  node_id: string
}
