import { Table } from "@mantine/core"
import { RegionApp } from "../../../api/Api"

interface AppsListProps {
  apps: RegionApp[]
}

export default function RegionAppsList({ apps }: AppsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((app) => (
          <Table.Tr key={app.name}>
            <Table.Td>{app.name}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
