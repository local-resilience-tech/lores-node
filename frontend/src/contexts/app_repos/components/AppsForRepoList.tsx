import { Table, Text } from "@mantine/core"
import AppBadge from "./AppBadge"
import { AppDefinition } from "../../../api/Api"

export default function AppsForRepoList({ apps }: { apps: AppDefinition[] }) {
  if (!apps || apps.length === 0) {
    return <Text c="dimmed">No apps found in this repository.</Text>
  }

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>App Name</Table.Th>
          <Table.Th>Versions</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((app) => (
          <Table.Tr key={app.name}>
            <Table.Th>{app.name}</Table.Th>
            <Table.Td>
              {app.versions.map((version) => (
                <Text size="sm" key={version}>
                  {version}
                </Text>
              ))}
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
