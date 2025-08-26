import { Text } from "@mantine/core"
import { format, isThisYear, parseISO } from "date-fns"

interface DateTextProps {
  date: Date | null | string | undefined
  formatString?: string
}

export default function DateText({ date, formatString }: DateTextProps) {
  if (typeof date === "string") {
    date = dateStringToDateTime(date)
  }

  if (!date) return null

  const finalFormatString =
    formatString || "LLL d" + (isThisYear(date) ? "" : ", yyyy")

  const formattedDate = format(date, finalFormatString)
  return <Text span>{formattedDate}</Text>
}

export function dateStringToDateTime(dateString: string | null): Date | null {
  if (!dateString) return null
  return parseISO(dateString)
}

export const formDateFormat = "MMMM d, yyy"
