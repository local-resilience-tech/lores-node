import { Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import AppsList from "../components/AppsList"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.apps)

  return (
    <Container>
      <Stack>
        <Title order={1}>Local Apps</Title>

        {apps && <AppsList apps={apps} />}
      </Stack>
    </Container>
  )
}
