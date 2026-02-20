import { getApi } from "../../../api"
import { BootstrapNodeData, Region } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"
import { useAppDispatch } from "../../../store"
import { regionLoaded } from "../../../store/region"
import CreateRegionForm from "./CreateRegionForm"

export default function CreateRegion() {
  const dispatch = useAppDispatch()

  const onSubmit = async (
    data: BootstrapNodeData,
  ): Promise<ActionPromiseResult> =>
    getApi()
      .nodeStewardApi.bootstrap(data)
      .then((result) => {
        if (result.status === 200) {
          console.log("Successfully bootstrapped", result)
          const newRegion: Region = {
            name: data.network_name,
          }
          dispatch(regionLoaded(newRegion))
          return actionSuccess()
        }
      })
      .catch(actionFailure)

  return <CreateRegionForm onSubmit={onSubmit} />
}
