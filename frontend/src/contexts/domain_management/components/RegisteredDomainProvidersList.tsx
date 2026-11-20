import { Table } from "@mantine/core"
import { RegisteredDomainProviderDetail } from "../../../api/Api"
import { IconArrowRight, IconX } from "@tabler/icons-react"
import { actionSuccess, Anchor } from "../../../components"
import { getApi } from "../../../api"
import { useNavigate } from "react-router-dom"
import { actionFailure } from "../../../components"

interface RegisteredDomainProvidersListProps {
  details: RegisteredDomainProviderDetail[]
}

export default function RegisteredDomainProvidersList({ details }: RegisteredDomainProvidersListProps) {
  const navigate = useNavigate()
  const handleDelete = async (registrationId: string) => {
    getApi()
      .nodeStewardApi.deleteRegisteredDomainProvider({ registration_id: registrationId })
      .then(() => actionSuccess())
      .catch((error) => {
        if (error.response?.status === 401 || error.response?.status === 403) {
          navigate("/auth/admin/login")
        } else {
          return actionFailure(error)
        }
      })
  }

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Domain Provider</Table.Th>
          <Table.Th>Username</Table.Th>
          <Table.Th>Delete</Table.Th>
          <Table.Th>View</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {details.map((detail) => (
          <RegisteredDomainProviderRow detail={detail} handleDelete={handleDelete} key={detail.registration_id} />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface RegisteredDomainProviderRowProps {
  detail: RegisteredDomainProviderDetail
  handleDelete: (registrationId: string) => Promise<void>
}

function RegisteredDomainProviderRow({ detail, handleDelete }: RegisteredDomainProviderRowProps) {
  const { registration_id, domain_provider, username } = detail
  return (
    <Table.Tr key={registration_id}>
      <Table.Td>{domain_provider}</Table.Td>
      <Table.Td>{username}</Table.Td>
      <Table.Td>
        <IconX style={{ cursor: "pointer", color: "red" }} onClick={() => handleDelete(registration_id)} />
      </Table.Td>
      <Table.Td>
        <Anchor href={`./${registration_id}`}>
          <IconArrowRight />
        </Anchor>
      </Table.Td>
    </Table.Tr>
  )
}
