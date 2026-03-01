import { Stack, Card, Text, Box, Table, useMantineTheme } from "@mantine/core"
import { Anchor } from "../../../components"
import { RegionNodeDetails } from "../../../api/Api"

const IpLink = ({ ip }: { ip: string | undefined | null }) => {
  if (!ip) return <Text c="dimmed">unknown</Text>

  return (
    <Anchor href={`https://${ip}`} newWindow>
      {ip}
    </Anchor>
  )
}

interface NodeCardProps {
  node: RegionNodeDetails
}

export default function NodeCard({ node }: NodeCardProps) {
  const theme = useMantineTheme()

  return (
    <Card key={node.id} withBorder>
      <Stack>
        <Box>
          <Text fw={500}>{node.name}</Text>
          <Text size="sm" ff="mono">
            {nodeName(node)}
          </Text>
        </Box>
        <Card.Section>
          <Table layout="fixed" bgcolor={theme.colors.dark[7]}>
            <Table.Tbody>
              <Table.Tr>
                <Table.Th w={160}>ID</Table.Th>
                <Table.Td>{node.node_id}</Table.Td>
              </Table.Tr>
              <Table.Tr>
                <Table.Th w={160}>Message</Table.Th>
                <Table.Td>{node.status_text}</Table.Td>
              </Table.Tr>
              {/*
                  <Table.Tr>
                    <Table.Th>IP</Table.Th>
                    <Table.Td>
                      <IpLink ip={node.public_ipv4} />
                    </Table.Td>
                  </Table.Tr> */}

              <Table.Tr>
                <Table.Th>State</Table.Th>
                <Table.Td>{node.state || "unknown"}</Table.Td>
              </Table.Tr>
            </Table.Tbody>
          </Table>
        </Card.Section>
      </Stack>
    </Card>
  )
}

function nodeName(node: RegionNodeDetails) {
  return node.name ?? node.node_id.slice(0, 12) + "..."
}
