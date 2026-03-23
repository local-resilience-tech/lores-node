import { Container, Stack, Title } from "@mantine/core"
import EditRegionMapForm from "../components/EditRegionMapForm"
import { useAppSelector } from "../../../store"

import { useParams } from "react-router-dom"
import { UpdateMapData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"
import { getApi } from "../../../api"
export default function EditRegionMap() {
  const { regionSlug } = useParams<{ regionSlug: string }>()
  const region = useAppSelector(
    (state) =>
      state.my_regions.all?.find((r) => r.region.slug === regionSlug)?.region,
  )
  const myNodeId = useAppSelector((state) => state.network?.node.id)

  if (!region) return <div>Region not found</div>

  const isCreator = myNodeId && region.creator_node_id === myNodeId
  if (!isCreator)
    return <div>You do not have permission to edit this region</div>

  const onSubmit = async (
    data: UpdateMapData,
  ): Promise<ActionPromiseResult> => {
    console.log("Submitting map update with data:", data)
    return getApi()
      .nodeStewardApi.updateMap(data)
      .then(() => {
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return (
    <Container>
      <Stack gap="lg">
        <Title>Edit map</Title>
        <EditRegionMapForm regionId={region.id} onSubmit={onSubmit} />
      </Stack>
    </Container>
  )
}
