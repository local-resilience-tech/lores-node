import { Container, Stack, Title, Text } from "@mantine/core"
import AppRepoForm from "../components/AppRepoForm"
import { actionFailure, ActionPromiseResult, Anchor } from "../../../components"
import { AppRepoSource } from "../../../api/Api"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"

export default function NewLocalApp() {
  const navigate = useNavigate()
  const handleSubmit = async (
    values: AppRepoSource
  ): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.createAppRepo(values)
      .then(() => {
        console.log("App repository created successfully")
        navigate("..")
      })
      .catch(actionFailure)
  }

  return (
    <Container>
      <Stack mb="xl">
        <Title order={1}>Install a new app repository</Title>
        <Text>
          An app for LoRes Mesh is a{" "}
          <Anchor href="https://docs.docker.com/reference/cli/docker/stack/">
            docker stack
          </Anchor>
          . It's defined in a docker compose file that points to images from any
          image registry.
        </Text>
        <Text>
          An app repository is a git repository that contains one or more
          folders, each containing a different app with it's own docker compose
          file.
        </Text>
      </Stack>
      <AppRepoForm onSubmit={handleSubmit} />
    </Container>
  )
}
