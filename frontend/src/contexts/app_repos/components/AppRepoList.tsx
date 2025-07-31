import { Group, Table } from "@mantine/core"
import { AppRepo } from "../../../api/Api"
import AppBadge from "./AppBadge"

export default function AppRepoList({
  repos,
}: {
  repos: AppRepo[] | null | undefined
}) {
  if (repos === null || repos === undefined) return null

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Apps</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {repos.map((repo) => (
          <Table.Tr key={repo.name}>
            <Table.Td>{repo.name}</Table.Td>
            <Table.Td>
              <Group>
                {repo.apps.map((app) => (
                  <AppBadge key={app.name} app={app} />
                ))}
              </Group>
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
