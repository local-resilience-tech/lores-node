import { Container, Stack, Title } from "@mantine/core"

export default function NewLocalApp() {
  // This component will handle the creation of a new local app
  return (
    <Container>
      <Stack>
        <Title order={1}>Create New Local App</Title>
        {/* Form for creating a new local app will go here */}
      </Stack>
    </Container>
  )
}
