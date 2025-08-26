import { Table } from "@mantine/core"
import { NodeSteward } from "../../../api/Api"

interface NodeStewardsListProps {
  nodeStewards: NodeSteward[]
}

export default function NodeStewardsList({
  nodeStewards,
}: NodeStewardsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Status</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {nodeStewards.map((steward) => (
          <Table.Tr key={steward.id}>
            <Table.Td>{steward.name}</Table.Td>
            <Table.Td>{steward.enabled ? "Enabled" : "Disabled"}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
