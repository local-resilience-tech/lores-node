import { Table } from "@mantine/core"
import { LocalAppInstallation } from "../../../api/Api"
import { Anchor } from "../../../components"

interface AppsListProps {
  apps: LocalAppInstallation[]
}

export default function LocalAppsList({ apps }: AppsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Region</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((installation) => (
          <LocalAppRow
            key={installation.app.name}
            installation={installation}
          />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface LocalAppRowProps {
  installation: LocalAppInstallation
}

function LocalAppRow({ installation }: LocalAppRowProps) {
  const { app } = installation
  return (
    <Table.Tr key={app.name}>
      <Table.Td>
        <Anchor href={`app/${app.name}`}>{app.name}</Anchor>
      </Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>{installation.region_id ?? "N/A"}</Table.Td>
    </Table.Tr>
  )
}
