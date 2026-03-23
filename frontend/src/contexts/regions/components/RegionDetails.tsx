import { Table } from "@mantine/core"
import { Region } from "../../../api/Api"
import { Anchor } from "../../../components"

export default function RegionDetails({
  region,
}: {
  region: Region | null | undefined
}) {
  if (!region) return null

  return (
    <Table>
      <Table.Tbody>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Td>{region.name}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>ID</Table.Th>
          <Table.Td>{region.id}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Creator Node</Table.Th>
          <Table.Td>{region.creator_node_id}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Slug</Table.Th>
          <Table.Td>{region.slug}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Organisation Name</Table.Th>
          <Table.Td>{region.organisation_name}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Organisation URL</Table.Th>
          <Table.Td>
            {region.organisation_url && (
              <Anchor href={region.organisation_url}>
                {region.organisation_url}
              </Anchor>
            )}
          </Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>User Conduct URL</Table.Th>
          <Table.Td>
            {region.user_conduct_url && (
              <Anchor href={region.user_conduct_url}>
                {region.user_conduct_url}
              </Anchor>
            )}
          </Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>User Privacy URL</Table.Th>
          <Table.Td>
            {region.user_privacy_url && (
              <Anchor href={region.user_privacy_url}>
                {region.user_privacy_url}
              </Anchor>
            )}
          </Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Node Steward Conduct URL</Table.Th>
          <Table.Td>
            {region.node_steward_conduct_url && (
              <Anchor href={region.node_steward_conduct_url}>
                {region.node_steward_conduct_url}
              </Anchor>
            )}
          </Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  )
}
