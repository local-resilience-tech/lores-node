import { Button, Table } from "@mantine/core"
import { App } from "../../../api/Api"

interface AppsListProps {
  apps: App[]
  onRegister?: (app: App) => void
}

export default function AppsList({ apps, onRegister }: AppsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((app) => (
          <Table.Tr key={app.name}>
            <Table.Td>{app.name}</Table.Td>
            <Table.Td>{app.version}</Table.Td>
            <Table.Td>
              {onRegister && (
                <Button variant="outline" onClick={() => onRegister(app)}>
                  Register
                </Button>
              )}
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
