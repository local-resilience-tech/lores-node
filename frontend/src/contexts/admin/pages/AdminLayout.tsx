import {
  AppShell,
  Breadcrumbs,
  Burger,
  Container,
  Group,
  Text,
} from "@mantine/core"
import { Outlet } from "react-router"
import { useDisclosure } from "@mantine/hooks"
import { Anchor, NavLink } from "../../../components"

import classes from "../../../pages/Layout/Layout.module.css"
import { IconUsers } from "@tabler/icons-react"

const iconSize = 20

export default function AdminLayout() {
  const [opened, { toggle }] = useDisclosure()

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
        </Group>
      </AppShell.Header>
      <AppShell.Navbar p={0}>
        <AppShell.Section className={classes.menu_section}>
          <Text className={classes.section_title}>Node Admin</Text>

          <NavLink
            label="Node stewards"
            href="/admin/node_stewards"
            key="node_stewards"
            leftSection={<IconUsers size={iconSize} />}
            onClick={toggle}
          />
        </AppShell.Section>
      </AppShell.Navbar>
      <AppShell.Main>
        <Container>
          <Outlet />
        </Container>
      </AppShell.Main>
    </AppShell>
  )
}
