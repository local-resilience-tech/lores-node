import { Alert, Button, Stack, Text } from "@mantine/core"
import { AppDefinition, AppRepo, LocalApp } from "../../../api/Api"
import { IconAlertCircle } from "@tabler/icons-react"
import semver from "semver"
import { useLoading } from "../../shared"

interface LocalAppUpgradesProps {
  app: LocalApp
  appRepo?: AppRepo
  onUpgrade: (version: string) => Promise<void>
}

export default function LocalAppUpgrades({
  app,
  appRepo,
  onUpgrade,
}: LocalAppUpgradesProps) {
  const [upgradeLoading, withUpgradeLoading] = useLoading(false)

  if (!appRepo) {
    return (
      <Alert title="No repository found" icon={<IconAlertCircle />} color="red">
        This app is not registered in any current repository. This is unusual
        for an installed app and may indicate that you have removed the
        repository that the app was installed from.
      </Alert>
    )
  }

  const appDef = appRepo.apps?.find((def) => def.name === app.name)

  if (!appDef) {
    return (
      <Alert
        title="App definition not found"
        icon={<IconAlertCircle />}
        color="red"
      >
        The app definition for {app.name} is not found in the repository{" "}
        {appRepo.name}.
      </Alert>
    )
  }

  if (!appDef.latest_version) {
    return (
      <Alert
        title="No latest version available"
        icon={<IconAlertCircle />}
        color="yellow"
      >
        The app definition in the repository <strong>{appRepo.name}</strong>{" "}
        does not have a latest version defined.
      </Alert>
    )
  }

  const latest_version = appDef.latest_version
  const newerVersion = semver.gt(appDef.latest_version, app.version)

  if (newerVersion) {
    return (
      <Alert
        title="New version available"
        icon={<IconAlertCircle />}
        color="green"
      >
        <Stack align="flex-start">
          <Text>
            A new version of <strong>{app.name}</strong> is available:{" "}
            <strong>{appDef.latest_version}</strong>
          </Text>
          <Button
            onClick={() => withUpgradeLoading(() => onUpgrade(latest_version))}
            loading={upgradeLoading}
          >
            Upgrade
          </Button>
        </Stack>
      </Alert>
    )
  }

  return <Text>This app is up to date.</Text>
}
