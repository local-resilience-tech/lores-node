import {
  ActionIcon,
  ActionIconProps,
  Button,
  Group,
  HoverCard,
  PolymorphicComponentProps,
  Text,
} from "@mantine/core"
import { IconAlertCircle, IconX } from "@tabler/icons-react"
import { useState } from "react"

interface LoadingActionItemResult {
  success: boolean
  error?: string
}

export type LoadingActionItemReturn = void | LoadingActionItemResult

type LoadingActionIconProps = PolymorphicComponentProps<
  "button",
  ActionIconProps
> & {
  children?: React.ReactNode
  onClick?: () => Promise<LoadingActionItemReturn>
  successColor?: string
  errorColor?: string
  errorIcon?: React.ReactNode
}

export default function LoadingActionIcon({
  children,
  onClick,
  successColor = "green",
  errorColor = "red",
  errorIcon = <IconAlertCircle />,
  ...props
}: LoadingActionIconProps) {
  const [loading, setLoading] = useState(false)
  const [showResult, setShowResult] = useState(false)
  const [result, setResult] = useState<LoadingActionItemResult | null>(null)

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
        .then(setSuccess)
        .catch(setError)
        .finally(() => {
          setLoading(false)
        })
    }
  }

  let resultProps = {}
  if (showResult) {
    if (result?.success) {
      resultProps = { color: successColor }
    }
    if (result?.error) {
      resultProps = { color: errorColor }
    }
  }

  return (
    <ActionIcon
      {...props}
      onClick={handleClick}
      loading={loading}
      {...resultProps}
    >
      {result?.error && showResult && errorIcon ? (
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
      ) : (
        children
      )}
    </ActionIcon>
  )
}
