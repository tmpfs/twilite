import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { FileWarning } from "lucide-react";
import type React from "react";

export function ErrorScreen({
  title,
  children,
}: {
  title: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <div className="flex h-full w-full items-center justify-center p-4">
      <ErrorAlert title={title}>{children}</ErrorAlert>
    </div>
  );
}

export function ErrorAlert({
  title,
  children,
}: {
  title: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <Alert variant="destructive" className="max-w-prose">
      <FileWarning />
      <AlertTitle className="font-semibold">{title}</AlertTitle>
      <AlertDescription>{children}</AlertDescription>
    </Alert>
  );
}
