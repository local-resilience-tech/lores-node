import {
  AppShell,
  Avatar,
  Badge,
  Breadcrumbs,
  Burger,
  Container,
  Group,
  Text,
} from "@mantine/core"
import { Anchor, NavLink } from "../../components"
import { Outlet } from "react-router-dom"
import { useDisclosure } from "@mantine/hooks"
import {
  IconAffiliate,
  IconApps,
  IconBrandDocker,
  IconBrandGit,
  IconBrandGithub,
  IconGhost,
  IconHome,
  IconTimelineEventText,
  IconUser,
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
  const regionNode = useAppSelector((state) => state.thisRegionNode)
  const nodesCount = useAppSelector((state) => state.nodes?.length)
  const localAppsCount = useAppSelector((state) => state.localApps?.length)
  const me = useAppSelector((state) => state.me)

  const readyForApps = region && regionNode
  const pandaRunning = region

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
            {regionNode && <Text>{regionNode.name}</Text>}
          </Breadcrumbs>
        </Group>
      </AppShell.Header>
      <AppShell.Navbar p={0}>
        <AppShell.Section className={classes.user_section}>
          {me ? (
            <Group justify="center" gap="sm">
              <Avatar>
                <IconUser size={24} />
              </Avatar>
              <Text fw="bold">{me.name}</Text>
            </Group>
          ) : (
            <Group justify="center" gap="sm">
              <Avatar>
                <IconGhost size={24} />
              </Avatar>
              <Text fw="bold">
                Guest - <Anchor href="/auth/node_steward/login">log in</Anchor>
              </Text>
            </Group>
          )}
        </AppShell.Section>
        <AppShell.Section className={classes.menu_section}>
          <Text className={classes.section_title}>
            {regionNode?.name ? (
              <>
                <Text span c="dimmed">
                  Node:{" "}
                </Text>
                <Text span>{regionNode.name}</Text>
              </>
            ) : (
              "This Node"
            )}
          </Text>

          <NavLink
            label="This node"
            href="/this_region_node"
            key={regionNode ? regionNode.id : "this_region_node"}
            leftSection={<IconHome size={iconSize} />}
            onClick={toggle}
          />
          {readyForApps && (
            <>
              <NavLink
                label="Local apps"
                href="/this_region_node/apps"
                leftSection={<IconApps size={iconSize} />}
                onClick={toggle}
                rightSection={
                  localAppsCount !== undefined && (
                    <Badge circle>{localAppsCount}</Badge>
                  )
                }
              />
            </>
          )}
        </AppShell.Section>

        {region && (
          <AppShell.Section className={classes.menu_section}>
            <Text className={classes.section_title}>
              {region?.network_id ? (
                <>
                  <Text span c="dimmed">
                    Region:{" "}
                  </Text>
                  <Text span>{region.network_id}</Text>
                </>
              ) : (
                "This Region"
              )}
            </Text>
            <NavLink
              label="Nodes"
              href="/this_region/nodes"
              leftSection={<IconAffiliate size={iconSize} />}
              rightSection={
                nodesCount !== undefined &&
                nodesCount > 0 && (
                  <Badge circle color={nodesCount > 1 ? "blue" : "gray"}>
                    {nodesCount}
                  </Badge>
                )
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

        {pandaRunning && (
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
            {readyForApps && (
              <NavLink
                label="Docker stacks"
                href="/debug/stacks"
                leftSection={<IconBrandDocker size={iconSize} />}
                onClick={toggle}
              />
            )}
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
        )}
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
