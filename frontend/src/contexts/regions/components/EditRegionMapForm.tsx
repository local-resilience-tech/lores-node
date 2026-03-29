import { Button, Stack, Text, FileInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import { UpdateMapData } from "../../../api/Api"
import LatLngInput, {
  EditableLatLng,
  emptyEditableLatLng,
  toLatLng,
  validateLatLng,
} from "../../../components/LatLngInput"

export interface UpdateMapFormData {
  image_file: File | null
  max_latlng: EditableLatLng
  min_latlng: EditableLatLng
  region_id: string
}

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

  const form = useForm<UpdateMapFormData>({
    mode: "controlled",
    initialValues: {
      region_id: regionId,
      image_file: null,
      min_latlng: emptyEditableLatLng(),
      max_latlng: emptyEditableLatLng(),
    },
    validate: {
      min_latlng: validateLatLng,
      max_latlng: validateLatLng,
    },
  })

  const convertDataAndSubmit = async (
    data: UpdateMapFormData,
  ): Promise<ActionPromiseResult> => {
    const dataUrl = await convertFileToDataUrl(data.image_file)

    const updateData: UpdateMapData = {
      ...data,
      min_latlng: toLatLng(data.min_latlng)!,
      max_latlng: toLatLng(data.max_latlng)!,
      image_data_url: dataUrl || "",
    }
    return onSubmitWithResult(updateData)
  }

  return (
    <form onSubmit={form.onSubmit(convertDataAndSubmit)}>
      <Stack gap="lg">
        <Text>
          When you edit the map for a region, you can update the image and the
          geographical boundaries.
        </Text>

        <Stack>
          <FileInput
            label="Image File"
            description="The image file representing your region"
            placeholder="Choose an image file"
            key="image_file"
            withAsterisk
            {...form.getInputProps("image_file")}
          />

          <LatLngInput
            label="Min Lat & Lng"
            description="The minimum latitude and longitude for the region"
            key="min_latlng"
            withAsterisk
            {...form.getInputProps("min_latlng")}
          />

          <LatLngInput
            label="Max Lat & Lng"
            description="The maximum latitude and longitude for the region"
            key="max_latlng"
            withAsterisk
            {...form.getInputProps("max_latlng")}
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

async function convertFileToDataUrl(
  file: File | null,
): Promise<string | null | undefined> {
  if (!file) return null

  return new Promise((resolve) => {
    const reader = new FileReader()

    reader.addEventListener("load", () => {
      resolve(reader.result as string)
    })

    reader.readAsDataURL(file)
  })
}
