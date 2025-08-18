import {
  ActionIcon,
  Button,
  Group,
  HoverCard,
  Table,
  Text,
} from "@mantine/core"
import {
  AppDefinition,
  AppRepo,
  LocalApp,
  LocalAppInstallStatus,
} from "../../../api/Api"
import { IconAlertCircle } from "@tabler/icons-react"
import { useLoading } from "../../shared"
import { ActionResult, Anchor } from "../../../components"
import LocalAppStatusBadge from "./LocalAppStatusBadge"

export interface LocalAppWithRepo {
  app: LocalApp
  repo: AppRepo | undefined
  repo_app_definition: AppDefinition | undefined
}

interface AppsListProps {
  apps: LocalAppWithRepo[]
  appErrors?: Map<string, string>
  onDeploy?: (app: LocalApp) => Promise<void>
  onRemoveDeploy?: (app: LocalApp) => Promise<void>
  onRegister?: (app: LocalApp) => Promise<void>
}

export default function LocalAppsList({
  apps,
  appErrors,
  onDeploy,
  onRemoveDeploy,
  onRegister,
}: AppsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Latest</Table.Th>
          <Table.Th>Status</Table.Th>
          <Table.Th>Actions</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((appWithRepo) => (
          <LocalAppRow
            key={appWithRepo.app.name}
            app={appWithRepo.app}
            repo_app_definition={appWithRepo.repo_app_definition}
            onDeploy={onDeploy}
            onRemoveDeploy={onRemoveDeploy}
            onRegister={onRegister}
            error={appErrors?.get(appWithRepo.app.name)}
          />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface LocalAppRowProps {
  app: LocalApp
  repo_app_definition?: AppDefinition
  error?: string
  onDeploy?: (app: LocalApp) => Promise<void>
  onRemoveDeploy?: (app: LocalApp) => Promise<void>
  onRegister?: (app: LocalApp) => Promise<void>
}

function LocalAppRow({
  app,
  repo_app_definition,
  error,
  onDeploy,
  onRemoveDeploy,
  onRegister,
}: LocalAppRowProps) {
  const [loading, withLoading] = useLoading(false)

  const handleButtonPress = (
    app: LocalApp,
    action: (app: LocalApp) => Promise<void>
  ) => {
    withLoading(async () => {
      await action(app)
    })
  }

  return (
    <Table.Tr key={app.name}>
      <Table.Td>
        <Anchor href={`app/${app.name}`}>{app.name}</Anchor>
      </Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>{repo_app_definition?.latest_version}</Table.Td>
      <Table.Td>
        <LocalAppStatusBadge status={app.status} />
      </Table.Td>
      <Table.Td>
        <Group gap="xs">
          {onDeploy && app.status === LocalAppInstallStatus.Installed && (
            <Button
              onClick={() => handleButtonPress(app, onDeploy)}
              loading={loading}
              size="xs"
            >
              Deploy
            </Button>
          )}
          {onRemoveDeploy &&
            app.status === LocalAppInstallStatus.StackDeployed && (
              <Button
                variant="outline"
                color="red"
                onClick={() => handleButtonPress(app, onRemoveDeploy)}
                loading={loading}
                size="xs"
              >
                Remove
              </Button>
            )}
          {onRegister && app.status === LocalAppInstallStatus.StackDeployed && (
            <Button
              variant="outline"
              onClick={() => handleButtonPress(app, onRegister)}
              loading={loading}
              size="xs"
            >
              Register
            </Button>
          )}
          {error && (
            <HoverCard width={280} shadow="md">
              <HoverCard.Target>
                <ActionIcon variant="transparent" color="red" size="lg">
                  <IconAlertCircle />
                </ActionIcon>
              </HoverCard.Target>
              <HoverCard.Dropdown>
                <Text size="sm">{error}</Text>
              </HoverCard.Dropdown>
            </HoverCard>
          )}
        </Group>
      </Table.Td>
    </Table.Tr>
  )
}
