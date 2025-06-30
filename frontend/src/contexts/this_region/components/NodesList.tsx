import { Link, VStack, Card, Text, DataList } from "@chakra-ui/react"
import { NodeDetailsWithStatus } from "../../this_node"

const IpLink = ({ ip }: { ip: string }) => {
  return (
    <Link href={`https://${ip}`} target="_blank" rel="noopener noreferrer">
      {ip}
    </Link>
  )
}

export default function NodesList({
  nodes: nodes,
}: {
  nodes: NodeDetailsWithStatus[]
}) {
  return (
    <VStack alignItems="stretch" gap={4}>
      {nodes.map((node) => (
        <Card.Root key={node.id}>
          <Card.Body gap="2">
            <Card.Title mt="2">{node.name}</Card.Title>
            <Text fontSize="xs" fontFamily="mono" mt={-2}>
              {node.id}
            </Text>
            <DataList.Root orientation="horizontal">
              <DataList.Item key="message">
                <DataList.ItemLabel>Message</DataList.ItemLabel>
                <DataList.ItemValue>{node.status_text}</DataList.ItemValue>
              </DataList.Item>
              <DataList.Item key="ip">
                <DataList.ItemLabel>IP</DataList.ItemLabel>
                <DataList.ItemValue>
                  {node.public_ipv4 || "unknown"}
                </DataList.ItemValue>
              </DataList.Item>
              <DataList.Item key="state">
                <DataList.ItemLabel>State</DataList.ItemLabel>
                <DataList.ItemValue>
                  {node.state || "unknown"}
                </DataList.ItemValue>
              </DataList.Item>
            </DataList.Root>
          </Card.Body>
        </Card.Root>
      ))}
    </VStack>
  )
}
