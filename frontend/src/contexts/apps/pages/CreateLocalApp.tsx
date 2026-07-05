import { Container, Stack, Title } from "@mantine/core"
import { useNavigate } from "react-router-dom"
import { getApi } from "../../../api"
import { LocalAppFormData } from "../../../api/Api"
import {
  actionFailure,
  ActionPromiseResult,
  actionSuccess,
} from "../../../components"
import LocalAppForm from "../components/LocalAppForm"

export default function CreateLocalApp() {
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
        <Title order={1}>Create local app</Title>
        <LocalAppForm
          onSubmit={onSubmit}
          submitLabel="Create app"
          cancelPath="/node/apps"
        />
      </Stack>
    </Container>
  )
}
