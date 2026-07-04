import { Table } from "@mantine/core"
import {
  LocalApp,
  LocalAppInstallation,
  RegionWithNodes,
} from "../../../api/Api"
import { Anchor } from "../../../components"
import { useAppSelector } from "../../../store"
import { regionDisplayName } from "../../regions"

interface AppsListProps {
  apps: LocalAppInstallation[]
}

export default function LocalAppsList({ apps }: AppsListProps) {
  const regions = useAppSelector((state) => state.my_regions.all)

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Region</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((installation) => {
          const region = regions?.find(
            (r) => r.region.id === installation.region_id,
          )
          return (
            <LocalAppRow
              key={installation.app.name}
              app={installation.app}
              region={region}
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
}

function LocalAppRow({ app, region }: LocalAppRowProps) {
  const regionName = region ? regionDisplayName(region.region) : ""
  return (
    <Table.Tr key={app.name}>
      <Table.Td>
        <Anchor href={`app/${app.name}`}>{app.name}</Anchor>
      </Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>{regionName}</Table.Td>
    </Table.Tr>
  )
}
