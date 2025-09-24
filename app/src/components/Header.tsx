"use client";

import { Button } from "@/components/ui/button";
import { ThemeToggle } from "@/components/ThemeToggle";
import Link from 'next/link';

export function Header({
  children,
}: Readonly<{
  children?: React.ReactNode;
}>) {
  return (
    <header className="flex items-center justify-between bg-card px-4 py-2 text-sm">
      <div>
        <Button asChild variant="link" className="p-0 h-auto">
          <Link href="/">Home</Link>
        </Button>
      </div>
      <div className="space-x-4">
        <Button asChild variant="link" className="p-0 h-auto">
          <Link href="/new">New page</Link>
        </Button>
        <ThemeToggle/>
      </div>
    </header>
  );
}
