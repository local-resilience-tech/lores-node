import { BaseApi } from "../shared"
import { ApiResult } from "../shared/types"
import { NodeIdentity, RegionDetails } from "./types"

export interface UpdateNodeData {
  name: string
  public_ipv4: string
}

export interface PostStatusData {
  text: string
  state?: "active" | "inactive" | "maintenance" | "development"
}

export default class ThisNodeApi extends BaseApi {
  show(): Promise<ApiResult<NodeIdentity | null, any>> {
    return this.apiCall("this_node")
  }

  create({ name }: { name: string }): Promise<ApiResult<NodeIdentity, any>> {
    return this.apiCall("this_node/create", "POST", { name })
  }

  update(data: UpdateNodeData): Promise<ApiResult<NodeIdentity, any>> {
    return this.apiCall(`this_node/`, "PATCH", data)
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

  postStatus(data: PostStatusData) {
    return this.apiCall("this_node/status", "POST", data)
  }
}
