import { Button } from "@/components/ui/button";
import { Search } from "lucide-react";
import { useSearch } from "@/context/search";

export function SearchButton() {
  const { setOpen } = useSearch();
  return (
    <Button
      variant="outline"
      className="justify-start text-muted-foreground gap-2"
      onClick={() => setOpen(true)}
    >
      <Search className="h-4 w-4" />
      <span className="flex-1 text-left">Search...</span>
      <kbd className="pointer-events-none inline-flex items-center rounded border bg-muted px-1.5 font-mono text-[10px] font-medium opacity-100">
        Ctrl+K
      </kbd>
    </Button>
  );
}
