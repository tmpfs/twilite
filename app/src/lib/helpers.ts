import { format, parseISO } from "date-fns";

export function formatUtcDateTime(dateTime: string, fmt?: string): string {
  const date = parseISO(dateTime);
  return format(date, fmt ?? "EEE LLL d hh:mm aa");
}
