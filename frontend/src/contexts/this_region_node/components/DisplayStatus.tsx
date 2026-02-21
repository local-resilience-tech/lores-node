import { Table } from "@mantine/core"

interface DisplayStatusProps {
  state?: string | null
  status_text?: string | null
}

export default function ThisNodeDetails({
  state,
  status_text,
}: DisplayStatusProps) {
  return (
    <Table>
      <Table.Tbody>
        <Table.Tr>
          <Table.Th>State</Table.Th>
          <Table.Td>{state}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Message</Table.Th>
          <Table.Td>{status_text}</Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  )
}
