import { Breadcrumbs, Container, Stack, Title, Text, Card } from "@mantine/core"
import { actionFailure, actionSuccess, Anchor } from "../../../components"
import { useNavigate, useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import LocalAppDetails from "../components/LocalAppDetails"
import { useAppRepo } from "../../../store/app_repos"
import LocalAppUpgrades from "../components/LocalAppUpgrades"
import { getApi } from "../../../api"
import { LocalApp, LocalAppInstallStatus } from "../../../api/Api"
import LocalAppActions, {
  confirmLocalAppAction,
  LocalAppAction,
} from "../components/LocalAppActions"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { notifications } from "@mantine/notifications"

export default function ShowLocalApp() {
  const navigate = useNavigate()
  const { appName } = useParams<{ appName: string }>()
  const app = useAppSelector((state) =>
    (state.localApps || []).find((a) => a.name === appName)
  )
  const appRepo = useAppRepo(app?.repo_name)

  if (!appName) {
    return <Container>Error: App name is required</Container>
  }

  if (!app) {
    return <Container>Error: App not found</Container>
  }

  const onAppRegister = async (app: LocalApp) => {
    console.log("Registering app:", app)
    return getApi()
      .nodeStewardApi.registerApp({ app_name: app.name })
      .then((_) => actionSuccess())
      .catch(actionFailure)
  }

  const onAppDelete = async (app: LocalApp) => {
    console.log("Deleting app:", app)
    return getApi()
      .nodeStewardApi.deleteLocalApp(app.name)
      .then((_) => {
        navigate("../")
        notifications.show({
          message: `App "${app.name}" deleted successfully`,
          autoClose: 3000,
        })
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  const actions: LocalAppAction[] = []

  if (app.status === LocalAppInstallStatus.StackDeployed) {
  } else {
    actions.push({
      type: "delete",
      buttonColor: "red",
      handler: confirmLocalAppAction(
        onAppDelete,
        "Confirm App Deletion",
        <Text size="sm">
          Are you sure you want to delete the app "{app.name}"? This action
          cannot be undone.
        </Text>
      ),
    })
  }

  actions.push({
    type: "register",
    buttonColor: "blue",
    handler: onAppRegister,
  })

  if (app.has_config_schema) {
    actions.push({
      type: "configure",
      buttonColor: "green",
      handler: async (_) => {
        navigate(`./configure`)
        return actionSuccess()
      },
      primary: true,
    })
  }

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/this_node/`apps">Local Apps</Anchor>
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

        <Stack>
          <Title order={2}>Upgrades</Title>
          <LocalAppUpgrades app={app} appRepo={appRepo} />
        </Stack>

        <IfNodeSteward>
          <Stack>
            <Title order={2}>Actions</Title>
            <LocalAppActions actions={actions} app={app} />
          </Stack>
        </IfNodeSteward>
      </Stack>
    </Container>
  )
}
