"use client";

import { Button } from "@/components/ui/button";
import Link from "next/link";
import { ThemeToggle } from "@/components/ThemeToggle";
import { Plus } from "lucide-react";
import { SearchButton } from "@/components/SearchButton";

export function Header() {
  return (
    <header className="flex items-center justify-between bg-card px-4 py-2">
      <div className="shrink">
        <Button asChild variant="link" className="p-0 h-auto text-lg">
          <Link href="/wiki">Wiki</Link>
        </Button>
      </div>
      <div className="space-x-4 flex">
        <Button asChild variant="secondary">
          <div className="flex">
            <Plus />
            <Link href="/new">New page</Link>
          </div>
        </Button>
        <SearchButton />
        <ThemeToggle />
      </div>
    </header>
  );
}
