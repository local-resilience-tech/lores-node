import { Table } from "@mantine/core"
import { LocalApp } from "../../../api/Api"
import { useLoading } from "../../shared"
import { Anchor } from "../../../components"
import LocalAppStatusBadge from "./LocalAppStatusBadge"

interface AppsListProps {
  apps: LocalApp[]
}

export default function LocalAppsList({ apps }: AppsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Status</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((app) => (
          <LocalAppRow key={app.name} app={app} />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface LocalAppRowProps {
  app: LocalApp
}

function LocalAppRow({ app }: LocalAppRowProps) {
  const [loading, withLoading] = useLoading(false)

  return (
    <Table.Tr key={app.name}>
      <Table.Td>
        <Anchor href={`app/${app.name}`}>{app.name}</Anchor>
      </Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>
        <LocalAppStatusBadge status={app.status} />
      </Table.Td>
    </Table.Tr>
  )
}
