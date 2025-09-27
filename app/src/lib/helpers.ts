import { format, parseISO } from "date-fns";

export function formatUtcDateTime(dateTime: string, fmt?: string): string {
  const date = parseISO(dateTime);
  return format(date, fmt ?? "EEE LLL d hh:mm aa");
}

export function scrollToTop() {
  const topElement = document.getElementById("top");
  if (topElement) {
    topElement.scrollIntoView({ behavior: "smooth" });
  } else {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }
}
