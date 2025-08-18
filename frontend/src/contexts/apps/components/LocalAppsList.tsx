import { Table } from "@mantine/core"
import { AppDefinition, AppRepo, LocalApp } from "../../../api/Api"
import { useLoading } from "../../shared"
import { Anchor } from "../../../components"
import LocalAppStatusBadge from "./LocalAppStatusBadge"

export interface LocalAppWithRepo {
  app: LocalApp
  repo: AppRepo | undefined
  repo_app_definition: AppDefinition | undefined
}

interface AppsListProps {
  apps: LocalAppWithRepo[]
}

export default function LocalAppsList({ apps }: AppsListProps) {
  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Version</Table.Th>
          <Table.Th>Latest</Table.Th>
          <Table.Th>Status</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {apps.map((appWithRepo) => (
          <LocalAppRow
            key={appWithRepo.app.name}
            app={appWithRepo.app}
            repo_app_definition={appWithRepo.repo_app_definition}
          />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface LocalAppRowProps {
  app: LocalApp
  repo_app_definition?: AppDefinition
}

function LocalAppRow({ app, repo_app_definition }: LocalAppRowProps) {
  const [loading, withLoading] = useLoading(false)

  const handleButtonPress = (
    app: LocalApp,
    action: (app: LocalApp) => Promise<void>
  ) => {
    withLoading(async () => {
      await action(app)
    })
  }

  return (
    <Table.Tr key={app.name}>
      <Table.Td>
        <Anchor href={`app/${app.name}`}>{app.name}</Anchor>
      </Table.Td>
      <Table.Td>{app.version}</Table.Td>
      <Table.Td>{repo_app_definition?.latest_version}</Table.Td>
      <Table.Td>
        <LocalAppStatusBadge status={app.status} />
      </Table.Td>
    </Table.Tr>
  )
}
