import { Stack, PasswordInput, Button, Select, TextInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import { ActionPromiseResult, DisplayActionResult, useOnSubmitWithResult } from "../../../components"
import { DomainManagementCredentials, DomainProvider, RegisteredDomainDetail } from "../../../api/Api"
import { useAppSelector } from "../../../store"

interface DomainProviderFormProps {
  registeredDomainDetail?: RegisteredDomainDetail
  onSubmit: (values: DomainManagementCredentials) => Promise<ActionPromiseResult | void>
}

export function DomainProviderForm({ onSubmit, registeredDomainDetail }: DomainProviderFormProps) {
  const [actionResult, onSubmitWithResult] = useOnSubmitWithResult<DomainManagementCredentials>(onSubmit)
  const providers = useAppSelector((state) => state.registeredDomains.availableDomainProviders) ?? []

  const form = useForm<DomainManagementCredentials>({
    mode: "controlled",
    initialValues: {
      provider: (registeredDomainDetail?.domain_provider as DomainProvider) ?? providers[0],
      username: registeredDomainDetail?.username ?? "",
      credential: registeredDomainDetail?.credential ?? "",
    },
    validate: {
      provider: (value) => {
        if (!value) return "Provider required"
        return null
      },
      username: (value) => {
        if (!value) return "Username is required"
        return null
      },
      credential: (value) => {
        if (!value) return "Credential is required"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <Select
          label="Domain provider"
          key="provider"
          description="Select the domain provider to authenticate with."
          data={providers}
          withAsterisk
          disabled={Boolean(registeredDomainDetail?.domain_provider)}
          {...form.getInputProps("provider")}
        />

        <TextInput
          label="Username"
          description="Username for the account"
          key="username"
          withAsterisk
          {...form.getInputProps("username")}
        />
        <PasswordInput
          label="Credential"
          description="Credential used to authenticate with the domain provider. This may be an API key or password."
          key="credential"
          withAsterisk
          {...form.getInputProps("credential")}
        />
        <DisplayActionResult result={actionResult} />
        <Button loading={form.submitting} type="submit">
          Store credential
        </Button>
      </Stack>
    </form>
  )
}
