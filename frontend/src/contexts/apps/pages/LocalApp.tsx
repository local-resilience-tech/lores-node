import { Breadcrumbs, Container, Stack, Title, Text, Card } from "@mantine/core"
import { Anchor } from "../../../components"
import { useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import LocalAppDetails from "../components/LocalAppDetails"
import { useAppRepo } from "../../../store/app_repos"
import LocalAppUpgrades, {
  UpgradeLocalAppError,
} from "../components/LocalAppUpgrades"
import { getApi } from "../../../api"
import { useState } from "react"

export default function LocalApp() {
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
      .api.upgradeLocalApp(app.name, { target_version: version })
      .then((response) => {
        console.log("Upgrade successful:", response)
      })
      .catch((error) => {
        console.error("Upgrade failed:", error)
        setUpgradeError(error.response?.data || "ServerError")
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
      </Stack>
    </Container>
  )
}
