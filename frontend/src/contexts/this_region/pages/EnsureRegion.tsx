import { Container } from "@mantine/core"
import SetRegion from "../components/SetRegion"
import { Outlet } from "react-router-dom"
import { getApi } from "../../../api"
import { BootstrapNodeData, Region } from "../../../api/Api"
import { regionLoaded } from "../../../store/region"
import { useAppDispatch, useAppSelector } from "../../../store"

export default function EnsureRegion({
  children,
}: {
  children?: React.ReactNode
}) {
  const region = useAppSelector((state) => state.region)
  const dispatch = useAppDispatch()

  const onSubmit = async (data: BootstrapNodeData) => {
    getApi()
      .publicApi.bootstrap(data)
      .then((result) => {
        if (result.status === 200) {
          console.log("Successfully bootstrapped", result)
          const newRegion: Region = {
            network_id: data.network_name,
          }
          dispatch(regionLoaded(newRegion))
        } else {
          console.log("Failed to bootstrap", result)
        }
      })
  }

  return (
    <Container>
      {!region && <SetRegion onSubmit={onSubmit} />}
      {region && (children || <Outlet />)}
    </Container>
  )
}
