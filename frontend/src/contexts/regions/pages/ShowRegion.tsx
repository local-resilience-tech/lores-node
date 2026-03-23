import { Card, Container, Stack, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import { useParams } from "react-router-dom"
import RegionDetails from "../components/RegionDetails"

export default function ShowRegion() {
  const { regionSlug } = useParams<{ regionSlug: string }>()
  const region = useAppSelector(
    (state) =>
      state.my_regions.all?.find((r) => r.region.slug === regionSlug)?.region,
  )

  return (
    <Container>
      <Stack>
        <Title order={1}>{region?.name}</Title>
        <Stack gap="xs">
          <Title order={2}>Details</Title>
          <Card>
            <Card.Section>
              <RegionDetails region={region} />
            </Card.Section>
          </Card>
        </Stack>
      </Stack>
    </Container>
  )
}
