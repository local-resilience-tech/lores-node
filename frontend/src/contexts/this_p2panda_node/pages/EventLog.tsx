import {
  ActionIcon,
  Container,
  Group,
  Stack,
  Table,
  Title,
  Text,
} from "@mantine/core"
import { IconRefresh } from "@tabler/icons-react"
import { getApi } from "../../../api"
import { useEffect, useState } from "react"
import { P2PandaLogCounts } from "../../../api/Api"
import { useAppSelector } from "../../../store"
import { hashById, NodesMap } from "../../../store/nodes"

export default function EventLog() {
  const [logCounts, setLogCounts] = useState<null | P2PandaLogCounts>(null)
  const nodesHash = hashById(useAppSelector((state) => state.nodes))

  const loadEventLog = async () => {
    getApi()
      .publicApi.p2PandaLogCounts()
      .then((response) => {
        console.log("Event log loaded:", response.data)
        setLogCounts(response.data)
      })
      .catch((error) => {
        console.error("Error loading event log:", error)
      })
  }

  const nodeName = (nodeId: string) => {
    if (!nodesHash) return null
    return nodesHash.get(nodeId)?.name
  }

  useEffect(() => {
    loadEventLog()
  }, [])

  return (
    <Container>
      <Stack>
        <Group justify="space-between">
          <Title order={1}>P2Panda Event Log</Title>
          <ActionIcon onClick={loadEventLog}>
            <IconRefresh />
          </ActionIcon>
        </Group>
        {logCounts && (
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Node</Table.Th>
                <Table.Th>Total Events</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {logCounts.counts.map((logCount) => (
                <Table.Tr key={logCount.node_id}>
                  <Table.Td>
                    <Stack gap={0}>
                      <Text size="md" fw={600} span>
                        {nodeName(logCount.node_id) || "Unknown Node"}
                      </Text>
                      <Text size="xs" span c="dimmed">
                        {logCount.node_id}
                      </Text>
                    </Stack>
                  </Table.Td>
                  <Table.Td>{logCount.total}</Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        )}
      </Stack>
    </Container>
  )
}
