import { Stack, Title, Container, Table, Box } from "@mantine/core"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import type { P2PandaNodeDetails } from "../../../api/Api"

const getNode = async (): Promise<P2PandaNodeDetails | null> => {
  const result = await getApi().publicApi.showThisPandaNode()

  if (result.status !== 200) {
    console.warn("Network not started", result)
    return null
  }

  return result.data
}

export default function ThisP2PandaNode() {
  const [node, setNode] = useState<P2PandaNodeDetails | null>(null)

  const fetchNode = async () => {
    const node = await getNode()
    console.log("fetched node", node)
    setNode(node)
  }

  useEffect(() => {
    fetchNode()
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
      <Stack>
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
                <pre>{node.panda_node_id}</pre>
              </Table.Td>
            </Table.Tr>
          </Table.Tbody>
        </Table>
      </Stack>
    </Container>
  )
}
