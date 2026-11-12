import { Stack, Title } from "@mantine/core"
import { useParams } from "react-router-dom"
import { getApi } from "../../../api"
import { useEffect, useState } from "react"
import { RegisteredDomainData } from "../../../api/Api"

export default function ShowRegisteredDomainProvider() {
  // to do:
  // - option to edit credentials
  // - display error when unable to auth with provider
  // - redirect to domain providers page on 404
  // - sort naming of these variables
  const [domainProviderDetails, setDomainProviderDetails] = useState<RegisteredDomainData>()

  const { domainRegistrationId } = useParams()
  console.log({ domainRegistrationId })

  const getRegisteredDomainDetails = async () => {
    getApi()
      .nodeStewardApi.getRegisteredDomain(domainRegistrationId)
      .then((response) => {
        setDomainProviderDetails(response.data)
      })
      .catch((error) => {
        console.log({ error })
        // navigate to login when 401
        // navigate to domain providers when any other error
      })
  }

  useEffect(() => {
    getRegisteredDomainDetails()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return (
    <Stack>
      <Title order={1}>Provider: {domainProviderDetails?.domain_provider}</Title>
      <Title order={2} c="dimmed" fz="xl">
        Account: {domainProviderDetails?.username}
      </Title>
    </Stack>
  )
}
