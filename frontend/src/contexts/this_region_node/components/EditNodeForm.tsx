import { useForm } from "@mantine/form"
import type { RegionNodeDetails, UpdateNodeDetails } from "../../../api/Api"
import { Button, Stack, TextInput } from "@mantine/core"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import LatLngInput, {
  EditableLatLng,
  emptyEditableLatLng,
  toLatLng,
  validateOptionalLatLng,
} from "../../../components/LatLngInput"
import isValidHostname from "is-valid-hostname"
import { isIPv4 } from "@chainsafe/is-ip"

interface EditNodeFormProps {
  node: RegionNodeDetails
  onSubmit: (data: UpdateNodeDetails) => Promise<ActionPromiseResult>
}

interface UpdateNodeFormData extends Omit<UpdateNodeDetails, "latlng"> {
  latlng: EditableLatLng
}

const defaultInitialValues: UpdateNodeFormData = {
  name: "",
  public_ipv4: "",
  domain_on_local_network: undefined,
  domain_on_internet: undefined,
  latlng: emptyEditableLatLng(),
}

export default function EditNodeForm({ node, onSubmit }: EditNodeFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<UpdateNodeDetails>(onSubmit)
  const nodeLatLng = (
    node as RegionNodeDetails & {
      latlng?: { lat?: number | null; lng?: number | null } | null
    }
  ).latlng

  const onSubmitWithResultWrapped = (data: UpdateNodeFormData) => {
    // Convert empty strings to undefined for optional fields
    const dataToSubmit: UpdateNodeDetails = {
      ...data,
      public_ipv4: data.public_ipv4 || undefined,
      domain_on_local_network: data.domain_on_local_network || undefined,
      domain_on_internet: data.domain_on_internet || undefined,
      latlng: toLatLng(data.latlng),
    }
    return onSubmitWithResult(dataToSubmit)
  }

  const form = useForm<UpdateNodeFormData>({
    mode: "controlled",
    initialValues: {
      ...defaultInitialValues,
      ...{
        name: node.name || "",
        public_ipv4: node.public_ipv4 || "",
        domain_on_local_network: node.domain_on_local_network || "",
        domain_on_internet: node.domain_on_internet || "",
        latlng: nodeLatLng
          ? {
              lat: nodeLatLng.lat ?? null,
              lng: nodeLatLng.lng ?? null,
            }
          : emptyEditableLatLng(),
      },
    },
    validate: {
      name: (value) => {
        if (!value) return "This is required"
        if (value.length > 50) return "Must be less than 50 characters"
        if (!/^[a-z0-9]+(-[a-z0-9]+)*$/.test(value))
          return "Lowercase letters, numbers and hyphens only, no spaces"
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
      latlng: validateOptionalLatLng,
    },
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResultWrapped)}>
      <Stack>
        <Stack>
          <TextInput
            label="Node name"
            placeholder="Enter node name"
            description="A name to identify your Node - use lowercase letters, numbers and hyphens only, no spaces"
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

          <LatLngInput
            label="Latitude and Longitude"
            description="The location of your node"
            key="latlng"
            {...form.getInputProps("latlng")}
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
