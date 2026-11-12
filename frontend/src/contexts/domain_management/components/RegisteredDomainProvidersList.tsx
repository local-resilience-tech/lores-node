import { Table } from "@mantine/core"
import { RegisteredDomainDetail } from "../../../api/Api"
import { IconArrowRight, IconX } from "@tabler/icons-react"
import { actionSuccess, Anchor } from "../../../components"
import { getApi } from "../../../api"

interface RegisteredDomainProvidersListProps {
  details: RegisteredDomainDetail[]
}

// there's also a console error to do with keys

export default function RegisteredDomainProvidersList({ details }: RegisteredDomainProvidersListProps) {
  const handleDelete = async (registrationId: string) => {
    getApi()
      .nodeStewardApi.deleteDomainRegistration({ registration_id: registrationId })
      .then(() => actionSuccess()) // not sure if we need this
      .catch((error) => {
        console.log("theres an error: ", error)
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
          <RegisteredDomainProviderRow detail={detail} handleDelete={handleDelete} />
        ))}
      </Table.Tbody>
    </Table>
  )
}

interface RegisteredDomainProviderRowProps {
  detail: RegisteredDomainDetail
  handleDelete: (registrationId: string) => Promise<void>
}

function RegisteredDomainProviderRow({ detail, handleDelete }: RegisteredDomainProviderRowProps) {
  return (
    <Table.Tr key={detail.registration_id}>
      <Table.Td>{detail.domain_provider}</Table.Td>
      <Table.Td>{detail.username}</Table.Td>
      <Table.Td>
        <IconX style={{ cursor: "pointer", color: "red" }} onClick={() => handleDelete(detail.registration_id)} />
      </Table.Td>
      <Table.Td>
        <Anchor href={`./${detail.registration_id}`}>
          <IconArrowRight />
        </Anchor>
      </Table.Td>
    </Table.Tr>
  )
}
