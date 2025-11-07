import { useForm } from "@mantine/form"
import type { Node, UpdateNodeDetails } from "../../../api/Api"
import { Button, Stack, TextInput } from "@mantine/core"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import isValidHostname from "is-valid-hostname"
import { isIPv4 } from "@chainsafe/is-ip"

interface EditNodeFormProps {
  node: Node
  onSubmit: (data: UpdateNodeDetails) => Promise<ActionPromiseResult>
}

const defaultInitialValues: UpdateNodeDetails = {
  name: "",
  public_ipv4: "",
  domain_on_local_network: undefined,
  domain_on_internet: undefined,
}

export default function EditNodeForm({ node, onSubmit }: EditNodeFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<UpdateNodeDetails>(onSubmit)

  const form = useForm<UpdateNodeDetails>({
    mode: "controlled",
    initialValues: {
      ...defaultInitialValues,
      ...{
        name: node.name,
        public_ipv4: node.public_ipv4 || "",
        domain_on_local_network: node.domain_on_local_network || "",
        domain_on_internet: node.domain_on_internet || "",
      },
    },
    validate: {
      name: (value) => {
        if (!value) return "This is required"
        if (value.length > 50) return "Must be less than 50 characters"
        if (!/^[a-z]+(-[a-z]+)*$/.test(value))
          return "Lowercase letters only, no spaces, hyphens allowed"
        return null
      },
      public_ipv4: (value) => {
        if (!value) return null // Optional field
        if (!isIPv4(value)) return "Invalid IPv4 address"
        return null
      },
      domain_on_local_network: (value) => {
        if (!value) return null // Optional field
        if (!isValidHostname(value)) return "Invalid hostname for local network"
        return null
      },
      domain_on_internet: (value) => {
        if (!value) return null // Optional field
        if (!isValidHostname(value)) return "Invalid hostname for internet"
        return null
      },
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack>
        <Stack>
          <TextInput
            label="Node name"
            placeholder="Enter node name"
            description="A name to identify your Node - use lowercase letters and no spaces"
            key="name"
            {...form.getInputProps("name")}
          />

          <TextInput
            label="Public IPv4"
            placeholder="Enter public IPv4"
            description="The public IPv4 address of your node"
            key="public_ipv4"
            {...form.getInputProps("public_ipv4")}
          />

          <TextInput
            label="Domain on Local Network"
            placeholder={`${form.values.name || "lores"}.local`}
            description="Hostname that is valid for clients on the local network"
            key="domain_on_local_network"
            {...form.getInputProps("domain_on_local_network")}
          />

          <TextInput
            label="Domain on Internet"
            placeholder={`${form.values.name || "lores"}.regionname.net`}
            description="Hostname that is valid for clients accessing over the internet"
            key="domain_on_internet"
            {...form.getInputProps("domain_on_internet")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Button loading={form.submitting} type="submit">
          Update
        </Button>
      </Stack>
    </form>
  )
}
