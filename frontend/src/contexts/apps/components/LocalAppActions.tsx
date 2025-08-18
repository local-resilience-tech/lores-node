import { Button, Group, MantineColor } from "@mantine/core"
import { LocalApp } from "../../../api/Api"
import { useState } from "react"
import { ActionPromiseResult, ActionResult } from "../../../components"

export interface LocalAppAction {
  type: "deploy" | "remove" | "register"
  buttonColor?: MantineColor
  primary?: boolean
  handler: (app: LocalApp) => Promise<ActionPromiseResult>
}

function LocalAppAction({
  action,
  app,
}: {
  action: LocalAppAction
  app: LocalApp
}) {
  const [result, setResult] = useState<ActionPromiseResult>(undefined)
  const [loading, setLoading] = useState(false)

  const handleButtonPress = async (
    app: LocalApp,
    handler: (app: LocalApp) => Promise<ActionPromiseResult>
  ) => {
    try {
      setLoading(true)
      const result = await handler(app)
      setResult(result)
    } catch (error) {
      console.error("Error occurred while handling button press:", error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <Button
      onClick={() => handleButtonPress(app, action.handler)}
      size="sm"
      variant={action.primary ? "filled" : "outline"}
      color={action.buttonColor}
      loading={loading}
    >
      {action.type.charAt(0).toUpperCase() + action.type.slice(1)}
    </Button>
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
    <Group>
      {actions.map((action) => (
        <LocalAppAction key={action.type} action={action} app={app} />
      ))}
    </Group>
  )
}
