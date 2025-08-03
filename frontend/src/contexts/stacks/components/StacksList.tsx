import { Table } from "@mantine/core"
import { DockerStack } from "../../../api/Api"

interface StacksListProps {
  stacks: DockerStack[]
}

export default function StacksList({ stacks }: StacksListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {stacks.map((stack) => (
          <Table.Tr key={stack.name}>
            <Table.Td>{stack.name}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
