import { Group, Stack, Table, Text, Title, Tooltip } from "@mantine/core"
import { OperationCountEntry, Region } from "../../../api/Api"

function HexId({ value }: { value: string }) {
  return (
    <Tooltip label={value}>
      <Text ff="monospace" size="md" style={{ cursor: "default" }}>
        {value.slice(0, 8)}
      </Text>
    </Tooltip>
  )
}

interface Props {
  counts: OperationCountEntry[]
  regions?: Region[]
  myNodeId?: string | null
}

export default function TopicCountsTable({ counts, regions, myNodeId }: Props) {
  const regionById = Object.fromEntries((regions ?? []).map((r) => [r.id, r]))
  if (counts.length === 0) {
    return (
      <Text c="dimmed" size="sm">
        No operations in the store.
      </Text>
    )
  }

  const byTopic = counts.reduce<Record<string, OperationCountEntry[]>>(
    (acc, entry) => {
      ;(acc[entry.topic] ??= []).push(entry)
      return acc
    },
    {},
  )

  return (
    <Stack gap="lg">
      {Object.entries(byTopic).map(([topic, entries]) => (
        <Stack key={topic} gap="xs">
          <Group>
            <Title order={4}>
              {regionById[topic]?.name ? (
                <Tooltip label={topic}>
                  <span style={{ cursor: "default" }}>
                    {regionById[topic].name}
                  </span>
                </Tooltip>
              ) : (
                <HexId value={topic} />
              )}
            </Title>
          </Group>
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Author node</Table.Th>
                <Table.Th>Count</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {[...entries]
                .sort((a, b) =>
                  a.author_node_id === myNodeId
                    ? -1
                    : b.author_node_id === myNodeId
                      ? 1
                      : 0,
                )
                .map((entry, i) => {
                  const isMe = entry.author_node_id === myNodeId
                  return (
                    <Table.Tr key={i} bg={isMe ? "blue.8" : undefined}>
                      <Table.Td>
                        <HexId value={entry.author_node_id} />
                      </Table.Td>
                      <Table.Td>{entry.count}</Table.Td>
                    </Table.Tr>
                  )
                })}
            </Table.Tbody>
          </Table>
        </Stack>
      ))}
    </Stack>
  )
}
