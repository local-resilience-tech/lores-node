import { Box, Text } from "@chakra-ui/react"
import {
  BootstrapNode,
  BootstrapNodeData,
  BootstrapPeer,
} from "../../this_p2panda_node"
import { useState } from "react"
import ThisRegionApi from "../api"
import { getApi } from "../../../api"

const regionApi = new ThisRegionApi()

export default function ExistingRegion() {
  const [bootstrapData, setBootstrapData] = useState<BootstrapNodeData | null>(
    null,
  )

  const onSubmitBootstrapNode = async (data: BootstrapNodeData) => {
    const peer: BootstrapPeer = {
      node_id: data.node_id,
    }
    const result = await getApi().api.bootstrap({
      network_name: data.network_name,
      bootstrap_peer: peer,
    })
    if (result.status !== 200) {
      console.error("Failed to bootstrap the network", result)
    }

    // temp
    setBootstrapData(data)
  }

  if (bootstrapData == null) {
    return <BootstrapNode onSubmit={onSubmitBootstrapNode} />
  }

  return (
    <Box>
      <Text>TODO: We should have booted the network</Text>
    </Box>
  )
}
