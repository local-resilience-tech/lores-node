import { Badge, Table, Text } from "@mantine/core"
import { NodeSteward, NodeStewardStatus } from "../../../api/Api"
import DateText from "../../../components/DateText"
import NodeStewardActions, { NodeStewardAction } from "./NodeStewardAction"

function NodeStewardStatusIndicator({ status }: { status: NodeStewardStatus }) {
  switch (status) {
    case NodeStewardStatus.Enabled:
      return <Badge color="green">Enabled</Badge>
    case NodeStewardStatus.Disabled:
      return <Badge color="red">Disabled</Badge>
    case NodeStewardStatus.Invited:
      return <Badge color="blue">Invited</Badge>
    case NodeStewardStatus.TokenExpired:
      return <Badge color="orange">Token Expired</Badge>
  }
}

interface NodeStewardsListProps {
  nodeStewards: NodeSteward[]
  getActions: (record: NodeSteward) => NodeStewardAction[]
}

export default function NodeStewardsList({
  nodeStewards,
  getActions,
}: NodeStewardsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Created</Table.Th>
          <Table.Th>Status</Table.Th>
          <Table.Th>Actions</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {nodeStewards.map((steward) => (
          <Table.Tr key={steward.id}>
            <Table.Td>
              <Text span>{steward.name}</Text>
            </Table.Td>
            <Table.Td>
              <DateText date={steward.created_at} />
            </Table.Td>
            <Table.Td>
              <NodeStewardStatusIndicator status={steward.status} />
            </Table.Td>
            <Table.Td>
              <NodeStewardActions
                actions={getActions(steward)}
                record={steward}
              />
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
