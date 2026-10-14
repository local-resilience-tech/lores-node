import { Stack, Title, Container, Button, Collapse, Table, Text } from "@mantine/core"
import { useAppSelector } from "../../../store"
import { IconAddressBookOff, IconPlug } from "@tabler/icons-react"
import { useDisclosure } from "@mantine/hooks"
import AddBootstrapNode from "../components/AddBootstrapNode"
import NodeConnectionStatus from "../components/NodeConnectionStatus"
import { useEffect, useState } from "react"
import { getApi } from "../../../api"
import { NodeStatusResponse } from "../../../api/Api"
import { IfNodeSteward } from "../../auth/node_steward_auth"
import { ActionButton, ActionPromiseResult } from "../../../components"
import { awaitConfirmModal } from "../../shared"
import { actionCancelled, actionFailure, actionSuccess } from "../../../components/ActionResult"

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

  const onClearAddressBook = async (): Promise<ActionPromiseResult> => {
    const confirmed = await awaitConfirmModal(
      "Clear address book",
      <Text size="sm">
        Are you sure you want to clear the node address book? It will attempt to re-create it from the available bootstrap nodes.
      </Text>,
    )
    if (!confirmed) return actionCancelled()

    console.log("clearing address book")

    return getApi()
      .nodeStewardApi.clearAddressBook()
      .then(() => {
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  // const restartNode = () => async () => {
  //   console.log("restarting node")
  //   await api.restart()
  //   fetchNode()
  // }

  return (
    <Container>
      <Stack align="flex-start">
        <Title order={1}>This P2Panda node</Title>
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
        <IfNodeSteward>
          <Button variant="outline" leftSection={<IconPlug />} onClick={toggleBootstrap}>
            Add bootstrap node
          </Button>
          <ActionButton variant="outline" color="red" leftSection={<IconAddressBookOff />} onClick={onClearAddressBook}>
            Clear address book
          </ActionButton>
        </IfNodeSteward>
        <Collapse expanded={openedBootstrap}>
          <AddBootstrapNode onSuccess={toggleBootstrap} />
        </Collapse>

        <Title order={2}>Connection Status</Title>
        <NodeConnectionStatus nodeStatus={nodeStatus} />
      </Stack>
    </Container>
  )
}
