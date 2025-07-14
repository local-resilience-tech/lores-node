import { BaseApi } from "../shared"
import { ApiResult } from "../shared/types"
import { NodeDetailsWithStatus } from "../this_node"

export default class ThisRegionApi extends BaseApi {
  nodes(): Promise<ApiResult<NodeDetailsWithStatus[], any>> {
    return this.apiCall("this_region/nodes")
  }
}
