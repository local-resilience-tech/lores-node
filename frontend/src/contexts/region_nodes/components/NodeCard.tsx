import {
  Stack,
  Card,
  Text,
  Box,
  Table,
  useMantineTheme,
  Group,
  Badge,
} from "@mantine/core"
import { Anchor } from "../../../components"
import { RegionNodeDetails } from "../../../api/Api"
import { nodeName } from "../../../store/my_regions"

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
  isRegionCreator?: boolean
}

export default function NodeCard({ node, isRegionCreator }: NodeCardProps) {
  const theme = useMantineTheme()

  return (
    <Card key={node.id} withBorder>
      <Stack>
        <Group justify="space-between">
          <Text fw={500}>{nodeName(node)}</Text>
          {isRegionCreator && <Badge>Admin</Badge>}
        </Group>
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
