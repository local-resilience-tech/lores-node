import { Breadcrumbs, Container, Stack, Title, Text, Card } from "@mantine/core"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
} from "../../../components"
import { useNavigate, useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import LocalAppDetails from "../components/LocalAppDetails"
import { useAppRepo } from "../../../store/app_repos"
import LocalAppUpgrades, {
  UpgradeLocalAppError,
} from "../components/LocalAppUpgrades"
import { getApi } from "../../../api"
import { LocalApp, LocalAppInstallStatus } from "../../../api/Api"
import { useState } from "react"
import LocalAppActions, {
  LocalAppAction,
  LocalAppActionHandler,
} from "../components/LocalAppActions"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { modals } from "@mantine/modals"

function confirmAction(
  actionHandler: LocalAppActionHandler,
  title?: string,
  children?: React.ReactNode
): LocalAppActionHandler {
  let result: Promise<ActionPromiseResult> | null = null

  const handler = async (app: LocalApp) => {
    const openModal = () =>
      modals.openConfirmModal({
        title: title ?? "Please confirm your action",
        children: children ?? (
          <Text size="sm">
            This action is so important that you are required to confirm it with
            a modal. Please click one of these buttons to proceed.
          </Text>
        ),
        labels: { confirm: "Confirm", cancel: "Cancel" },
        onCancel: () => (result = null),
        onConfirm: () => (result = actionHandler(app)),
      })

    openModal()
    console.log("Modal result:", result)
    if (result) {
      return result
    } else {
      return {
        success: false,
        error: "Action cancelled",
        login_needed: false,
      }
    }
  }
  return handler
}

export default function ShowLocalApp() {
  const navigate = useNavigate()
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

  const onAppDelete = async (app: LocalApp) => {
    console.log("Deleting app:", app)
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
  } else {
    actions.push({
      type: "delete",
      buttonColor: "red",
      handler: confirmAction(
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
