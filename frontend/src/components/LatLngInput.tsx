import { Group, Input, InputWrapperProps, NumberInput } from "@mantine/core"

export interface EditableLatLng {
  lat: number | null | undefined
  lng: number | null | undefined
}

export function emptyEditableLatLng(): EditableLatLng {
  return { lat: null, lng: null }
}

export function toLatLng(editable: EditableLatLng): {
  lat: number
  lng: number
} {
  return {
    lat: editable.lat ?? 0,
    lng: editable.lng ?? 0,
  }
}

export function validateLatLng(
  value: EditableLatLng | null | undefined,
): string | null {
  if (!value) {
    return "Latitude and longitude are required"
  }
  if (value.lat === null || value.lat === undefined) {
    return "Latitude is required"
  }
  if (value.lng === null || value.lng === undefined) {
    return "Longitude is required"
  }
  if (value.lat < -90 || value.lat > 90) {
    return "Latitude must be between -90 and 90"
  }
  if (value.lng < -180 || value.lng > 180) {
    return "Longitude must be between -180 and 180"
  }
  return null
}

type LatLngInputProps = Omit<InputWrapperProps, "onChange"> & {
  value?: EditableLatLng
  onChange: (value: EditableLatLng) => void
}

export default function LatLngInput({
  label,
  description,
  withAsterisk,
  error,
  value,
  onChange,
}: LatLngInputProps) {
  return (
    <Input.Wrapper
      label={label}
      description={description}
      error={error}
      withAsterisk={withAsterisk}
    >
      <Group>
        <NumberInput
          placeholder="Latitude"
          value={value?.lat ?? ""}
          onChange={(val) =>
            onChange({
              lat: val !== null ? Number(val) : null,
              lng: value?.lng,
            })
          }
        />
        <NumberInput
          placeholder="Longitude"
          value={value?.lng ?? ""}
          onChange={(val) =>
            onChange({
              lat: value?.lat,
              lng: val !== null ? Number(val) : null,
            })
          }
        />
      </Group>
    </Input.Wrapper>
  )
}
