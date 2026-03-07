import { useNavigate } from "react-router-dom"
import { getApi } from "../../../api"
import { BootstrapNodeRequest, CreateRegionData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

import AddBootstrapNodeForm from "./AddBootstrapNodeForm"

interface AddBootstrapNodeProps {
  onSuccess?: () => void
}

export default function AddBootstrapNode({ onSuccess }: AddBootstrapNodeProps) {
  const onSubmit = async (
    data: BootstrapNodeRequest,
  ): Promise<ActionPromiseResult> => {
    console.log("Adding bootstrap node with data", data)
    return getApi()
      .nodeStewardApi.addBootstrapNode(data)
      .then((_result) => {
        if (onSuccess) {
          onSuccess()
        }
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return <AddBootstrapNodeForm onSubmit={onSubmit} />
}
