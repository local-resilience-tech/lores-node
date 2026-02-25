import { ActionIcon, Combobox, Group, Text, useCombobox } from "@mantine/core"
import { IconChevronDown } from "@tabler/icons-react"
import { Region } from "../../api/Api"
import { useNavigate } from "react-router-dom"

interface RegionSelectorProps {
  regions: Region[]
  selected: Region | null
  onChange: (region: Region | undefined) => void
  addNewPath?: string
}

const ADD_NEW_VALUE = "__add_new__"

export function RegionSelector({
  regions,
  selected,
  onChange,
  addNewPath,
}: RegionSelectorProps) {
  const navigate = useNavigate()

  const combobox = useCombobox({
    onDropdownClose: () => combobox.resetSelectedOption(),
  })

  const iconSize = 20

  const options = regions.map((region) => (
    <Combobox.Option value={region.id} key={region.id}>
      {region.name}
    </Combobox.Option>
  ))

  if (addNewPath) {
    options.push(
      <Combobox.Option value={ADD_NEW_VALUE} key={ADD_NEW_VALUE}>
        <Text c="dimmed">Join another region</Text>
      </Combobox.Option>,
    )
  }

  const onIdChange = (id: string) => {
    if (addNewPath !== undefined && id === ADD_NEW_VALUE) {
      navigate(addNewPath)
    } else {
      const region = regions.find((r) => r.id === id)
      onChange(region)
    }
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
