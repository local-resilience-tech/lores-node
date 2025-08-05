import { Badge, Button, Group, Table } from "@mantine/core"
import { LocalApp, LocalAppInstallStatus } from "../../../api/Api"
import { IconBrandDocker, IconDatabase } from "@tabler/icons-react"
import { useLoading } from "../../shared"

interface AppsListProps {
  apps: LocalApp[]
  onDeploy?: (app: LocalApp) => Promise<Promise<void>>
  onRemoveDeploy?: (app: LocalApp) => Promise<void>
  onRegister?: (app: LocalApp) => Promise<void>
}

export default function LocalAppsList({
  apps,
  onDeploy,
  onRemoveDeploy,
  onRegister,
}: AppsListProps) {
  const [loading, withLoading] = useLoading(false)

  const handleButtonPress = (
    app: LocalApp,
    action: (app: LocalApp) => Promise<Promise<void>>
  ) => {
    withLoading(async () => {
      await action(app)
    })
  }

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Status</Table.Th>
          <Table.Th>Actions</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((app) => (
          <Table.Tr key={app.name}>
            <Table.Td>{app.name}</Table.Td>
            <Table.Td>{app.version}</Table.Td>
            <Table.Td>
              <LocalAppStatusBadge status={app.status} />
            </Table.Td>
            <Table.Td>
              <Group gap="xs">
                {onDeploy && app.status === LocalAppInstallStatus.Installed && (
                  <Button onClick={() => onDeploy(app)} loading={loading}>
                    Deploy
                  </Button>
                )}
                {onRemoveDeploy &&
                  app.status === LocalAppInstallStatus.StackDeployed && (
                    <Button
                      variant="outline"
                      color="red"
                      onClick={() => onRemoveDeploy(app)}
                      loading={loading}
                    >
                      Remove
                    </Button>
                  )}
                {onRegister &&
                  app.status === LocalAppInstallStatus.StackDeployed && (
                    <Button
                      variant="outline"
                      onClick={() => onRegister(app)}
                      loading={loading}
                    >
                      Register
                    </Button>
                  )}
              </Group>
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
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
