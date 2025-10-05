import { RJSFSchema } from "@rjsf/utils"
import validator from "@rjsf/validator-ajv8"
import Form from "@rjsf/mantine"
import { FormEvent } from "react"
import { Tabs } from "@mantine/core"
import { ActionPromiseResult, useOnSubmitWithResult } from "../ActionResult"

export interface JsonSchemaFormProps {
  displaySchema?: boolean
  schema: RJSFSchema
  initialData?: any
  onSubmit: (data: any) => Promise<ActionPromiseResult>
}

export default function JsonSchemaForm({
  schema,
  displaySchema,
  initialData,
  onSubmit,
}: JsonSchemaFormProps) {
  const log = (type: any) => console.log.bind(console, type)

  const [actionResult, onSubmitWithResult] =
    useOnSubmitWithResult<any>(onSubmit)

  const handleSubmit = (data: any, event: FormEvent<any>) => {
    onSubmitWithResult(data.formData)
  }

  const form = (
    <Form
      schema={schema}
      formData={initialData}
      validator={validator}
      onSubmit={handleSubmit}
      onError={log("errors")}
    />
  )

  if (displaySchema) {
    return (
      <Tabs defaultValue="form">
        <Tabs.List>
          <Tabs.Tab value="form">Form</Tabs.Tab>
          <Tabs.Tab value="schema">Schema</Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="form" pt="md">
          {form}
        </Tabs.Panel>

        <Tabs.Panel value="schema" pt="md">
          <pre>{JSON.stringify(schema, null, 2)}</pre>
        </Tabs.Panel>
      </Tabs>
    )
  } else {
    return form
  }
}
