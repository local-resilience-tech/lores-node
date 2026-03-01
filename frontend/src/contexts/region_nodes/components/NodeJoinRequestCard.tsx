import { Stack, Card, Text, Group, Table, useMantineTheme } from "@mantine/core"
import { ActionButton, ActionPromiseResult } from "../../../components"
import { RegionNodeDetails } from "../../../api/Api"
import TextWithNewlines from "../../../components/TextWithNewlines"

// const IpLink = ({ ip }: { ip: string | undefined | null }) => {
//   if (!ip) return <Text c="dimmed">unknown</Text>

//   return (
//     <Anchor href={`https://${ip}`} newWindow>
//       {ip}
//     </Anchor>
//   )
// }

interface NodeJoinRequestCardProps {
  node: RegionNodeDetails
  onApprove?: (node: RegionNodeDetails) => Promise<ActionPromiseResult>
}

export default function NodeJoinRequestCard({
  node,
  onApprove,
}: NodeJoinRequestCardProps) {
  const theme = useMantineTheme()
  const highlightColor = theme.colors.orange[6]

  return (
    <Card
      key={node.id}
      styles={{ root: { borderColor: highlightColor } }}
      withBorder
    >
      <Stack>
        <Text fw="bold" c={highlightColor}>
          Join Request
        </Text>
        <Card.Section>
          <Table layout="fixed" bgcolor={theme.colors.dark[7]}>
            <Table.Tbody>
              <Table.Tr>
                <Table.Th w={160}>ID</Table.Th>
                <Table.Td>{node.node_id}</Table.Td>
              </Table.Tr>
              <Table.Tr>
                <Table.Th w={160}>About</Table.Th>
                <Table.Td>
                  <TextWithNewlines text={node.about_your_node} />
                </Table.Td>
              </Table.Tr>
              <Table.Tr>
                <Table.Th w={160}>Stewards</Table.Th>
                <Table.Td>
                  <TextWithNewlines text={node.about_your_stewards} />
                </Table.Td>
              </Table.Tr>
              <Table.Tr>
                <Table.Th w={160}>Agreed Conduct URL</Table.Th>
                <Table.Td>{node.agreed_node_steward_conduct_url}</Table.Td>
              </Table.Tr>
            </Table.Tbody>
          </Table>
        </Card.Section>
        {onApprove && (
          <Group justify="flex-end" gap="md">
            <ActionButton onClick={() => onApprove(node)}>Approve</ActionButton>
          </Group>
        )}
      </Stack>
    </Card>
  )
}
