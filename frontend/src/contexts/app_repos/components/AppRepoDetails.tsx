import { Table } from "@mantine/core"
import { AppRepo } from "../../../api/Api"

export default function AppRepoDetails({ appRepo }: { appRepo: AppRepo }) {
  return (
    <Table>
      <Table.Tbody>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Td>{appRepo.name}</Table.Td>
        </Table.Tr>
        <Table.Tr>
          <Table.Th>Git URL</Table.Th>
          <Table.Td>{appRepo.git_url}</Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  )
}
