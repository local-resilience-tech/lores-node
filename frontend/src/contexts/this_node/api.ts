import { BaseApi } from "../shared"
import { ApiResult } from "../shared/types"
import { RegionDetails } from "./types"

export interface PostStatusData {
  text: string
  state?: "active" | "inactive" | "maintenance" | "development"
}

export default class ThisNodeApi extends BaseApi {
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
