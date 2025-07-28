import { Container, Stack, Title, Text } from "@mantine/core"
import AppRepoForm from "../components/AppRepoForm"
import { Anchor } from "../../../components"

export default function NewLocalApp() {
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
      <AppRepoForm />
    </Container>
  )
}
