import { Stack, Title, Text, Tabs, Box } from "@mantine/core"
import NewRegionForm from "./NewRegionForm"

import { IconMapPinPlus, IconMapPlus } from "@tabler/icons-react"
import { BootstrapNodeData } from "../../../api/Api"
import { BootstrapNodeForm } from "../../this_p2panda_node"
import { ActionPromiseResult } from "../../../components"

interface SetRegionProps {
  onSubmit: (data: BootstrapNodeData) => Promise<ActionPromiseResult>
}

export default function SetRegion({ onSubmit }: SetRegionProps) {
  return (
    <Stack gap="lg">
      <Stack>
        <Title order={1}>Welcome to LoRes Mesh</Title>
        <Text>
          In order to setup this Node, we need to connect you to a region.
        </Text>
        <Text>
          A Region is a cluster of Nodes that are in regular communication, and
          provide services to users that are redundantly available at many or
          all of the Nodes. This means that a region is generally a geographic
          area that makes sense to humans, such as your neighbourhood, town,
          river catchment, etc.
        </Text>
        <Text>
          You can join an existing region, or create a new one. What would you
          like to do?
        </Text>
      </Stack>
      <Tabs defaultValue="join">
        <Tabs.List>
          <Tabs.Tab value="join" leftSection={<IconMapPinPlus />}>
            Join Region
          </Tabs.Tab>
          <Tabs.Tab value="new" leftSection={<IconMapPlus />}>
            New Region
          </Tabs.Tab>
        </Tabs.List>
        <Tabs.Panel value="join" pt="md">
          <BootstrapNodeForm onSubmit={onSubmit} />
        </Tabs.Panel>
        <Tabs.Panel value="new" pt="md">
          <NewRegionForm onSubmit={onSubmit} />
        </Tabs.Panel>
      </Tabs>
    </Stack>
  )
}
