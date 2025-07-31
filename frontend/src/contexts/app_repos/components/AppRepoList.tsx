import { Table } from "@mantine/core"
import { AppRepo } from "../../../api/Api"

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
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {repos.map((repo) => (
          <Table.Tr key={repo.name}>
            <Table.Td>{repo.name}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  )
}
