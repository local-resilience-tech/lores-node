import {
  NavLink as MantineNavLink,
  type NavLinkProps as MantineNavLinkProps,
} from "@mantine/core"
import { Link } from "react-router-dom"

export type NavLinkProps = MantineNavLinkProps & {
  href: string
}

export default function NavLink({ href, ...innerProps }: NavLinkProps) {
  return <MantineNavLink component={Link} {...innerProps} to={href} />
}
