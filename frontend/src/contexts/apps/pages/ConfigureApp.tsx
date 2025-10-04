import { Breadcrumbs, Container, Stack, Title, Text } from "@mantine/core"
import {
  actionFailure,
  actionSuccess,
  Anchor,
  JsonSchemaForm,
} from "../../../components"
import { useAppSelector } from "../../../store"
import { useParams } from "react-router-dom"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"

export default function ConfigureApp() {
  const { appName } = useParams<{ appName: string }>()
  const app = useAppSelector((state) =>
    (state.localApps || []).find((a) => a.name === appName)
  )
  const [configSchema, setConfigSchema] = useState<object | null | undefined>(
    undefined
  )

  const loadConfigSchema = async () => {
    if (!app) return
    getApi()
      .nodeStewardApi.getLocalAppConfigSchema(app.name)
      .then((res) => {
        console.log("Config schema:", res.data)
        setConfigSchema(res.data)
      })
      .catch((err) => {
        console.error("Failed to load config schema:", err)
        setConfigSchema(null)
      })
  }

  const updateConfig = async (newConfig: any) => {
    if (!app) return

    console.log("Updating config to:", newConfig)
    getApi()
      .nodeStewardApi.updateLocalAppConfig(app.name, newConfig)
      .then(() => {
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  useEffect(() => {
    loadConfigSchema()
  }, [app])

  if (!appName || !app) {
    return <Container>Error: App not found</Container>
  }

  if (configSchema === undefined) {
    return <Container>Loading config schema...</Container>
  }

  if (configSchema === null) {
    return <Container>Error loading config schema.</Container>
  }

  return (
    <Container>
      <Stack gap="lg">
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

        <JsonSchemaForm
          schema={configSchema}
          displaySchema
          onSubmit={updateConfig}
        />
      </Stack>
    </Container>
  )
}
