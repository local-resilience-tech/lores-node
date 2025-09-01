import { Button, MantineColor, Popover, Stack } from "@mantine/core"
import { NodeSteward } from "../../../api/Api"
import React, { useState } from "react"
import {
  ActionButton,
  ActionPromiseResult,
  ActionResult,
} from "../../../components"

export interface NodeStewardAction {
  type: "reset_token" | "display_token" | "disable" | "enable"
  buttonColor?: MantineColor
  primary?: boolean
  handler?: (record: NodeSteward) => Promise<ActionPromiseResult>
  overlay?: React.ReactNode
}

const names: Record<NodeStewardAction["type"], string> = {
  reset_token: "Reset token",
  display_token: "Display token",
  disable: "Disable",
  enable: "Enable",
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

  const buttonProps = {
    size: "compact-sm",
    color: action.buttonColor,
    variant: action.primary ? "filled" : "outline",
  }
  const buttonText = names[action.type] || action.type

  if (action.handler) {
    return (
      <ActionButton
        onClick={() => handleButtonPress(record, action.handler!)}
        {...buttonProps}
      >
        {buttonText}
      </ActionButton>
    )
  }

  if (action.overlay) {
    return (
      <Popover position="bottom" withArrow shadow="md">
        <Popover.Target>
          <Button {...buttonProps}>{buttonText}</Button>
        </Popover.Target>
        <Popover.Dropdown>{action.overlay}</Popover.Dropdown>
      </Popover>
    )
  }

  return null
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
