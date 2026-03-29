import { Card, Container, Group, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import { useParams } from "react-router-dom"
import RegionDetails from "../components/RegionDetails"
import { Anchor } from "../../../components"
import { IfNodeSteward } from "../../auth/node_steward_auth"

export default function ShowRegion() {
  const { regionSlug } = useParams<{ regionSlug: string }>()
  const region = useAppSelector(
    (state) =>
      state.my_regions.all?.find((r) => r.region.slug === regionSlug)?.region,
  )
  const myNodeId = useAppSelector((state) => state.network?.node.id)

  if (!region) return <div>Region not found</div>

  const isCreator = myNodeId && region.creator_node_id === myNodeId

  return (
    <Container>
      <Stack gap="lg">
        <Title order={1}>{region?.name}</Title>
        <Stack gap="xs">
          <Title order={2}>Details</Title>
          <Card>
            <Card.Section>
              <RegionDetails region={region} />
            </Card.Section>
          </Card>
        </Stack>
        {isCreator && (
          <IfNodeSteward>
            <Stack gap="xs">
              <Title order={2}>Admin Actions</Title>
              <Card>
                <Group>
                  <Anchor href="./edit-map">Edit map</Anchor>
                </Group>
              </Card>
            </Stack>
          </IfNodeSteward>
        )}
      </Stack>
    </Container>
  )
}
