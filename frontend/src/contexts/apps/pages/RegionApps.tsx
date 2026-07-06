import { Container, Stack, Title } from "@mantine/core"

import RegionAppsList from "../components/RegionAppsList"
import { useAppSelector } from "../../../store"
import { activeRegionWithNodes, hashById } from "../../../store/my_regions"
import { regionAppsForRegion } from "../../../store/region_apps"

export default function RegionApps() {
  const activeRegionId = useAppSelector(
    (state) => state.my_regions.activeRegionId,
  )
  const apps = useAppSelector((state) =>
    regionAppsForRegion(state.regionApps, activeRegionId),
  )
  const nodes = hashById(
    useAppSelector(
      (state) => activeRegionWithNodes(state.my_regions)?.nodes ?? [],
    ),
  )

  return (
    <Container>
      <Stack>
        <Title order={1}>All apps</Title>

        {apps && <RegionAppsList apps={apps} nodes={nodes} />}
      </Stack>
    </Container>
  )
}
