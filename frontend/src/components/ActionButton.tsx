import {
  ActionIcon,
  ButtonProps,
  Button,
  Group,
  HoverCard,
  PolymorphicComponentProps,
  Text,
} from "@mantine/core"
import { IconAlertCircle, IconX } from "@tabler/icons-react"
import { useState } from "react"
import { ActionPromiseResult, ActionResult } from "./ActionResult"

type ActionButtonProps = PolymorphicComponentProps<"button", ButtonProps> & {
  children?: React.ReactNode
  onClick?: () => Promise<ActionPromiseResult>
  expand?: boolean
  successColor?: string
  errorColor?: string
  errorIcon?: React.ReactNode
}

export default function ActionButton({
  children,
  onClick,
  successColor = "green",
  errorColor = "red",
  expand = false,
  errorIcon = <IconAlertCircle />,
  ...props
}: ActionButtonProps) {
  const [loading, setLoading] = useState(false)
  const [showResult, setShowResult] = useState(false)
  const [result, setResult] = useState<ActionResult | null>(null)

  const handleResult = (result: ActionPromiseResult) => {
    if (result && typeof result === "object" && "success" in result) {
      if (result.success) {
        setSuccess()
      } else {
        setError(result.error || "Unknown error")
      }
    }
  }

  const setSuccess = () => {
    setResult({ success: true })
    setShowResult(true)
    setTimeout(() => {
      setShowResult(false)
      setResult(null)
    }, 2000)
  }

  const setError = (error: string) => {
    setResult({ success: false, error })
    setShowResult(true)
  }

  const clearError = () => {
    setShowResult(false)
    setResult(null)
  }

  const handleClick = async () => {
    if (onClick) {
      setLoading(true)
      onClick()
        .then(handleResult)
        .catch(setError)
        .finally(() => {
          setLoading(false)
        })
    }
  }

  let resultProps = {}
  if (showResult) {
    if (result?.success) {
      console.log("got success colour in result:", result)
      resultProps = { color: successColor }
    }
    if (result?.error) {
      resultProps = { color: errorColor }
    }
  }

  const button = (
    <Button
      {...props}
      onClick={handleClick}
      loading={loading}
      {...resultProps}
      style={{ flexGrow: expand ? 1 : 0 }}
    >
      {children}
    </Button>
  )

  if (result?.error && showResult && errorIcon) {
    return (
      <Group justify="stretch">
        {button}
        <HoverCard width={280} shadow="md">
          <HoverCard.Target>{errorIcon}</HoverCard.Target>
          <HoverCard.Dropdown>
            <Group justify="space-between" align="flex-start">
              <Text size="sm">
                {result.error ||
                  "An error occurred while performing the action."}
              </Text>
              <ActionIcon
                key="close-error"
                radius="xl"
                color="gray"
                onClick={(event) => {
                  event.stopPropagation()
                  clearError()
                }}
              >
                <IconX />
              </ActionIcon>
            </Group>
          </HoverCard.Dropdown>
        </HoverCard>
      </Group>
    )
  } else {
    return button
  }
}
