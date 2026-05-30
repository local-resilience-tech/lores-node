import { Stack, Title, Container, Button, Collapse, Table } from "@mantine/core"
import { useAppSelector } from "../../../store"
import { IconPlug } from "@tabler/icons-react"
import { useDisclosure } from "@mantine/hooks"
import AddBootstrapNode from "../components/AddBootstrapNode"
import NodeConnectionStatus from "../components/NodeConnectionStatus"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import { NodeStatusResponse } from "../../../api/Api"

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
        <NodeConnectionStatus nodeStatus={nodeStatus} />
      </Stack>
    </Container>
  )
}
