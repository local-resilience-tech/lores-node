import { Tabs } from "@mantine/core";

export default function ConfigSchemaForm({ schema }: { schema: object }) {
  return (
    <Tabs defaultValue="form">
      <Tabs.List>
        <Tabs.Tab value="form">
          Form
        </Tabs.Tab>
        <Tabs.Tab value="schema">
          Schema
        </Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="form">
        Form view
      </Tabs.Panel>

      <Tabs.Panel value="schema">
        <pre>{JSON.stringify(schema, null, 2)}</pre>
      </Tabs.Panel>
    </Tabs>
  )
}