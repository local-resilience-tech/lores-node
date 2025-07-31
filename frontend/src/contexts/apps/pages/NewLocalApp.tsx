import { Container, Stack, Title, Text } from "@mantine/core"
import InstallAppRepositoryForm from "../components/InstallAppRepositoryForm"
import { Anchor } from "../../../components"
import { useAppSelector } from "../../../store"
import { AppDefinitionReference } from "../../../api/Api"

export default function NewLocalApp() {
  const appRepos = useAppSelector((state) => state.appRepos)

  const handleSubmit = async (values: AppDefinitionReference) => {
    // Handle the form submission logic here, e.g., API call to install the app
    console.log("Installing app:", values)
  }

  return (
    <Container>
      <Stack mb="xl">
        <Title order={1}>Install a new local app</Title>
        <Text>
          An app for LoRes Mesh is a{" "}
          <Anchor href="https://docs.docker.com/reference/cli/docker/stack/">
            docker stack
          </Anchor>
          . It's defined in a docker compose file that points to images from any
          image registry.
        </Text>
        <Text>
          The docker compose file should be placed in a git repository so that
          we can pull it and treat tags on the repository as version numbers.
          It's fine to have multipole apps in the same repository, as long as
          they are in different paths, which is why we ask for an optional path
          below.
        </Text>
      </Stack>
      <InstallAppRepositoryForm appRepos={appRepos} onSubmit={handleSubmit} />
    </Container>
  )
}
