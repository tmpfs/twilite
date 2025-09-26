"use client";
import { usePathname, useRouter } from "next/navigation";
import NoSsr from "@/components/NoSsr";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import type { Page, PagePreview } from "@/lib/model";
import { formatUtcDateTime } from "@/lib/helpers";
import Link from "next/link";

export default function WikiRouter() {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  return (
    <NoSsr>
      {segments.length === 1 ? (
        <WikiIndex />
      ) : (
        <WikiPage pageName={segments[1]} />
      )}
    </NoSsr>
  );
}

function WikiIndex() {
  const [pages, setPages] = useState<PagePreview[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | undefined>();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch(`/api/page/recent`, {
          headers: { Accept: "application/json" },
        });
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        const pages = await res.json();
        console.log(pages);
        setPages(pages);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <div className="flex flex-col w-full p-4 space-y-2">
      <h3>Recent pages</h3>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-6 w-full">
        {pages.map((page) => {
          return (
            <Link
              href={`/wiki/${page.pageName}`}
              key={page.pageName}
              className="no-underline rounded-lg border border-muted shadow p-4 flex flex-col h-256px overflow-hidden"
            >
              <div className="text-muted-foreground">{page.pageName}</div>
              <div className="truncate">{page.previewText}</div>
            </Link>
          );
        })}
      </div>
    </div>
  );
}

function WikiPage({ pageName }: { pageName: string }) {
  const [page, setPage] = useState<Page | undefined>();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | undefined>();
  const router = useRouter();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch(`/api/page/${pageName}?include_files=true`, {
          headers: { Accept: "application/json" },
        });
        if (!res.ok) {
          if (res.status === 404) {
            return router.push(`/new/${pageName}`);
          } else {
            throw new Error(
              `HTTP request failed with status code ${res.status}`,
            );
          }
        }
        const page = await res.json();
        console.log(page);
        setPage(page);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [pageName]);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <div className="flex flex-col px-4">
      <div className="flex justify-between space-x-4">
        <h3 className="mt-2">{pageName}</h3>
        <Button
          onClick={() => router.push(`/edit/${pageName}`)}
          variant="link"
          className="p-0"
        >
          Edit
        </Button>
      </div>
      <Separator />
      <article
        className="prose py-4"
        dangerouslySetInnerHTML={{ __html: page?.pageContent || "" }}
      />
      <Separator />
      <div className="flex text-muted-foreground mt-2">
        {page && <small>{formatUtcDateTime(page?.updatedAt || "")}</small>}
      </div>
    </div>
  );
}
