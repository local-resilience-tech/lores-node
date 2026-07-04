import { Breadcrumbs, Container, Stack, Title, Text, Card } from "@mantine/core"
import { actionFailure, actionSuccess, Anchor } from "../../../components"
import { useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import LocalAppDetails from "../components/LocalAppDetails"
import { getApi } from "../../../api"
import { LocalApp } from "../../../api/Api"
import ActionButton from "../../../components/ActionButton"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { activeRegion } from "../../../store/my_regions"

export default function ShowLocalApp() {
  const { appName } = useParams<{ appName: string }>()
  const app = useAppSelector((state) =>
    (state.localApps || []).find((a) => a.name === appName),
  )
  const region = useAppSelector((state) => activeRegion(state.my_regions))

  if (!appName) {
    return <Container>Error: App name is required</Container>
  }

  if (!app) {
    return <Container>Error: App not found</Container>
  }

  const onRegister = region
    ? async (app: LocalApp) => {
        console.log("Registering app:", app)
        return getApi()
          .nodeStewardApi.registerApp({ app: app, region_id: region.id })
          .then((_) => actionSuccess())
          .catch(actionFailure)
      }
    : undefined

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/node/apps">Local Apps</Anchor>
            <Text c="dimmed">{app.name}</Text>
          </Breadcrumbs>
          <Title order={1}>
            <Text span inherit c="dimmed">
              Local App:{" "}
            </Text>
            {app.name}
          </Title>
        </Stack>

        <Stack gap="xs">
          <Title order={2}>Details</Title>
          <Card>
            <Card.Section>
              <LocalAppDetails app={app} />
            </Card.Section>
          </Card>
        </Stack>

        {onRegister && region && (
          <IfNodeSteward>
            <Stack>
              <Title order={2}>Actions</Title>
              <ActionButton size="sm" onClick={() => onRegister(app)}>
                Register with {region.slug}
              </ActionButton>
            </Stack>
          </IfNodeSteward>
        )}
      </Stack>
    </Container>
  )
}
