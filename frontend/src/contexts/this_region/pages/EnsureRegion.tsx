import { Container } from "@mantine/core"
import SetRegion from "../components/SetRegion"
import { Outlet } from "react-router-dom"
import { getApi } from "../../../api"
import { BootstrapNodeData, Region } from "../../../api/Api"
import { regionLoaded } from "../../../store/region"
import { useAppDispatch, useAppSelector } from "../../../store"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"

export default function EnsureRegion({
  children,
}: {
  children?: React.ReactNode
}) {
  const region = useAppSelector((state) => state.region)
  const dispatch = useAppDispatch()

  const onSubmit = async (
    data: BootstrapNodeData
  ): Promise<ActionPromiseResult> =>
    getApi()
      .nodeStewardApi.bootstrap(data)
      .then((result) => {
        if (result.status === 200) {
          console.log("Successfully bootstrapped", result)
          const newRegion: Region = {
            network_id: data.network_name,
          }
          dispatch(regionLoaded(newRegion))
          return actionSuccess()
        }
      })
      .catch(actionFailure)

  return (
    <Container>
      {!region && <SetRegion onSubmit={onSubmit} />}
      {region && (children || <Outlet />)}
    </Container>
  )
}
