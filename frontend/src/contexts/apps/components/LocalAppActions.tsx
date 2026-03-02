import {
  Button,
  Group,
  HoverCard,
  MantineColor,
  Stack,
  Tooltip,
} from "@mantine/core"
import { LocalApp } from "../../../api/Api"
import { useState } from "react"
import { ActionPromiseResult, ActionResult } from "../../../components"
import { ActionResultErrorIcon } from "../../../components/ActionResult"
import { awaitConfirmModal } from "../../shared"

export type LocalAppActionHandler = (
  app: LocalApp,
) => Promise<ActionPromiseResult>

export type LocalAppActionType = "register"

export interface LocalAppAction {
  type: LocalAppActionType
  buttonColor?: MantineColor
  primary?: boolean
  handler: LocalAppActionHandler
  disabled?: boolean
  tooltip?: string
  buildName?: (app: LocalApp, type: LocalAppActionType) => string
}

function LocalAppAction({
  action,
  app,
}: {
  action: LocalAppAction
  app: LocalApp
}) {
  const [result, setResult] = useState<ActionResult | undefined>(undefined)
  const [loading, setLoading] = useState(false)

  const handleButtonPress = async (
    app: LocalApp,
    handler: LocalAppActionHandler,
  ) => {
    try {
      setLoading(true)
      const result = await handler(app)
      console.log("THE result:", result)
      setResult(result || undefined)
    } catch (error) {
      console.error("Error occurred while handling button press:", error)
    } finally {
      setLoading(false)
    }
  }

  const button = (
    <Button
      onClick={() => handleButtonPress(app, action.handler)}
      size="sm"
      variant={action.primary ? "filled" : "outline"}
      color={action.buttonColor}
      loading={loading}
      disabled={action.disabled}
    >
      {action.buildName
        ? action.buildName(app, action.type)
        : nameFromActionType(action.type)}
    </Button>
  )

  return (
    <Group gap="xs">
      {action.tooltip ? (
        <Tooltip label={action.tooltip}>{button}</Tooltip>
      ) : (
        button
      )}
      <ActionResultErrorIcon result={result} />
    </Group>
  )
}

export default function LocalAppActions({
  actions,
  app,
}: {
  actions: LocalAppAction[]
  app: LocalApp
}) {
  return (
    <Stack>
      {actions.map((action) => (
        <LocalAppAction key={action.type} action={action} app={app} />
      ))}
    </Stack>
  )
}

export function confirmLocalAppAction(
  actionHandler: LocalAppActionHandler,
  title?: string,
  children?: React.ReactNode,
): LocalAppActionHandler {
  return async (app: LocalApp) => {
    const confirmed = await awaitConfirmModal(title, children)
    if (confirmed) {
      return actionHandler(app)
    } else {
      const result: ActionPromiseResult = {
        success: false,
        error: "Action cancelled by user",
      }
      return result
    }
  }
}

function nameFromActionType(type: string) {
  return type.charAt(0).toUpperCase() + type.slice(1)
}
