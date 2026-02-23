import { Container, Stack, Title } from "@mantine/core"
import NodesList from "../components/NodesList"
import { useAppSelector } from "../../../store"
import { activeRegion } from "../../../store/regions"

export default function Nodes() {
  const region = useAppSelector((state) => activeRegion(state.regions))
  const nodes = useAppSelector((state) => state.nodes)

  if (!region) {
    return <Container>No region</Container>
  }

  return (
    <Container>
      <Stack>
        <Title order={1}>Nodes</Title>
        <Title order={2}>{region.name}</Title>

        {nodes && <NodesList nodes={nodes} />}
      </Stack>
    </Container>
  )
}
