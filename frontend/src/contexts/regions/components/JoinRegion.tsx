import { ActionPromiseResult } from "../../../components"

import JoinRegionForm, { JoinRegionData } from "./JoinRegionForm"

export default function JoinRegion() {
  const onSubmit = async (
    data: JoinRegionData,
  ): Promise<ActionPromiseResult> => {
    console.log("Submitting join region form with data:", data)
  }

  return <JoinRegionForm onSubmit={onSubmit} />
}
