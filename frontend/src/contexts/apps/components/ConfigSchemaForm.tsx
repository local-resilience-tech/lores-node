import { JsonSchemaForm } from '../../../components';

export default function ConfigSchemaForm({ schema }: { schema: object }) {
  return <JsonSchemaForm schema={schema} />
}