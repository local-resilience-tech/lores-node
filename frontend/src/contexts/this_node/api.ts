import { BaseApi } from "../shared"
import { ApiResult } from "../shared/types"
import { NodeIdentity, RegionDetails } from "./types"

export interface UpdateNodeData {
  name: string
  public_ipv4: string
}

export default class ThisNodeApi extends BaseApi {
  show(): Promise<ApiResult<NodeIdentity | null, any>> {
    return this.apiCall("this_node")
  }

  create({ name }: { name: string }): Promise<ApiResult<NodeIdentity, any>> {
    return this.apiCall("this_node/create", "POST", { name })
  }

  update({
    name,
    public_ipv4,
  }: UpdateNodeData): Promise<ApiResult<NodeIdentity, any>> {
    return this.apiCall(`this_node/`, "PATCH", {
      name,
      public_ipv4,
    })
  }

  showRegion(): Promise<ApiResult<RegionDetails, any>> {
    return this.apiCall("this_region")
  }

  createRegion(
    name: string,
    description: string,
  ): Promise<ApiResult<RegionDetails, any>> {
    return this.apiCall("this_region/create", "POST", {
      name,
      description,
    })
  }
}
