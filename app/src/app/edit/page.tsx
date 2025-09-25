"use client";

import { PageForm } from "@/components/PageForm";
import { NotFound } from "@/components/NotFound";
import NoSsr from "@/components/NoSsr";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function EditPage() {
  const [data, setData] = useState<string>("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const router = useRouter();
  const pageName = segments[1];

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch(`/api/page/${pageName}`);
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        const content = await res.text();
        setData(content);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, ["pageName"]);

  if (segments.length < 2) {
    return <NotFound />;
  }

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  const page = { pageName, pageContent: data };
  return (
    <NoSsr>
      <PageForm
        page={page}
        edit
        onSuccess={(pageName) => router.push(`/wiki/${pageName}`)}
      />
    </NoSsr>
  );
}
