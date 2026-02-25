import { getApi } from "../../../api"
import { CreateRegionData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

import CreateRegionForm from "./CreateRegionForm"

export default function CreateRegion() {
  const onSubmit = async (
    data: CreateRegionData,
  ): Promise<ActionPromiseResult> => {
    console.log("Creating region with data", data)
    getApi()
      .nodeStewardApi.createRegion(data)
      .then((_result) => {
        return actionSuccess()
      })
      .catch(actionFailure)
    return actionSuccess()
  }

  return <CreateRegionForm onSubmit={onSubmit} />
}
