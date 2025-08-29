import { Breadcrumbs, Container, Stack, Title, Text, Card } from "@mantine/core"
import {
  actionFailure,
  ActionResult,
  actionSuccess,
  Anchor,
} from "../../../components"
import { useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import LocalAppDetails from "../components/LocalAppDetails"
import { useAppRepo } from "../../../store/app_repos"
import LocalAppUpgrades, {
  UpgradeLocalAppError,
} from "../components/LocalAppUpgrades"
import { getApi } from "../../../api"
import { LocalApp, LocalAppInstallStatus } from "../../../api/Api"
import { useState } from "react"
import LocalAppActions, { LocalAppAction } from "../components/LocalAppActions"
import { IfNodeSteward } from "../../auth/node_steward_auth"

export default function ShowLocalApp() {
  const { appName } = useParams<{ appName: string }>()
  const app = useAppSelector((state) =>
    (state.localApps || []).find((a) => a.name === appName)
  )
  const appRepo = useAppRepo(app?.repo_name)
  const [upgradeError, setUpgradeError] = useState<UpgradeLocalAppError | null>(
    null
  )

  if (!appName) {
    return <Container>Error: App name is required</Container>
  }

  if (!app) {
    return <Container>Error: App not found</Container>
  }

  const handleUpgrade = async (version: string) => {
    console.log("Upgrading app:", app.name, "to version:", version)
    return getApi()
      .nodeStewardApi.upgradeLocalApp(app.name, { target_version: version })
      .then((response) => {
        console.log("Upgrade successful:", response)
      })
      .catch((error) => {
        console.error("Upgrade failed:", error)
        setUpgradeError(error.response?.data || "ServerError")
      })
  }

  const onAppDeploy = async (app: LocalApp) => {
    console.log("Deploying app:", app)
    return getApi()
      .nodeStewardApi.deployLocalApp(app.name)
      .then((_) => actionSuccess())
      .catch(actionFailure)
  }

  const onAppRemoveDeploy = async (app: LocalApp) => {
    console.log("Removing deployment of app:", app)
    return getApi()
      .nodeStewardApi.removeDeploymentOfLocalApp(app.name)
      .then((_) => actionSuccess())
      .catch(actionFailure)
  }

  const onAppRegister = async (app: LocalApp) => {
    console.log("Registering app:", app)
    return getApi()
      .nodeStewardApi.registerApp({ app_name: app.name })
      .then((_) => actionSuccess())
      .catch(actionFailure)
  }

  const actions: LocalAppAction[] = []
  if (app.status === LocalAppInstallStatus.Installed) {
    actions.push({
      type: "deploy",
      buttonColor: "blue",
      primary: true,
      handler: onAppDeploy,
    })
  }

  if (app.status === LocalAppInstallStatus.StackDeployed) {
    actions.push({
      type: "remove",
      buttonColor: "red",
      handler: onAppRemoveDeploy,
    })
  }

  actions.push({
    type: "register",
    buttonColor: "blue",
    handler: onAppRegister,
  })

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/this_node/apps">Local Apps</Anchor>
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
          <LocalAppUpgrades
            app={app}
            appRepo={appRepo}
            onUpgrade={handleUpgrade}
            upgradeError={upgradeError}
          />
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
