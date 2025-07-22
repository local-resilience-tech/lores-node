import { Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import AppsList from "../components/AppsList"

export default function AllApps() {
  return (
    <Container>
      <Stack>
        <Title order={1}>All Apps</Title>

        <AppsList apps={[]} />
      </Stack>
    </Container>
  )
}
