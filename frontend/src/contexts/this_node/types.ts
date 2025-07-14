import { NodeDetails } from "../../api/Api"

export type NodeDetailsWithStatus = NodeDetails & {
  status_text?: string | null
  state?: string | null
}

export type RegionDetails = {
  id: string
  name: string
  description: string
}
