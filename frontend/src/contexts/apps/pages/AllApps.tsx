import { Container, Stack, Title } from "@mantine/core"

import RegionAppsList from "../components/RegionAppsList"

export default function AllApps() {
  return (
    <Container>
      <Stack>
        <Title order={1}>All Apps</Title>

        <RegionAppsList apps={[]} />
      </Stack>
    </Container>
  )
}
