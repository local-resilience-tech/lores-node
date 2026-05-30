import { Stack, Text, Table, Badge } from "@mantine/core"
import { NodeStatusResponse, PeerConnectionStatus } from "../../../api/Api"
import TruncatedId from "./TruncatedId"

const peerStatusColor: Record<PeerConnectionStatus, string> = {
  [PeerConnectionStatus.Connected]: "green",
  [PeerConnectionStatus.Syncing]: "blue",
  [PeerConnectionStatus.SyncFailed]: "red",
  [PeerConnectionStatus.Unknown]: "gray",
}

interface Props {
  nodeStatus: NodeStatusResponse | null
}

export default function NodeConnectionStatus({ nodeStatus }: Props) {
  if (nodeStatus === null) {
    return <Text c="dimmed">Loading...</Text>
  }

  if (nodeStatus.topics.length === 0) {
    return <Text c="dimmed">No active topic subscriptions.</Text>
  }

  return (
    <Stack>
      {nodeStatus.topics.map((topic) => (
        <Stack key={topic.topic_hex} gap="xs">
          <div>
            Topic: <TruncatedId id={topic.topic_hex} />
          </div>
          {topic.connections.length === 0 ? (
            <Text size="sm" c="dimmed">
              No peers seen yet.
            </Text>
          ) : (
            <Table withColumnBorders withTableBorder>
              <Table.Thead>
                <Table.Tr>
                  <Table.Th>Peer Node ID</Table.Th>
                  <Table.Th>Status</Table.Th>
                </Table.Tr>
              </Table.Thead>
              <Table.Tbody>
                {topic.connections.map((conn) => (
                  <Table.Tr key={conn.node_id}>
                    <Table.Td>
                      <TruncatedId id={conn.node_id} />
                    </Table.Td>
                    <Table.Td>
                      <Badge color={peerStatusColor[conn.status]}>
                        {conn.status}
                      </Badge>
                    </Table.Td>
                  </Table.Tr>
                ))}
              </Table.Tbody>
            </Table>
          )}
        </Stack>
      ))}
    </Stack>
  )
}
