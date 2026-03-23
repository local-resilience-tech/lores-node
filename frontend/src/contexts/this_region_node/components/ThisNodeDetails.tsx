import { Table } from "@mantine/core"
import { RegionNodeDetails } from "../../../api/Api"

export default function ThisNodeDetails({
  node,
}: {
  node: RegionNodeDetails | null
}) {
  return (
    <Table>
      <Table.Tbody>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Td>{node?.name}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>ID</Table.Th>
          <Table.Td>{node?.node_id}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Public IPv4</Table.Th>
          <Table.Td>{node?.public_ipv4}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Domain on Local Network</Table.Th>
          <Table.Td>{node?.domain_on_local_network}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Domain on Internet</Table.Th>
          <Table.Td>{node?.domain_on_internet}</Table.Td>
        </Table.Tr>
        {node?.latlng && (
          <Table.Tr>
            <Table.Th>Location</Table.Th>
            <Table.Td>
              {node.latlng.lat}, {node.latlng.lng}
            </Table.Td>
          </Table.Tr>
        )}
      </Table.Tbody>
    </Table>
  )
}
