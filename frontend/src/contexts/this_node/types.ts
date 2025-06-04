export type NodeIdentity = {
  id: string
  name: string
}

export type NodeDetails = {
  id: string
  name: string
  public_ipv4: string
}

export type NodeDetailsWithStatus = NodeDetails & {
  status_text?: string | null
  state?: string | null
}

export type RegionDetails = {
  id: string
  name: string
  description: string
}
