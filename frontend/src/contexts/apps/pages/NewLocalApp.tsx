import { Breadcrumbs, Container, Stack, Title, Text } from "@mantine/core"
import { useNavigate } from "react-router-dom"
import { getApi } from "../../../api"
import { LocalAppFormData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
  Anchor,
} from "../../../components"
import LocalAppForm from "../components/LocalAppForm"

export default function NewLocalApp() {
  const navigate = useNavigate()

  const onSubmit = async (
    data: LocalAppFormData,
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.createLocalApp(data)
      .then((response) => {
        navigate("/node/apps/")
        return actionSuccess()
      })
      .catch(actionFailure)
  }

  return (
    <Container>
      <Stack gap="lg">
        <Stack gap="xs">
          <Breadcrumbs>
            <Anchor href="/node/apps">Local apps</Anchor>
            <Text c="dimmed">new</Text>
          </Breadcrumbs>
          <Title order={1}>New local app</Title>
        </Stack>

        <LocalAppForm
          onSubmit={onSubmit}
          submitLabel="Create app"
          cancelPath="/node/apps"
        />
      </Stack>
    </Container>
  )
}
