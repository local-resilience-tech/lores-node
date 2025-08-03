import { Button, Group, Table } from "@mantine/core"
import { LocalApp } from "../../../api/Api"

interface AppsListProps {
  apps: LocalApp[]
  onStart?: (app: LocalApp) => void
  onRegister?: (app: LocalApp) => void
}

export default function LocalAppsList({
  apps,
  onStart,
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
            <Table.Td>{app.status}</Table.Td>
            <Table.Td>
              <Group gap="xs">
                {onStart && <Button onClick={() => onStart(app)}>Start</Button>}
                {onRegister && (
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
