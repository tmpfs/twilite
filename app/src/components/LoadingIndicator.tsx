import { Loader2 } from "lucide-react";

export function LoadingScreen({ label = "Loading..." }: { label?: string }) {
  return (
    <div className="flex h-full w-full items-center justify-center p-4">
      <LoadingIndicator label={label} />
    </div>
  );
}

export function LoadingIndicator({ label = "Loading..." }: { label?: string }) {
  return (
    <div
      className="flex items-center justify-center gap-2 p-4"
      role="status"
      aria-label={label}
    >
      <Loader2 className="h-5 w-5 animate-spin text-slate-600" aria-hidden />
      <span className="text-sm text-slate-600">{label}</span>
    </div>
  );
}
