import { ActionIcon, HoverCard, Text } from "@mantine/core"
import { IconAlertCircle } from "@tabler/icons-react"
import { useState } from "react"

export interface ActionResult {
  success: boolean
  error?: string
}

export type ActionPromiseResult = void | ActionResult

export function actionFailure(error: any): ActionPromiseResult {
  console.error("Action failed:", error)
  const errorResult = {
    success: false,
    error: error.response?.data || "ServerError",
  }
  return errorResult
}

export function actionSuccess(): ActionPromiseResult {
  console.log("Action succeeded")
  return { success: true }
}

export function useOnSubmitWithResult<ValType>(
  onSubmit: (values: ValType) => Promise<ActionPromiseResult>
): [ActionResult | null, (values: ValType) => Promise<void>] {
  const [actionResult, setActionResult] = useState<ActionResult | null>(null)

  const wrappedSubmit = (values: ValType): Promise<void> => {
    return onSubmit(values).then((result) => {
      setActionResult(result ?? null)
    })
  }

  return [actionResult, wrappedSubmit]
}

export type ActionResultHandlers = Record<string, React.ReactNode>

interface DisplayActionResultProps {
  result: ActionResult | null
  displaySuccess?: boolean
  handlers?: ActionResultHandlers
}

export function DisplayActionResult({
  result,
  handlers = {},
  displaySuccess = false,
}: DisplayActionResultProps) {
  if (!result) return null

  if (result.success) {
    if (displaySuccess) return <Text c="green">Action succeeded!</Text>
  } else if (result.error) {
    if (handlers && handlers[result.error]) {
      return handlers[result.error]
    } else {
      return <Text c="red">Action failed: {result.error}</Text>
    }
  }
}

interface ActionResultErrorIconProps {
  result: ActionResult | undefined
}

export function ActionResultErrorIcon({ result }: ActionResultErrorIconProps) {
  if (!result) return null
  if (result.success || !result.error) return null

  return (
    <HoverCard width={280} shadow="md">
      <HoverCard.Target>
        <ActionIcon variant="transparent" color="red" size="lg">
          <IconAlertCircle />
        </ActionIcon>
      </HoverCard.Target>
      <HoverCard.Dropdown>
        <Text size="sm">{result.error}</Text>
      </HoverCard.Dropdown>
    </HoverCard>
  )
}
