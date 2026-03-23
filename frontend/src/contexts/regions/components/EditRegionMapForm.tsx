import { Button, Stack, Text, FileInput } from "@mantine/core"
import { useForm } from "@mantine/form"
import {
  ActionPromiseResult,
  DisplayActionResult,
  useOnSubmitWithResult,
} from "../../../components"
import { LatLng, UpdateMapData } from "../../../api/Api"

export interface UpdateMapFormData {
  image_file: File | null
  max_latlng: LatLng
  min_latlng: LatLng
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
      min_latlng: { lat: 0, lng: 0 },
      max_latlng: { lat: 0, lng: 0 },
    },
    validate: {},
  })

  const convertDataAndSubmit = async (
    data: UpdateMapFormData,
  ): Promise<ActionPromiseResult> => {
    const dataUrl = await convertFileToDataUrl(data.image_file)

    const updateData: UpdateMapData = {
      ...data,
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
