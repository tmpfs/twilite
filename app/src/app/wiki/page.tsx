'use client';
import { usePathname } from "next/navigation";
import NoSsr from '@/components/NoSsr';
import { useEffect, useState } from 'react';

export default function WikiRouter() {
  const pathname = usePathname(); 
  const segments = pathname.split("/").filter(Boolean); 
  return (
    <NoSsr>
      { segments.length === 1 ? <WikiIndex /> : <WikiPage pageName={segments[1]} /> }
    </NoSsr>
  )
}

function WikiIndex() {
  return (
    <div>
      <h1>Wiki index</h1>
    </div>
  )
}

function WikiPage({pageName}: {pageName: string}) {
  const [data, setData] = useState<string>("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch(`/api/page/${pageName}`);
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        const json = await res.text();
        setData(json);
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

    <article
      className="prose p-4"
      dangerouslySetInnerHTML={{ __html: data }}
    />
  )
}
