import { Table, Text } from "@mantine/core"
import { IconBrandDocker, IconDatabase } from "@tabler/icons-react"
import { LocalApp, LocalAppSource, RegionWithNodes } from "../../../api/Api"
import { Anchor } from "../../../components"
import { useAppSelector } from "../../../store"
import { regionDisplayName } from "../../regions"

interface AppsListProps {
  apps: LocalApp[]
}

export default function LocalAppsList({ apps }: AppsListProps) {
  const regions = useAppSelector((state) => state.my_regions.all)
  const activeRegionId = useAppSelector(
    (state) => state.my_regions.activeRegionId,
  )

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th w={1} style={{ whiteSpace: "nowrap" }} />
          <Table.Th>Name</Table.Th>
          <Table.Th>Instance ID</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Region</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((app) => {
          const region = regions?.find(
            (r) => r.region.id === app.bound_to_region_id,
          )
          return (
            <LocalAppRow
              key={`${app.name}-${app.instance_id}`}
              app={app}
              region={region}
              isActiveRegion={region?.region.id === activeRegionId}
            />
          )
        })}
      </Table.Tbody>
    </Table>
  )
}

interface LocalAppRowProps {
  app: LocalApp
  region?: RegionWithNodes
  isActiveRegion?: boolean
}

function LocalAppRow({ app, region, isActiveRegion }: LocalAppRowProps) {
  const regionName = region ? regionDisplayName(region.region) : ""
  const source = app.source ?? LocalAppSource.Docker

  const sourceIcon =
    source === LocalAppSource.Db ? (
      <IconDatabase size={18} stroke={1.8} aria-hidden="true" />
    ) : (
      <IconBrandDocker size={18} stroke={1.8} aria-hidden="true" />
    )

  const sourceAltText =
    source === LocalAppSource.Db ? "Database app" : "Docker app"

  return (
    <Table.Tr key={app.name}>
      <Table.Td
        w={1}
        style={{ whiteSpace: "nowrap", verticalAlign: "middle" }}
        px="xs"
      >
        <span
          role="img"
          aria-label={sourceAltText}
          style={{ display: "inline-flex", verticalAlign: "middle" }}
        >
          {sourceIcon}
        </span>
      </Table.Td>
      <Table.Td>
        <Anchor
          href={`app/${encodeURIComponent(app.name || "-")}/${encodeURIComponent(app.instance_id || "-")}`}
        >
          {app.name}
        </Anchor>
      </Table.Td>
      <Table.Td>
        {app.instance_id ? (
          <Text size="xs" ff="monospace" truncate="end">
            {app.instance_id}
          </Text>
        ) : (
          <Text c="dimmed" size="xs" ff="monospace">
            [singleton]
          </Text>
        )}
      </Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>
        <Text fw={isActiveRegion ? 700 : undefined}>{regionName}</Text>
      </Table.Td>
    </Table.Tr>
  )
}
