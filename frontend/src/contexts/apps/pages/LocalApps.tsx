import { Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import LocalAppsList from "../components/LocalAppsList"

export default function LocalApps() {
  const apps = useAppSelector((state) => state.localApps)

  return (
    <Container>
      <Stack>
        <Title order={1}>Local Apps</Title>

        {apps && <LocalAppsList apps={apps} />}
      </Stack>
    </Container>
  )
}
