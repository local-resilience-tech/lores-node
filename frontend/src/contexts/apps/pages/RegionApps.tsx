import { Container, Stack, Title } from "@mantine/core"

import RegionAppsList from "../components/RegionAppsList"
import { useAppSelector } from "../../../store"
import { activeRegionWithNodes, hashById } from "../../../store/my_regions"

export default function RegionApps() {
  const apps = useAppSelector((state) => state.regionApps)
  const nodes = hashById(
    useAppSelector(
      (state) => activeRegionWithNodes(state.my_regions)?.nodes ?? [],
    ),
  )

  return (
    <Container>
      <Stack>
        <Title order={1}>All Apps</Title>

        {apps && <RegionAppsList apps={apps} nodes={nodes} />}
      </Stack>
    </Container>
  )
}
