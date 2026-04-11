import { Alert, Container, Stack, Text, Title } from "@mantine/core"
import { IconDatabaseExclamation } from "@tabler/icons-react"
import { getApi } from "../../../api"
import { useEffect, useState } from "react"
import {
  ActionPromiseResult,
  actionSuccess,
  actionFailure,
  ActionButton,
} from "../../../components"
import { OperationCountEntry } from "../../../api/Api"
import TopicCountsTable from "../components/TopicCountsTable"
import { useAppSelector } from "../../../store"

export default function EventLog() {
  const [replayMessage, setReplayMessage] = useState<string | null>(null)
  const [operationCounts, setOperationCounts] = useState<
    OperationCountEntry[] | null
  >(null)
  const regions = useAppSelector((state) =>
    state.my_regions.all?.map((r) => r.region),
  )
  const myNodeId = useAppSelector((state) => state.network?.node.id)

  useEffect(() => {
    getApi()
      .nodeStewardApi.getOperationCounts()
      .then((response) => setOperationCounts(response.data))
      .catch(console.error)
  }, [])

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
        <Title order={2}>Operations Store</Title>

        {operationCounts !== null && (
          <Stack gap="xs">
            <Title order={3}>Operation counts by topic and author</Title>
            <TopicCountsTable
              counts={operationCounts}
              regions={regions}
              myNodeId={myNodeId}
            />
          </Stack>
        )}

        <Title order={3}>Replay Operations</Title>
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
