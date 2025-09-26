"use client";

import { useState, useEffect } from "react";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
} from "@/components/ui/command";
import Link from "next/link";
import { useSearch } from "@/context/search";

export function SearchMenu() {
  const { open, setOpen } = useSearch();
  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder="Type a command or search..." />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup heading="Suggestions">
          <Link href="/wiki">
            <CommandItem>Wiki</CommandItem>
          </Link>
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
