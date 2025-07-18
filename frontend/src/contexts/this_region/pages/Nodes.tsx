import { Container, Stack, Title } from "@mantine/core"
import { useEffect } from "react"
import NodesList from "../components/NodesList"
import { Loading, useLoading } from "../../shared"
import { getApi } from "../../../api"
import { NodeDetails } from "../../../api/Api"
import { useAppDispatch, useAppSelector } from "../../../store"
import { nodesLoaded } from "../../../store/nodes"

const getNodes = async (): Promise<NodeDetails[] | null> => {
  const result = await getApi().api.showRegionNodes()
  if (result.status === 200) return result.data
  return null
}

export default function Nodes() {
  const region = useAppSelector((state) => state.region)
  const nodes = useAppSelector((state) => state.nodes)
  const dispatch = useAppDispatch()

  if (!region) {
    return <Container>No region</Container>
  }

  const [loading, withLoading] = useLoading(nodes == null)

  const fetchNodes = async () => {
    withLoading(async () => {
      const result = await getNodes()
      console.log("EFFECT: fetchNodes", result)
      if (result) {
        dispatch(nodesLoaded(result))
      }
    })
  }

  useEffect(() => {
    if (nodes == null) fetchNodes()
  }, [])

  if (loading) return <Loading />

  return (
    <Container>
      <Stack>
        <Title order={1}>Nodes</Title>
        <Title order={2}>{region.network_id}</Title>

        {nodes && <NodesList nodes={nodes} />}
      </Stack>
    </Container>
  )
}
