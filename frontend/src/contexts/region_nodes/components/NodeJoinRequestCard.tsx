import { Stack, Card, Text, Box, Table, useMantineTheme } from "@mantine/core"
import { Anchor } from "../../../components"
import { RegionNodeDetails } from "../../../api/Api"
import TextWithNewlines from "../../../components/TextWithNewlines"

const IpLink = ({ ip }: { ip: string | undefined | null }) => {
  if (!ip) return <Text c="dimmed">unknown</Text>

  return (
    <Anchor href={`https://${ip}`} newWindow>
      {ip}
    </Anchor>
  )
}

interface NodeJoinRequestCardProps {
  node: RegionNodeDetails
}

export default function NodeJoinRequestCard({
  node,
}: NodeJoinRequestCardProps) {
  const theme = useMantineTheme()
  const highlightColor = theme.colors.orange[6]

  let temp_about_your_node =
    "Dolorem et beatae temporibus est. Excepturi officiis qui molestias maiores reiciendis minima et. Tempora eos ut tempore est. Quae adipisci quod quis dolor. Blanditiis ab voluptatem officiis tempora.\n\nCulpa molestiae sit et. Omnis accusantium et eos quaerat sit ipsam nisi nihil. Autem quibusdam aut eligendi. Non nemo ut commodi adipisci porro. Aut quam inventore neque veritatis."

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
                  <TextWithNewlines text={temp_about_your_node} />
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
      </Stack>
    </Card>
  )
}
