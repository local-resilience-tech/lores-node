import { Container, Stack, Title } from "@mantine/core"
import NodesList from "../components/NodesList"
import { useAppSelector } from "../../../store"
import { activeRegionWithNodes } from "../../../store/my_regions"

export default function Nodes() {
  const region = useAppSelector((state) =>
    activeRegionWithNodes(state.my_regions),
  )

  if (!region) {
    return <Container>No region</Container>
  }

  const nodes = region.nodes

  console.log("Region nodes", nodes)

  return (
    <Container>
      <Stack>
        <Title order={1}>Nodes</Title>
        <Title order={2}>{region.region.name}</Title>

        {nodes && <NodesList nodes={nodes} />}
      </Stack>
    </Container>
  )
}
