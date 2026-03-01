import { Box, Stack, Text } from "@mantine/core"

export default function TextWithNewlines({
  text,
}: {
  text: string | null | undefined
}) {
  if (!text) return null

  const lines = text.split("\n\n")

  return (
    <Stack>
      {lines.map((line, index) => (
        <Text key={index}>{line}</Text>
      ))}
    </Stack>
  )
}
