import { Anchor, AppShell, Burger, Container, Group } from "@mantine/core"
import { NavLink } from "../../components"
import { Outlet } from "react-router-dom"
import { useDisclosure } from "@mantine/hooks"
import { IconAffiliate, IconBrandGithub, IconHome } from "@tabler/icons-react"
import packageJson from "../../../package.json"
import pangaLogoUrl from "../../assets/deepsea-panda.svg"

import classes from "./Layout.module.css"
import { useAppSelector } from "../../store"

export default function Layout() {
  const [opened, { toggle }] = useDisclosure()
  const iconSize = 20

  const region = useAppSelector((state) => state.region)
  const node = useAppSelector((state) => state.thisNode)

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
        {region && (
          <AppShell.Section className={classes.menu_section}>
            <NavLink
              label="Nodes"
              href="/nodes"
              leftSection={<IconAffiliate size={iconSize} />}
              onClick={toggle}
            />
            <NavLink
              label={node ? node.name : "This Node"}
              href="/this_node"
              key={node ? node.id : "this_node"}
              leftSection={<IconHome size={iconSize} />}
              onClick={toggle}
            />
            <NavLink
              label="P2Panda"
              href="/p2panda_node"
              leftSection={
                <img src={pangaLogoUrl} alt="P2Panda Icon" width={iconSize} />
              }
              onClick={toggle}
            />
          </AppShell.Section>
        )}

        <AppShell.Section className={classes.footer_section}>
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
