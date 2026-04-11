import { Alert, Container, Stack, Text, Title } from "@mantine/core"
import { IconDatabaseExclamation } from "@tabler/icons-react"
import { getApi } from "../../../api"
import { useState } from "react"
import {
  ActionPromiseResult,
  actionSuccess,
  actionFailure,
  ActionButton,
} from "../../../components"

export default function EventLog() {
  const [replayMessage, setReplayMessage] = useState<string | null>(null)

  const replayProjections = async (): Promise<ActionPromiseResult> =>
    getApi()
      .nodeStewardApi.replayProjections()
      .then((response) => {
        setReplayMessage(response.data)
        return actionSuccess()
      })
      .catch(actionFailure)

  return (
    <Container>
      <Stack>
        <Title order={2}>Replay Operations</Title>
        <Text c="dimmed" size="sm">
          Truncates all projection tables and re-processes every stored
          operation through the event handlers. Use this after fixing handler
          code during development.
        </Text>
        <ActionButton onClick={replayProjections} color="orange">
          Replay all operations
        </ActionButton>
        {replayMessage && (
          <Alert color="green" icon={<IconDatabaseExclamation />}>
            {replayMessage}
          </Alert>
        )}
      </Stack>
    </Container>
  )
}
