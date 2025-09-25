"use client";

import { Button } from "@/components/ui/button";
import Link from "next/link";
import { ThemeToggle } from "@/components/ThemeToggle";
import { Plus } from "lucide-react";

export function Header() {
  return (
    <header className="flex items-center justify-between bg-card px-4 py-2 text-sm">
      <div>
        <Button asChild variant="link" className="p-0 h-auto">
          <Link href="/wiki">Wiki</Link>
        </Button>
      </div>
      <div className="space-x-4">
        <Button asChild variant="secondary">
          <div className="flex">
            <Plus />
            <Link href="/new">New page</Link>
          </div>
        </Button>
        <ThemeToggle />
      </div>
    </header>
  );
}
