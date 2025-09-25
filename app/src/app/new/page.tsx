'use client';

import { PageForm } from "@/components/PageForm";
import { useRouter } from 'next/navigation';

export default function NewPage() {
  const router = useRouter();
  const page = { pageName: "", pageContent: "" };
  return (
    <PageForm page={page} onSuccess={(pageName) => router.push(`/wiki/${pageName}`) } />
  );
}
