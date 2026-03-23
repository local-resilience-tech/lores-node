import { TextInput, Button, Stack, Text } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import { UpdateMapData } from "../../../api/Api"

interface EditRegionMapFormProps {
  regionId: string
  onSubmit: (data: UpdateMapData) => Promise<ActionPromiseResult>
}

export default function EditRegionMapForm({
  onSubmit,
  regionId,
}: EditRegionMapFormProps) {
  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<UpdateMapData>(onSubmit)

  const form = useForm<UpdateMapData>({
    mode: "controlled",
    initialValues: {
      region_id: regionId,
      image_data_url: "",
      min_latlng: { lat: 0, lng: 0 },
      max_latlng: { lat: 0, lng: 0 },
    },
    validate: {},
  })

  return (
    <form onSubmit={form.onSubmit(onSubmitWithResult)}>
      <Stack gap="lg">
        <Text>
          When you edit the map for a region, you can update the image and the
          geographical boundaries.
        </Text>

        <Stack>
          <TextInput
            label="Image Data URL"
            description="The URL for the image representing your region"
            placeholder="eg https://merri-crk.coop/map.png"
            key="image_data_url"
            withAsterisk
            {...form.getInputProps("image_data_url")}
          />
        </Stack>

        <DisplayActionResult result={actionResult} />

        <Stack>
          <Button loading={form.submitting} type="submit">
            Update Map
          </Button>
        </Stack>
      </Stack>
    </form>
  )
}
