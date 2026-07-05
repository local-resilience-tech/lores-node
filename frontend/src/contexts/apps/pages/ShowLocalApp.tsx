import {
  Breadcrumbs,
  Container,
  Stack,
  Title,
  Text,
  Card,
  Group,
  ActionIcon,
} from "@mantine/core"
import { actionFailure, actionSuccess, Anchor } from "../../../components"
import { useNavigate, useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import LocalAppDetails from "../components/LocalAppDetails"
import { getApi } from "../../../api"
import ActionButton from "../../../components/ActionButton"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { activeRegion } from "../../../store/my_regions"
import { regionDisplayName } from "../../regions"
import { IconEdit } from "@tabler/icons-react"

export default function ShowLocalApp() {
  const { appName, instanceId } = useParams<{
    appName: string
    instanceId: string
  }>()
  const installation = useAppSelector((state) =>
    (state.localApps || []).find(
      (i) => i.app.name === appName && i.app.instance_id === instanceId,
    ),
  )
  const region = useAppSelector((state) => activeRegion(state.my_regions))
  const appRegion = useAppSelector((state) =>
    installation
      ? state.my_regions.all?.find(
          (r) => r.region.id === installation.region_id,
        )
      : undefined,
  )
  const navigate = useNavigate()

  if (!appName) {
    return <Container>Error: App name is required</Container>
  }

  if (!installation) {
    return <Container>Error: App not found</Container>
  }

  const app = installation.app

  const onRegister = region
    ? async () => {
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
            <Anchor href="/node/apps">Local apps</Anchor>
            <Text c="dimmed">{app.name}</Text>
          </Breadcrumbs>
          <Group justify="space-between" align="center">
            <Title order={1}>
              <Text span inherit c="dimmed">
                Local App:{" "}
              </Text>
              {app.name}
            </Title>
            <IfNodeSteward>
              <ActionIcon size="lg" onClick={() => navigate("./edit")}>
                <IconEdit />
              </ActionIcon>
            </IfNodeSteward>
          </Group>
        </Stack>

        <Stack gap="xs">
          <Title order={2}>Details</Title>
          <Card>
            <Card.Section>
              <LocalAppDetails app={app} />
            </Card.Section>
          </Card>
        </Stack>

        {appRegion && (
          <Card>
            <Text>
              <Text c="dimmed" span>
                Region:
              </Text>{" "}
              {regionDisplayName(appRegion.region)}
            </Text>
          </Card>
        )}

        {onRegister &&
          region &&
          (!appRegion || appRegion.region.id !== region.id) && (
            <IfNodeSteward>
              <ActionButton size="sm" onClick={() => onRegister()}>
                {appRegion ? "Change to" : "Register with"} {region.slug}
              </ActionButton>
            </IfNodeSteward>
          )}
      </Stack>
    </Container>
  )
}
