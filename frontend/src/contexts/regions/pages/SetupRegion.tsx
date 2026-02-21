import { Container, Stack, Tabs, Text, Title } from "@mantine/core"
import { useAppSelector } from "../../../store"
import { Anchor } from "../../../components"
import { IconMapPinPlus, IconMapPlus } from "@tabler/icons-react"

import CreateRegion from "../components/CreateRegion"
import JoinRegion from "../components/JoinRegion"

export default function SetupRegion() {
  const isNodeSteward = useAppSelector((state) => !!state.me)

  return (
    <Container>
      <Stack gap="lg">
        <Stack>
          <Title order={1}>Join a Region</Title>

          <Text>
            A Region is a cluster of Nodes that are in regular communication,
            and provide services to users that are redundantly available at many
            or all of the Nodes. This means that a region is generally a
            geographic area that makes sense to humans, such as your
            neighbourhood, town, river catchment, etc.
          </Text>
          {isNodeSteward && (
            <Text>
              You can either join an existing region, or create a new one. What
              would you like to do?
            </Text>
          )}
          {!isNodeSteward && (
            <Text c="red">
              Your Node Stewards have not yet assigned this node to a region. If
              you're a Node Steward, please{" "}
              <Anchor href="/auth/node_steward/login">log in</Anchor> and set up
              the region.
            </Text>
          )}
        </Stack>
        {isNodeSteward && (
          <Tabs defaultValue="join">
            <Tabs.List>
              <Tabs.Tab value="join" leftSection={<IconMapPinPlus />}>
                Join Region
              </Tabs.Tab>
              <Tabs.Tab value="new" leftSection={<IconMapPlus />}>
                Create New Region
              </Tabs.Tab>
            </Tabs.List>
            <Tabs.Panel value="join" pt="md">
              <JoinRegion />
            </Tabs.Panel>
            <Tabs.Panel value="new" pt="md">
              <CreateRegion />
            </Tabs.Panel>
          </Tabs>
        )}
      </Stack>
    </Container>
  )
}
