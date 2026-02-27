import { useNavigate } from "react-router-dom"
import { getApi } from "../../../api"
import { CreateRegionData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

import CreateRegionForm from "./CreateRegionForm"

export default function CreateRegion() {
  const navigate = useNavigate()

  const onSubmit = async (
    data: CreateRegionData,
  ): Promise<ActionPromiseResult> => {
    console.log("Creating region with data", data)
    return getApi()
      .nodeStewardApi.createRegion(data)
      .then((_result) => {
        navigate(`/regions/${data.slug}`)
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return <CreateRegionForm onSubmit={onSubmit} />
}
