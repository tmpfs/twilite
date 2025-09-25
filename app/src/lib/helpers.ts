import { format, parseISO } from "date-fns";

export function formatUtcDateTime(dateTime: string, fmt?: string): string {
  const date = parseISO(dateTime);
  return format(date, fmt ?? "EEE LLL d hh:mm:SS");
}

export function formatUtcMonthYear(dateTime: string): string {
  return formatUtcDateTime(dateTime, "MM/yyyy");
}

export function formatUtcDayMonthYear(dateTime: string): string {
  return formatUtcDateTime(dateTime, "dd/MM/yyyy");
}
