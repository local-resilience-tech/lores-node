import { Stack, Card, Text, Box, Table } from "@mantine/core"
import { NodeDetailsWithStatus } from "../../this_node"
import { Anchor } from "../../../components"

const IpLink = ({ ip }: { ip: string | undefined | null }) => {
  if (!ip) return <Text c="dimmed">unknown</Text>

  return (
    <Anchor href={`https://${ip}`} newWindow>
      {ip}
    </Anchor>
  )
}

export default function NodesList({
  nodes: nodes,
}: {
  nodes: NodeDetailsWithStatus[]
}) {
  return (
    <Stack>
      {nodes.map((node) => (
        <Card key={node.id} withBorder>
          <Stack>
            <Box>
              <Text fw={500}>{node.name}</Text>
              <Text size="xs" ff="mono">
                {node.id}
              </Text>
            </Box>
            <Table variant="vertical" layout="fixed" withTableBorder>
              <Table.Tbody>
                <Table.Tr>
                  <Table.Th w={160}>Message</Table.Th>
                  <Table.Td>{node.status_text}</Table.Td>
                </Table.Tr>

                <Table.Tr>
                  <Table.Th>IP</Table.Th>
                  <Table.Td>
                    <IpLink ip={node.public_ipv4} />
                  </Table.Td>
                </Table.Tr>

                <Table.Tr>
                  <Table.Th>State</Table.Th>
                  <Table.Td>{node.state || "unknown"}</Table.Td>
                </Table.Tr>
              </Table.Tbody>
            </Table>
          </Stack>
        </Card>
      ))}
    </Stack>
  )
}
