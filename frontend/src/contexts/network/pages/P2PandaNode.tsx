import { Stack, Title, Container, Table, Button, Collapse } from "@mantine/core"
import { useAppSelector } from "../../../store"
import { IconPlug } from "@tabler/icons-react"
import { useDisclosure } from "@mantine/hooks"
import AddBootstrapNode from "../components/AddBootstrapNode"

export default function P2PandaNode() {
  const node = useAppSelector((state) => state.network?.node)
  const [openedBootstrap, { toggle: toggleBootstrap }] = useDisclosure(false)

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
        <Collapse in={openedBootstrap}>
          <AddBootstrapNode onSuccess={toggleBootstrap} />
        </Collapse>
      </Stack>
    </Container>
  )
}
