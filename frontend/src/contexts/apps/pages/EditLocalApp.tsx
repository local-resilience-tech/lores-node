import { Breadcrumbs, Container, Stack, Title, Text } from "@mantine/core"
import { useNavigate, useParams } from "react-router-dom"
import { getApi } from "../../../api"
import { LocalAppFormData } from "../../../api/Api"
import {
  ActionButton,
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
} from "../../../components"
import LocalAppForm from "../components/LocalAppForm"
import { useAppSelector } from "../../../store"
import { awaitConfirmModal } from "../../shared/components/Modal"
import { actionCancelled } from "../../../components/ActionResult"

function appPath(
  appName: string | undefined,
  instanceId: string | undefined | null,
) {
  return `/node/apps/app/${encodeURIComponent(appName || "-")}/${encodeURIComponent(instanceId || "-")}`
}

export default function EditLocalApp() {
  const navigate = useNavigate()

  const { appName, instanceId: instanceIdParam } = useParams<{
    appName: string
    instanceId: string
  }>()
  const instanceId: string | null =
    instanceIdParam === "-" ? null : instanceIdParam || null

  const app = useAppSelector((state) =>
    (state.localApps || []).find(
      (app) => app.name === appName && app.instance_id === instanceId,
    ),
  )

  if (!appName) {
    return <Container>Error: App name is required</Container>
  }

  if (!app) {
    return (
      <Container>
        Error: App not found with name "{appName}" and instance ID "{instanceId}
        "
      </Container>
    )
  }

  const onSubmit = async (
    data: LocalAppFormData,
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.updateLocalApp(data)
      .then(() => {
        navigate(appPath(appName, instanceId))
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  const onDelete = async (): Promise<ActionPromiseResult> => {
    const confirmed = await awaitConfirmModal(
      "Delete app",
      <Text size="sm">
        Are you sure you want to delete <strong>{appName}</strong>? This action
        cannot be undone.
      </Text>,
    )
    if (!confirmed) return actionCancelled()

    console.log("Deleting app", appName, instanceId)

    return getApi()
      .nodeStewardApi.deleteLocalAppRecord({
        name: appName,
        instance_id: instanceId,
      })
      .then(() => {
        navigate("/node/apps")
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/node/apps">Local apps</Anchor>
            <Anchor href={appPath(appName, instanceId)}>{appName}</Anchor>
            <Text c="dimmed">edit</Text>
          </Breadcrumbs>
          <Title order={1}>Edit local app</Title>
        </Stack>

        <LocalAppForm
          onSubmit={onSubmit}
          submitLabel="Save changes"
          cancelPath={appPath(appName, instanceId)}
          disableKeyFields
          initialValues={
            app
              ? {
                  instance_id: app.instance_id,
                  name: app.name,
                  version: app.version,
                }
              : undefined
          }
        />

        <Stack align="flex-start">
          <ActionButton onClick={onDelete} variant="outline" color="red">
            Delete
          </ActionButton>
        </Stack>
      </Stack>
    </Container>
  )
}
