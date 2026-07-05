import { Breadcrumbs, Container, Stack, Title, Text } from "@mantine/core"
import { useNavigate, useParams } from "react-router-dom"
import { getApi } from "../../../api"
import { LocalAppFormData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
} from "../../../components"
import LocalAppForm from "../components/LocalAppForm"
import { useAppSelector } from "../../../store"

function appPath(appName: string | undefined, instanceId: string | undefined) {
  return `/node/apps/app/${encodeURIComponent(appName || "-")}/${encodeURIComponent(instanceId || "-")}`
}

export default function EditLocalApp() {
  const navigate = useNavigate()
  const { appName, instanceId } = useParams<{
    appName: string
    instanceId: string
  }>()
  const installation = useAppSelector((state) =>
    (state.localApps || []).find(
      (i) => i.app.name === appName && i.app.instance_id === instanceId,
    ),
  )

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
            installation
              ? {
                  instance_id: installation.app.instance_id,
                  name: installation.app.name,
                  version: installation.app.version,
                }
              : undefined
          }
        />
      </Stack>
    </Container>
  )
}
