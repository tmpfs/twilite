"use client";

import { Button } from "@/components/ui/button";
import { ThemeToggle } from "@/components/ThemeToggle";
import { useRouter } from "next/navigation";

export function Header({
  children,
}: Readonly<{
  children?: React.ReactNode;
}>) {
  const router = useRouter();

  return (
    <header className="flex items-center justify-between bg-card p-2 text-sm">
      <div>
        Logo
      </div>
      <div className="space-x-4">
        <Button onClick = {() => router.push('/new')}>New page</Button>
        <ThemeToggle/>
      </div>
    </header>
  );
}
