import { ActionIcon, Combobox, Group, Text, useCombobox } from "@mantine/core"
import { IconChevronDown } from "@tabler/icons-react"
import { Region } from "../../api/Api"

interface RegionSelectorProps {
  regions: Region[]
  selected: Region | null
  onChange: (region: Region | undefined) => void
}

export function RegionSelector({
  regions,
  selected,
  onChange,
}: RegionSelectorProps) {
  const combobox = useCombobox({
    onDropdownClose: () => combobox.resetSelectedOption(),
  })

  const iconSize = 20

  const options = regions.map((region) => (
    <Combobox.Option value={region.id} key={region.id}>
      {region.name}
    </Combobox.Option>
  ))

  const onIdChange = (id: string) => {
    const region = regions.find((r) => r.id === id)
    onChange(region)
  }

  return (
    <Combobox
      store={combobox}
      width={250}
      position="bottom-start"
      withArrow
      withinPortal={false}
      onOptionSubmit={(val) => {
        onIdChange(val)
        combobox.closeDropdown()
      }}
    >
      <Combobox.Target>
        <Group justify="flex-start" gap={4}>
          <Text span>{selected?.name ?? "Unknown"}</Text>
          <ActionIcon
            onClick={() => combobox.toggleDropdown()}
            variant="transparent"
          >
            <IconChevronDown size={iconSize} />
          </ActionIcon>
        </Group>
      </Combobox.Target>

      <Combobox.Dropdown>
        <Combobox.Options>{options}</Combobox.Options>
      </Combobox.Dropdown>
    </Combobox>
  )
}
