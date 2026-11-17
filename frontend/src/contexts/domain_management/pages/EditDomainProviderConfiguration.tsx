import { Stack, Title } from "@mantine/core"
import { DomainProviderForm } from "../components/DomainProviderForm"
import { useNavigate, useParams } from "react-router-dom"
import { useAppSelector } from "../../../store"
import { findDomainProviderDetail } from "../../../store/domain_management"
import { useEffect } from "react"
import { DomainManagementCredentials } from "../../../api/Api"
import { actionFailure, ActionPromiseResult } from "../../../components"
import { getApi } from "../../../api"

export default function EditDomainProviderConfiguration() {
  const { domainRegistrationId } = useParams()
  const navigate = useNavigate()

  const domainProviderDetail = useAppSelector((state) => findDomainProviderDetail(state.registeredDomains, domainRegistrationId))

  useEffect(() => {
    if (!domainProviderDetail) {
      navigate(-1)
    }
  }, [domainRegistrationId, domainProviderDetail, navigate])

  const handleSubmit = async (values: DomainManagementCredentials): Promise<ActionPromiseResult> => {
    return getApi()
      .nodeStewardApi.updateDomainRegistration(domainRegistrationId!, values)
      .then((response) => {
        const { data: domainRegistrationId } = response
        navigate(`../../${domainRegistrationId}`, { relative: "path" })
      })
      .catch((error) => {
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          return actionFailure(error)
        }
      })
  }

  return (
    <Stack gap="lg" h="300">
      <Title order={1}>Edit domain provider</Title>
      <DomainProviderForm onSubmit={handleSubmit} registeredDomainDetail={domainProviderDetail} />
    </Stack>
  )
}
