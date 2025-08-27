import { Table } from "@mantine/core"
import { Node } from "../../../api/Api"

export default function ThisNodeDetails({ node }: { node: Node }) {
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
      </Table.Tbody>
    </Table>
  )
}
