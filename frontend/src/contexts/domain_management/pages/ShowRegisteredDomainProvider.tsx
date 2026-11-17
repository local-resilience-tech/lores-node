import { ActionIcon, Group, Stack, Title } from "@mantine/core"
import { useNavigate, useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import { useEffect } from "react"
import { findDomainProviderDetail } from "../../../store/domain_management"
import { IconEdit } from "@tabler/icons-react"

export default function ShowRegisteredDomainProvider() {
  // to do:
  // - sort naming of these variables

  const { domainRegistrationId } = useParams()
  const navigate = useNavigate()

  const domainProviderDetail = useAppSelector((state) => findDomainProviderDetail(state.registeredDomains, domainRegistrationId))

  useEffect(() => {
    if (!domainProviderDetail) {
      navigate(-1)
    }
  }, [domainRegistrationId, domainProviderDetail, navigate])

  return (
    <Group justify="space-between">
      <Stack>
        <Title order={1}>Provider: {domainProviderDetail?.domain_provider}</Title>
        <Title order={2} c="dimmed" fz="xl">
          Account: {domainProviderDetail?.username}
        </Title>
      </Stack>
      <ActionIcon size="lg" onClick={() => navigate("./edit")}>
        <IconEdit />
      </ActionIcon>
    </Group>
  )
}
