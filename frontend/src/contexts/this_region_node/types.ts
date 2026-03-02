import { RegionNodeDetails } from "../../api/Api"

export type RegionNodeDetailsWithStatus = RegionNodeDetails & {
  status_text?: string | null
  state?: string | null
}

export type RegionDetails = {
  id: string
  name: string
  description: string
}
