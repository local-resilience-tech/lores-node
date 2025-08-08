import {
  ActionIcon,
  Badge,
  Button,
  Group,
  HoverCard,
  Table,
  Text,
} from "@mantine/core"
import { LocalApp, LocalAppInstallStatus } from "../../../api/Api"
import {
  IconAlertCircle,
  IconBrandDocker,
  IconDatabase,
} from "@tabler/icons-react"
import { useLoading } from "../../shared"

interface AppsListProps {
  apps: LocalApp[]
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
        {apps.map((app) => (
          <LocalAppRow
            key={app.name}
            app={app}
            onDeploy={onDeploy}
            onRemoveDeploy={onRemoveDeploy}
            onRegister={onRegister}
            error={appErrors?.get(app.name)}
          />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface LocalAppRowProps {
  app: LocalApp
  error?: string
  onDeploy?: (app: LocalApp) => Promise<void>
  onRemoveDeploy?: (app: LocalApp) => Promise<void>
  onRegister?: (app: LocalApp) => Promise<void>
}

function LocalAppRow({
  app,
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
      <Table.Td>{app.name}</Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>X.X.X</Table.Td>
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

function LocalAppStatusBadge({ status }: { status: LocalAppInstallStatus }) {
  const sharedProps = {
    size: "lg",
    variant: "outline",
  }

  switch (status) {
    case LocalAppInstallStatus.Installed:
      return (
        <Badge color="yellow" {...sharedProps} leftSection={<IconDatabase />}>
          Installed
        </Badge>
      )
    case LocalAppInstallStatus.StackDeployed:
      return (
        <Badge color="blue" {...sharedProps} leftSection={<IconBrandDocker />}>
          Stack Deployed
        </Badge>
      )
  }
}
