import { Table } from "@mantine/core"
import LocalAppStatusBadge from "./LocalAppStatusBadge"
import { LocalApp } from "../../../api/Api"

export default function LocalAppDetails({ app }: { app: LocalApp }) {
  return (
    <Table>
      <Table.Tbody>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Td>{app.name}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Version</Table.Th>
          <Table.Td>{app.version}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Repository</Table.Th>
          <Table.Td>{app.repo_name || "N/A"}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Status</Table.Th>
          <Table.Td>
            <LocalAppStatusBadge status={app.status} />
          </Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  )
}
