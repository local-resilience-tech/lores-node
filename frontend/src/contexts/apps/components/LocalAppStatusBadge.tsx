import { Badge } from "@mantine/core"
import { LocalAppInstallStatus } from "../../../api/Api"
import { IconBrandDocker, IconDatabase } from "@tabler/icons-react"

export default function LocalAppStatusBadge({
  status,
}: {
  status: LocalAppInstallStatus
}) {
  const sharedProps = {
    size: "lg",
    variant: "outline",
  }

  switch (status) {
    case LocalAppInstallStatus.Installed:
      return (
        <Badge color="yellow" {...sharedProps} leftSection={<IconDatabase />}>
          Installed
        </Badge>
      )
    case LocalAppInstallStatus.StackDeployed:
      return (
        <Badge color="blue" {...sharedProps} leftSection={<IconBrandDocker />}>
          Stack Deployed
        </Badge>
      )
  }
}
