import { Link, Table } from "@chakra-ui/react"
import { NodeDetails } from "../../this_node"

const IpLink = ({ ip }: { ip: string }) => {
  return (
    <Link href={`https://${ip}`} target="_blank" rel="noopener noreferrer">
      {ip}
    </Link>
  )
}

export default function NodesList({ nodes: nodes }: { nodes: NodeDetails[] }) {
  return (
    <Table.Root variant="line">
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeader>Name</Table.ColumnHeader>
          <Table.ColumnHeader>Node ID</Table.ColumnHeader>
          <Table.ColumnHeader>IP</Table.ColumnHeader>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {nodes.map((node) => (
          <Table.Row key={node.id}>
            <Table.Cell>{node.name}</Table.Cell>
            <Table.Cell>{node.id}</Table.Cell>
            <Table.Cell>
              {node.public_ipv4 && <IpLink ip={node.public_ipv4} />}
            </Table.Cell>
          </Table.Row>
        ))}
      </Table.Body>
    </Table.Root>
  )
}
