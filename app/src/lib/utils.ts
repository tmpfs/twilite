import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function toFormData(values: Record<string, any>): FormData {
  const formData = new FormData();
  for (const [key, value] of Object.entries(values)) {
    if (value !== undefined && value !== null) {
      formData.append(key, String(value));
    }
  }
  return formData;
}

