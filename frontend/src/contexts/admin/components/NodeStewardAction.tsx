import { Button, Group, MantineColor, Stack } from "@mantine/core"
import { NodeSteward } from "../../../api/Api"
import { useState } from "react"
import {
  ActionButton,
  ActionPromiseResult,
  ActionResult,
} from "../../../components"
import { ActionResultErrorIcon } from "../../../components/ActionResult"

export interface NodeStewardAction {
  type: "extend"
  buttonColor?: MantineColor
  primary?: boolean
  handler: (record: NodeSteward) => Promise<ActionPromiseResult>
}

const names: Record<NodeStewardAction["type"], string> = {
  extend: "Extend token",
}

function NodeStewardActionButton({
  action,
  record,
}: {
  action: NodeStewardAction
  record: NodeSteward
}) {
  const [result, setResult] = useState<ActionResult | undefined>(undefined)
  const [loading, setLoading] = useState(false)

  const handleButtonPress = async (
    record: NodeSteward,
    handler: (record: NodeSteward) => Promise<ActionPromiseResult>
  ) => {
    try {
      setLoading(true)
      const result = await handler(record)
      console.log("THE result:", result)
      setResult(result || undefined)
    } catch (error) {
      console.error("Error occurred while handling button press:", error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <ActionButton
      onClick={() => handleButtonPress(record, action.handler)}
      size="compact-sm"
      color={action.buttonColor}
      variant={action.primary ? "filled" : "outline"}
    >
      {names[action.type] || action.type}
    </ActionButton>
  )

  // return (
  //   <Group gap="xs">
  //     <Button
  //       onClick={() => handleButtonPress(record, action.handler)}
  //       size="compact-sm"
  //       variant={action.primary ? "filled" : "outline"}
  //       color={action.buttonColor}
  //       loading={loading}
  //     >
  //       {names[action.type] || action.type}
  //     </Button>
  //     <ActionResultErrorIcon result={result} />
  //   </Group>
  // )
}

export default function NodeStewardActions({
  actions,
  record,
}: {
  actions: NodeStewardAction[]
  record: NodeSteward
}) {
  return (
    <Stack align="flex-start">
      {actions.map((action) => (
        <NodeStewardActionButton
          key={action.type}
          action={action}
          record={record}
        />
      ))}
    </Stack>
  )
}
