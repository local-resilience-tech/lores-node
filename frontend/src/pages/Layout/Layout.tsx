import {
  Anchor,
  AppShell,
  Badge,
  Breadcrumbs,
  Burger,
  Container,
  Group,
  Text,
  Title,
} from "@mantine/core"
import { NavLink } from "../../components"
import { Outlet } from "react-router-dom"
import { useDisclosure } from "@mantine/hooks"
import {
  IconAffiliate,
  IconApps,
  IconBrandGithub,
  IconHome,
  IconTimelineEventText,
} from "@tabler/icons-react"
import packageJson from "../../../package.json"
import pangaLogoUrl from "../../assets/deepsea-panda.svg"

import classes from "./Layout.module.css"
import { handleClientEvent, useAppSelector } from "../../store"
import useWebSocket from "react-use-websocket"
import { getSocketUrl } from "../../api"

export default function Layout() {
  const [opened, { toggle }] = useDisclosure()
  const iconSize = 20

  const region = useAppSelector((state) => state.region)
  const node = useAppSelector((state) => state.thisNode)
  const nodesCount = useAppSelector((state) => state.nodes?.length)
  const localAppsCount = useAppSelector((state) => state.localApps?.length)

  const {} = useWebSocket(getSocketUrl(), {
    share: true,
    onOpen: (event) => {
      console.log("WebSocket connection opened", event)
    },
    onClose: () => {
      console.log("WebSocket connection closed")
    },
    onMessage: (event) => {
      console.log("WebSocket message received", event)

      handleClientEvent(JSON.parse(event.data))
    },
    heartbeat: false,
  })

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{ width: 300, breakpoint: "sm", collapsed: { mobile: !opened } }}
      padding="md"
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
          <Anchor href="/">LoRes Mesh</Anchor>
          <Breadcrumbs>
            {region && <Text>{region.network_id}</Text>}
            {node && <Text>{node.name}</Text>}
          </Breadcrumbs>
        </Group>
      </AppShell.Header>
      <AppShell.Navbar p={0}>
        <AppShell.Section className={classes.menu_section}>
          <Text className={classes.section_title}>
            {node?.name || "This Node"}
          </Text>

          <NavLink
            label="Node status"
            href="/this_node"
            key={node ? node.id : "this_node"}
            leftSection={<IconHome size={iconSize} />}
            onClick={toggle}
          />
          <NavLink
            label="Local apps"
            href="/this_node/apps"
            leftSection={<IconApps size={iconSize} />}
            onClick={toggle}
            rightSection={
              localAppsCount !== undefined && (
                <Badge circle>{localAppsCount}</Badge>
              )
            }
          />
        </AppShell.Section>

        {region && (
          <AppShell.Section className={classes.menu_section}>
            <Text className={classes.section_title}>
              {region?.network_id || "This Region"}
            </Text>
            <NavLink
              label="Nodes"
              href="/this_region/nodes"
              leftSection={<IconAffiliate size={iconSize} />}
              rightSection={
                nodesCount !== undefined && <Badge circle>{nodesCount}</Badge>
              }
              onClick={toggle}
            />
            <NavLink
              label="All apps"
              href="/this_region/apps"
              leftSection={<IconApps size={iconSize} />}
              onClick={toggle}
            />
          </AppShell.Section>
        )}

        <AppShell.Section className={classes.footer_section}>
          <Text className={classes.section_title}>Debug</Text>
          <NavLink
            label="P2Panda node"
            href="/debug/p2panda_node"
            leftSection={
              <img src={pangaLogoUrl} alt="P2Panda Icon" width={iconSize} />
            }
            onClick={toggle}
          />
          <NavLink
            label="Event log"
            href="/debug/event_log"
            leftSection={<IconTimelineEventText size={iconSize} />}
            onClick={toggle}
          />
          <NavLink
            c="dimmed"
            label={"v" + packageJson.version}
            href={
              packageJson.homepage + "/releases/tag/v" + packageJson.version
            }
            leftSection={<IconBrandGithub size={18} />}
            onClick={toggle}
          />
        </AppShell.Section>
      </AppShell.Navbar>
      <AppShell.Main>
        <Container p={0}>
          <Outlet />
        </Container>
      </AppShell.Main>
    </AppShell>
  )

  // return (
  //   <Stack h="100vh" w="100vw">
  //     <Navbar />
  //     <Container padding={4} centerContent>
  //       <Outlet />
  //     </Container>
  //   </Stack>
  // )
}
