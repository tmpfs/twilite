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
import type { SearchResult } from "@/lib/model";

export function SearchMenu() {
  const [query, setQuery] = useState<string>("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const { open, setOpen } = useSearch();

  useEffect(() => {
    // if (query.length < 3) return;

    const fetchData = async () => {
      try {
        const res = await fetch(`/api/search?q=${query}`, {
          headers: { Accept: "application/json" },
        });
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        const results = await res.json();
        setResults(results);
      } catch (err: any) {
        // setError(err.message);
      } finally {
        // setLoading(false);
      }
    };

    fetchData();
  }, [query]);

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <Command shouldFilter={results.length === 0}>
        <CommandInput
          placeholder="Type a command or search..."
          value={query}
          onValueChange={setQuery}
        />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          {results.length > 0 && (
            <CommandGroup heading={`Search Results (${results.length})`}>
              {results.map((res) => {
                return (
                  <CommandItem key={res.rowId}>
                    <span>{res.title}</span>
                    <span>{res.body}</span>
                  </CommandItem>
                );
              })}
            </CommandGroup>
          )}
          <CommandGroup heading="Suggestions">
            <Link href="/wiki">
              <CommandItem>Wiki</CommandItem>
            </Link>
          </CommandGroup>
        </CommandList>
      </Command>
    </CommandDialog>
  );
}
