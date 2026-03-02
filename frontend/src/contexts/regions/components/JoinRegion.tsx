import { useNavigate } from "react-router-dom"
import { getApi } from "../../../api"
import { JoinRegionRequestData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

import JoinRegionForm from "./JoinRegionForm"

export default function JoinRegion() {
  const navigate = useNavigate()

  const onSubmit = async (
    data: JoinRegionRequestData,
  ): Promise<ActionPromiseResult> => {
    console.log("Submitting join region form with data:", data)
    return getApi()
      .nodeStewardApi.joinRegion(data)
      .then((_result) => {
        navigate(`/regions`)
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return <JoinRegionForm onSubmit={onSubmit} />
}
