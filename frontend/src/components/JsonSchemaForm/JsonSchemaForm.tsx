
import { RegistryWidgetsType, RJSFSchema, WidgetProps } from '@rjsf/utils';
import validator from '@rjsf/validator-ajv8';
import Form from '@rjsf/mantine';

export default function JsonSchemaForm({ schema }: { schema: RJSFSchema }) {
  const log = (type: any) => console.log.bind(console, type);


  return (<Form
    schema={schema}
    validator={validator}
    onChange={log('changed')}
    onSubmit={log('submitted')}
    onError={log('errors')}
  />)
}