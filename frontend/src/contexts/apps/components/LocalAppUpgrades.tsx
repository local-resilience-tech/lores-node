import { Alert, Button, Stack, Text, Title } from "@mantine/core"
import { AppDefinition, AppRepo, LocalApp } from "../../../api/Api"
import { IconAlertCircle } from "@tabler/icons-react"
import semver from "semver"
import { useLoading } from "../../shared"
import { Anchor } from "../../../components"

export type UpgradeLocalAppError = "AppNotFound" | "InUse" | "ServerError"

interface LocalAppUpgradesProps {
  app: LocalApp
  appRepo?: AppRepo
  onUpgrade: (version: string) => Promise<void>
  upgradeError?: UpgradeLocalAppError | null
}

export default function LocalAppUpgrades({
  app,
  appRepo,
  onUpgrade,
  upgradeError = null,
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
        color={upgradeError ? "red" : "green"}
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
          {upgradeError === "InUse" && (
            <Text c="red">
              <strong>Upgrade failed - Repository in use.</strong>
              <br />
              The repository is checked out to another version, which generally
              means that it's in-use for another upgrade. It's possible that it
              got stuck in this state due to a crash during an upgrade. If you
              are confident that nothing else is using this repository, you can
              refresh it on the{" "}
              <Anchor href={`/this_node/app_repos/repo/${appRepo.name}`}>
                {appRepo.name}
              </Anchor>{" "}
              repository page. Then come back here and try the upgrade again.
            </Text>
          )}

          {upgradeError === "AppNotFound" && (
            <Text c="red">
              <strong>Upgrade failed - App not found.</strong>
              <br />
              The app definition for {app.name} is not found in the repository{" "}
              {appRepo.name}. This may indicate that the app was removed from
              the repository.
            </Text>
          )}

          {upgradeError === "ServerError" && (
            <Text c="red">
              <strong>Upgrade failed - Server error.</strong>
              <br />
              An unexpected server error occurred while trying to upgrade the
              app. Please try again later or contact the developers if the issue
              persists.
            </Text>
          )}
        </Stack>
      </Alert>
    )
  }

  return <Text>This app is up to date.</Text>
}
