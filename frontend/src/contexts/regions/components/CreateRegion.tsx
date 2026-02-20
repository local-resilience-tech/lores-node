import { ActionPromiseResult, actionSuccess } from "../../../components"
import { useAppDispatch } from "../../../store"
import CreateRegionForm, { CreateRegionData } from "./CreateRegionForm"

export default function CreateRegion() {
  const onSubmit = async (
    data: CreateRegionData,
  ): Promise<ActionPromiseResult> => {
    console.log("Creating region with data", data)
    return actionSuccess()
  }

  return <CreateRegionForm onSubmit={onSubmit} />
}
