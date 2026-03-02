import { Badge, Group, Table, Text } from "@mantine/core"
import { RegionAppWithInstallations, RegionNodeDetails } from "../../../api/Api"
import { NodesMap } from "../../../store/my_regions"

interface AppsListProps {
  apps: RegionAppWithInstallations[]
  nodes: NodesMap
}

function NodeName({ node }: { node: RegionNodeDetails }) {
  return <Badge>{node.name}</Badge>
}

export default function RegionAppsList({ apps, nodes }: AppsListProps) {
  return <Text>This needs fixing</Text>
  // return (
  //   <Table>
  //     <Table.Thead>
  //       <Table.Tr>
  //         <Table.Th>Name</Table.Th>
  //         <Table.Th>Nodes</Table.Th>
  //       </Table.Tr>
  //     </Table.Thead>
  //     <Table.Tbody>
  //       {apps.map((app) => (
  //         <Table.Tr key={app.name}>
  //           <Table.Td>{app.name}</Table.Td>
  //           <Table.Td>
  //             <Group gap={4}>
  //               {app.installations.map((installation) => {
  //                 const node = nodes.get(installation.node_id)
  //                 return (
  //                   node && <NodeName key={installation.node_id} node={node} />
  //                 )
  //               })}
  //             </Group>
  //           </Table.Td>
  //         </Table.Tr>
  //       ))}
  //     </Table.Tbody>
  //   </Table>
  // )
}
