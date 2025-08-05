import { Table } from "@mantine/core"
import { DockerService, DockerStackWithServices } from "../../../api/Api"
import { ReactNode } from "react"

interface StacksListProps {
  stacks: DockerStackWithServices[]
}

function buildServiceCells(service: DockerService): ReactNode[] {
  return [
    <Table.Td>{service.name}</Table.Td>,
    <Table.Td>{service.image}</Table.Td>,
    <Table.Td>{service.current_state}</Table.Td>,
  ]
}

export default function StacksList({ stacks }: StacksListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Stack</Table.Th>
          <Table.Th colSpan={3}>Services</Table.Th>
        </Table.Tr>
        <Table.Tr c="dimmed">
          <Table.Th></Table.Th>
          <Table.Th>Name</Table.Th>
          <Table.Th>Image</Table.Th>
          <Table.Th>Current State</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {stacks.map((stack) => (
          <>
            <Table.Tr key={stack.name}>
              <Table.Td rowSpan={stack.services.length}>{stack.name}</Table.Td>
              {stack.services.length > 0 ? (
                buildServiceCells(stack.services[0])
              ) : (
                <Table.Td colSpan={3}>No services</Table.Td>
              )}
            </Table.Tr>
            {stack.services.slice(1).map((service) => (
              <Table.Tr key={service.name}>
                {buildServiceCells(service)}
              </Table.Tr>
            ))}
          </>
        ))}
      </Table.Tbody>
    </Table>
  )
}
