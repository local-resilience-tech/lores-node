import { Badge, Button, Group, Table } from "@mantine/core"
import { LocalApp, LocalAppInstallStatus } from "../../../api/Api"
import { IconBrandDocker, IconDatabase } from "@tabler/icons-react"

interface AppsListProps {
  apps: LocalApp[]
  onDeploy?: (app: LocalApp) => void
  onRemoveDeploy?: (app: LocalApp) => void
  onRegister?: (app: LocalApp) => void
}

export default function LocalAppsList({
  apps,
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
                  <Button onClick={() => onDeploy(app)}>Deploy</Button>
                )}
                {onRemoveDeploy &&
                  app.status === LocalAppInstallStatus.StackDeployed && (
                    <Button
                      variant="outline"
                      color="red"
                      onClick={() => onRemoveDeploy(app)}
                    >
                      Remove
                    </Button>
                  )}
                {onRegister &&
                  app.status === LocalAppInstallStatus.StackDeployed && (
                    <Button variant="outline" onClick={() => onRegister(app)}>
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
