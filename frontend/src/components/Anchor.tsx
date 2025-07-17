import { Link } from "react-router-dom"
import {
  Anchor as MantineAnchor,
  type AnchorProps as MantineAnchorProps,
} from "@mantine/core"

export type AnchorProps = MantineAnchorProps & {
  href: string
  children: React.ReactNode
  newWindow?: boolean
}

export default function Anchor({
  href,
  children,
  newWindow,
  ...otherProps
}: AnchorProps) {
  return (
    <MantineAnchor
      component={Link}
      to={href}
      {...otherProps}
      target={newWindow ? "_blank" : undefined}
      rel={newWindow ? "noopener noreferrer" : undefined}
    >
      {children}
    </MantineAnchor>
  )
}
