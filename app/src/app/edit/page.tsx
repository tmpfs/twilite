"use client";

import { PageForm } from "@/components/PageForm";
import { NotFound } from "@/components/NotFound";
import NoSsr from "@/components/NoSsr";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import type { Page } from "@/lib/model";
import { toast } from "sonner";
import { useFlashToast } from "@/context/toast";

export default function EditPage() {
  const [page, setPage] = useState<Page | undefined>();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { flashToastAndNavigate } = useFlashToast();

  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const router = useRouter();
  const pageName = segments[1];

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch(`/api/page/${pageName}`, {
          headers: { Accept: "application/json" },
        });
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        const page = await res.json();
        setPage(page);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, ["pageName"]);

  const onDelete = () => {
    toast(`Wiki page ${page?.pageName} deleted`, { duration: 15000 });
    router.push("/");

    flashToastAndNavigate(
      {
        type: "success",
        title: "Page deleted!",
        description: `Wiki page ${pageName} was deleted`,
      },
      `/wiki/`,
    );
  };

  const onSuccess = (pageName: string) => {
    flashToastAndNavigate(
      {
        type: "success",
        title: "Page updated!",
        description: `Wiki page ${pageName} was updated`,
      },
      `/wiki/${pageName}`,
    );
  };

  const onCancel = () => {
    router.push(`/wiki/${page?.pageName}`);
  };

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  if (segments.length < 2) {
    return <NotFound />;
  }

  return (
    <NoSsr>
      <PageForm
        page={page as Page}
        edit
        onDelete={onDelete}
        onCancel={onCancel}
        onSuccess={onSuccess}
      />
    </NoSsr>
  );
}
