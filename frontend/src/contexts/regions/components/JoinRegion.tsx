import { BootstrapNodeData } from "../../../api/Api"
import { ActionPromiseResult } from "../../../components"
import { useAppDispatch } from "../../../store"

import JoinRegionForm from "./JoinRegionForm"

export default function JoinRegion() {
  const onSubmit = async (
    data: BootstrapNodeData,
  ): Promise<ActionPromiseResult> => {
    console.log("Submitting join region form with data:", data)
  }

  return <JoinRegionForm onSubmit={onSubmit} />
}
