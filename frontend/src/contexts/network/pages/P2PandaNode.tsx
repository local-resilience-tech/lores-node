import {
  Stack,
  Title,
  Container,
  Table,
  Button,
  Collapse,
  Text,
  Badge,
} from "@mantine/core"
import { useAppSelector } from "../../../store"
import { IconPlug } from "@tabler/icons-react"
import { useDisclosure } from "@mantine/hooks"
import AddBootstrapNode from "../components/AddBootstrapNode"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import { NodeStatusResponse, PeerConnectionStatus } from "../../../api/Api"

export default function P2PandaNode() {
  const node = useAppSelector((state) => state.network?.node)
  const [openedBootstrap, { toggle: toggleBootstrap }] = useDisclosure(false)
  const [nodeStatus, setNodeStatus] = useState<NodeStatusResponse | null>(null)

  useEffect(() => {
    getApi()
      .publicApi.nodeStatus()
      .then((response) => setNodeStatus(response.data))
      .catch(console.error)
  }, [])

  if (!node) {
    return <></>
  }

  // const restartNode = () => async () => {
  //   console.log("restarting node")
  //   await api.restart()
  //   fetchNode()
  // }

  return (
    <Container>
      <Stack align="flex-start">
        <Title order={1}>This P2Panda Node</Title>
        {/* <Box mb={4}>
        <Button onClick={restartNode()}>Restart Node</Button>
      </Box> */}
        <Table>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Key</Table.Th>
              <Table.Th>Value</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            <Table.Tr>
              <Table.Td>Panda Node Id</Table.Td>
              <Table.Td>
                <pre>{node.id}</pre>
              </Table.Td>
            </Table.Tr>
          </Table.Tbody>
        </Table>
        <Button
          variant="outline"
          leftSection={<IconPlug />}
          onClick={toggleBootstrap}
        >
          Add bootstrap node
        </Button>
        <Collapse expanded={openedBootstrap}>
          <AddBootstrapNode onSuccess={toggleBootstrap} />
        </Collapse>

        <Title order={2}>Connection Status</Title>
        {nodeStatus === null && <Text c="dimmed">Loading...</Text>}
        {nodeStatus !== null && nodeStatus.topics.length === 0 && (
          <Text c="dimmed">No active topic subscriptions.</Text>
        )}
        {nodeStatus !== null &&
          nodeStatus.topics.map((topic) => (
            <Stack key={topic.topic_hex} gap="xs">
              <Text size="sm" fw={600} ff="monospace">
                {topic.topic_hex}
              </Text>
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
                          <pre style={{ margin: 0 }}>{conn.node_id}</pre>
                        </Table.Td>
                        <Table.Td>
                          <Badge
                            color={
                              conn.status === PeerConnectionStatus.Syncing
                                ? "blue"
                                : "gray"
                            }
                          >
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
    </Container>
  )
}
