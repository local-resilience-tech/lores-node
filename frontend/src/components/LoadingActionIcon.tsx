import {
  ActionIcon,
  ActionIconProps,
  PolymorphicComponentProps,
} from "@mantine/core"
import { useLoading } from "../contexts/shared"

type LoadingActionIconProps = PolymorphicComponentProps<
  "button",
  ActionIconProps
> & {
  children?: React.ReactNode
  onClick?: () => Promise<void>
}

export default function LoadingActionIcon({
  children,
  onClick,
  ...props
}: LoadingActionIconProps) {
  const [loading, withLoading] = useLoading(false)

  const handleClick = async () => {
    if (onClick) {
      withLoading(async () => {
        await onClick()
      })
    }
  }

  return (
    <ActionIcon {...props} onClick={handleClick} loading={loading}>
      {children}
    </ActionIcon>
  )
}
