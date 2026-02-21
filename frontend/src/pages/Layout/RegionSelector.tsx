import { ActionIcon, Box, Combobox, Text, useCombobox } from "@mantine/core"
import { IconChevronDown } from "@tabler/icons-react"
import { useState } from "react"
import { Region } from "../../api/Api"

const groceries = [
  "🍎 Apples",
  "🍌 Bananas",
  "🥦 Broccoli",
  "🥕 Carrots",
  "🍫 Chocolate",
  "🍇 Grapes",
]

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
    <>
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
          <ActionIcon
            onClick={() => combobox.toggleDropdown()}
            variant="transparent"
          >
            <IconChevronDown size={iconSize} />
          </ActionIcon>
        </Combobox.Target>

        <Combobox.Dropdown>
          <Combobox.Options>{options}</Combobox.Options>
        </Combobox.Dropdown>
      </Combobox>

      <Box mt="xs">
        <Text span size="sm" c="dimmed">
          Selected item:{" "}
        </Text>

        <Text span size="sm">
          {selected ? selected.name : "Nothing selected"}
        </Text>
      </Box>
    </>
  )
}
