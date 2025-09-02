import { ActionIcon, HoverCard, Stack, Text } from "@mantine/core"
import { IconAlertCircle } from "@tabler/icons-react"
import { useState } from "react"
import { useNavigate } from "react-router-dom"

export interface ActionResult {
  success: boolean
  error?: string
  login_needed?: boolean
}

export type ActionPromiseResult = void | ActionResult

const statusResponses: Record<number, string> = {
  400: "BadRequest",
  401: "Unauthorized",
  403: "Forbidden",
  404: "NotFound",
}

export function actionFailure(error: any): ActionPromiseResult {
  console.error("Action failed:", error)

  const errorString =
    error.response?.data ||
    statusResponses[error.response?.status] ||
    "ServerError"

  const errorResult = {
    success: false,
    error: errorString,
    login_needed:
      error.response?.status === 401 || error.response?.status === 403,
  }
  console.log("Action result:", errorResult)
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

export function DisplayFormError({
  heading,
  description,
}: {
  heading: string
  description?: string | React.ReactNode
}) {
  return (
    <Stack gap="md">
      <Text c="red" fw="bold" fs="lg">
        {heading}
      </Text>
      {description && typeof description === "string" ? (
        <Text c="red">{description}</Text>
      ) : (
        description
      )}
    </Stack>
  )
}

interface DisplayActionResultProps {
  result: ActionResult | null
  displaySuccess?: boolean
  handlers?: ActionResultHandlers
  redirectToLogin?: boolean
}

export function DisplayActionResult({
  result,
  handlers = {},
  displaySuccess = false,
  redirectToLogin = true,
}: DisplayActionResultProps) {
  console.log("Displaying action result:", result)
  const navigate = useNavigate()

  if (!result) return null

  if (result.success) {
    if (displaySuccess) return <Text c="green">Action succeeded!</Text>
  } else if (result.error) {
    if (result.login_needed && redirectToLogin) {
      navigate("/auth/node_steward/login")
      return null
    }

    if (handlers && handlers[result.error]) {
      return handlers[result.error]
    } else {
      return <DisplayFormError heading={`Action failed: ${result.error}`} />
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
