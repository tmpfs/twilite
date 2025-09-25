"use client";
import { usePathname, useRouter } from "next/navigation";
import NoSsr from "@/components/NoSsr";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";

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
  return (
    <div>
      <h1>Wiki index</h1>
    </div>
  );
}

function WikiPage({ pageName }: { pageName: string }) {
  const [data, setData] = useState<string>("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch(`/api/page/${pageName}`, {
          headers: { Accept: "text/html" },
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

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <div className="flex flex-col px-4">
      <div className="flex justify-end space-x-4">
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
        dangerouslySetInnerHTML={{ __html: data }}
      />
    </div>
  );
}
