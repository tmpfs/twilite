"use client";
import { usePathname, useRouter } from "next/navigation";
import NoSsr from "@/components/NoSsr";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { LoadingScreen } from "@/components/LoadingIndicator";
import type { Page, PagePreview } from "@/lib/model";
import { formatUtcDateTime } from "@/lib/helpers";
import Link from "next/link";
import { useFetchWithDelay } from "@/hooks/fetch";
import { Edit } from "lucide-react";
import { ErrorScreen } from "@/components/ErrorScreen";
import { WithTableOfContents } from "@/components/WithTableOfContents";
import { HeroGallery } from '@/components/HeroGallery';

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
  const state = useFetchWithDelay(
    () =>
      fetch(`/api/page/recent`, {
        headers: { Accept: "application/json" },
      }).then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        return res.json();
      }),
    [],
  );

  if (state.status === "loading") {
    return <LoadingScreen />;
  } else if (state.status === "error") {
    return (
      <ErrorScreen title="Network error">{state.error.message}</ErrorScreen>
    );
  } else if (state.status === "success") {
    const pages = state.data as PagePreview[];
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
  } else {
    throw new Error("unsupported fetch loader state");
  }
}

function WikiPage({ pageName }: { pageName: string }) {
  const router = useRouter();

  const state = useFetchWithDelay(
    () =>
      fetch(`/api/page/${pageName}?include_files=true`, {
        headers: { Accept: "application/json" },
      }).then((res) => {
        if (!res.ok) {
          if (res.status === 404) {
            return router.push(`/new/${pageName}`);
          } else {
            throw new Error(
              `HTTP request failed with status code ${res.status}`,
            );
          }
        }
        return res.json();
      }),
    [pageName],
  );

  if (state.status === "loading") {
    return <LoadingScreen />;
  } else if (state.status === "error") {
    return (
      <ErrorScreen title="Network error">{state.error.message}</ErrorScreen>
    );
  } else if (state.status === "success") {
    
    const page = state.data as Page;

    const WikiPageContents = () => {
      return (<aside className="prose py-2">
        <div
          dangerouslySetInnerHTML={{ __html: page?.pageToc || "" }}
        ></div>
      </aside>);
    }

    const images = (page.pageFiles || []).filter((file) => file.contentType.startsWith("image/jpeg")).map((file) => {
      return {url: `/files/${file.fileUuid}`};
    });

    return (
        <div className="flex flex-col px-4">
          <div className="flex justify-between space-x-4 items-center">
            <h3 className="mt-8 mb-4 text-4xl font-semibold tracking-tight">
              {pageName}
            </h3>
            <Link href={`/edit/${pageName}`}>
              <Button asChild variant="secondary">
                <div className="flex">
                  <Edit />
                  <span>Edit</span>
                </div>
              </Button>
            </Link>
          </div>
          <Separator />
          <WithTableOfContents
            contents={page.pageToc ? <WikiPageContents /> : null}
          >
            <div className="flex flex-col">
              { images.length > 0 && <HeroGallery images={images} />}
              <article
                className="prose py-4"
                dangerouslySetInnerHTML={{ __html: page?.pageContent || "" }}
              />
            </div>
          </WithTableOfContents>
          <Separator />
          <div className="flex text-muted-foreground mt-2 justify-end">
            {page && <small className="text-sm">{formatUtcDateTime(page?.updatedAt || "")}</small>}
          </div>
        </div>
    );
  } else {
    throw new Error("unsupported fetch loader state");
  }
}
