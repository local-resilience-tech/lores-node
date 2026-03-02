import { Stack, Title, Container, Table, Box } from "@mantine/core"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import type { P2PandaNodeDetails } from "../../../api/Api"
import { useAppSelector } from "../../../store"

export default function P2PandaNode() {
  const node = useAppSelector((state) => state.network?.node)

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
                <pre>{node.id}</pre>
              </Table.Td>
            </Table.Tr>
          </Table.Tbody>
        </Table>
      </Stack>
    </Container>
  )
}
