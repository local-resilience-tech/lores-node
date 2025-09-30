import { Breadcrumbs, Container, Stack, Title, Text } from "@mantine/core"
import { Anchor } from "../../../components"
import { useAppSelector } from "../../../store"
import { useParams } from "react-router-dom"

export default function ConfigureApp() {
  const { appName } = useParams<{ appName: string }>()
  const app = useAppSelector((state) =>
    (state.localApps || []).find((a) => a.name === appName)
  )

  if (!appName || !app) {
    return <Container>Error: App not found</Container>
  }

  return (
    <Container>
      <Stack gap="xs">
        <Breadcrumbs>
          <Anchor href="/this_node/apps">Local Apps</Anchor>
          <Anchor href={`/this_node/apps/app/${app.name}`}>{app.name}</Anchor>
          <Text c="dimmed">configure</Text>
        </Breadcrumbs>
        <Title order={1}>
          <Text span inherit c="dimmed">
            Configure App:{" "}
          </Text>
          {app.name}
        </Title>
      </Stack>
    </Container>
  )
}
