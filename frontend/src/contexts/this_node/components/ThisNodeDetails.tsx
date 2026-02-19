import { Table } from "@mantine/core"
import { RegionNode, NodeDetails } from "../../../api/Api"

export default function ThisNodeDetails({
  node,
  nodeDetails,
}: {
  node: RegionNode
  nodeDetails: NodeDetails | null
}) {
  return (
    <Table>
      <Table.Tbody>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Td>{node.name}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>ID</Table.Th>
          <Table.Td>{node.id}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Public IPv4</Table.Th>
          <Table.Td>{nodeDetails?.public_ipv4}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Domain on Local Network</Table.Th>
          <Table.Td>{nodeDetails?.domain_on_local_network}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Domain on Internet</Table.Th>
          <Table.Td>{nodeDetails?.domain_on_internet}</Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  )
}
