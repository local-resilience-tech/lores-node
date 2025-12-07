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
          <Table.Th>Status</Table.Th>
          <Table.Td>
            <LocalAppStatusBadge status={app.status} />
          </Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Internet URL</Table.Th>
          <Table.Td>
            {app.url && app.url.internet_url ? (
              <a href={app.url.internet_url}>{app.url.internet_url}</a>
            ) : (
              "not set"
            )}
          </Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Local Network URL</Table.Th>
          <Table.Td>
            {app.url && app.url.local_network_url ? (
              <a href={app.url.local_network_url}>
                {app.url.local_network_url}
              </a>
            ) : (
              "not set"
            )}
          </Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  )
}
