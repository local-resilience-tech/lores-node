import { Badge } from "@mantine/core"
import { AppDefinition } from "../../../api/Api"

export default function AppBadge({ app }: { app: AppDefinition }) {
  return (
    <Badge variant="light" color="blue" style={{ textTransform: "none" }}>
      {app.name} {app.latest_version ? `v${app.latest_version}` : ""}
    </Badge>
  )
}
