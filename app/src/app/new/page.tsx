'use client';

import { PageForm } from "@/components/PageForm";
import { useRouter } from 'next/navigation';

export default function NewPage() {
  const router = useRouter();

  return (
    <PageForm onSuccess={(pageName) => router.push(`/wiki/${pageName}`) } />
  );
}
