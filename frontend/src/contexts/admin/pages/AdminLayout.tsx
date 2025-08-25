import {
  AppShell,
  Breadcrumbs,
  Burger,
  Container,
  Group,
  Text,
} from "@mantine/core"
import { Outlet, useNavigate } from "react-router"
import { getApi } from "../../../api"
import type { Node } from "../../../api/Api"
import { useEffect, useState } from "react"
import { useDisclosure } from "@mantine/hooks"
import { Anchor, NavLink } from "../../../components"

import classes from "../../../pages/Layout/Layout.module.css"
import { IconUsers } from "@tabler/icons-react"

const iconSize = 20

export default function AdminLayout() {
  const navigate = useNavigate()
  const [node, setNode] = useState<Node | null>(null)
  const [opened, { toggle }] = useDisclosure()

  const loadNode = () => {
    getApi()
      .adminApi.showThisNode()
      .then((response) => {
        setNode(response.data)
      })
      .catch((error) => {
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          console.error("Error fetching node stewards:", error)
        }
      })
  }

  useEffect(() => {
    loadNode()
  }, [])

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
          <Breadcrumbs>{node && <Text>{node.name}</Text>}</Breadcrumbs>
        </Group>
      </AppShell.Header>
      <AppShell.Navbar p={0}>
        <AppShell.Section className={classes.menu_section}>
          <Text className={classes.section_title}>
            {node?.name ? (
              <>
                <Text span c="dimmed">
                  ADMIN:{" "}
                </Text>
                <Text span>{node.name}</Text>
              </>
            ) : (
              "This Node"
            )}
          </Text>

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
