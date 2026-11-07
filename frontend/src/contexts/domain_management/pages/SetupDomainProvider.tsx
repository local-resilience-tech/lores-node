import { Stack, Title } from "@mantine/core"
import { DomainProviderForm } from "../components/DomainProviderForm"
import { DomainManagementCredentials } from "../../../api/Api"
import { actionFailure, ActionPromiseResult } from "../../../components"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"

export default function SetupDomainProvider() {
  const navigate = useNavigate()

  const handleSubmit = async (values: DomainManagementCredentials): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.registerProvider(values)
      .then((response) => {
        const { data: domainRegistrationId } = response
        navigate(`../${domainRegistrationId}`, { relative: "path" })
      })
      .catch((error) => {
        // to do - double check our endpoint can return this response
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          return actionFailure(error)
        }
      })
  }

  return (
    <Stack gap="lg" h="300">
      <Title order={1}>Setup new domain provider</Title>
      <DomainProviderForm onSubmit={handleSubmit} />
    </Stack>
  )
}
