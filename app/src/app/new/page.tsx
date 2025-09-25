"use client";

import NoSsr from "@/components/NoSsr";
import { PageForm } from "@/components/PageForm";
import { useRouter, usePathname } from "next/navigation";

export default function NewPage() {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const router = useRouter();
  const page = { pageName: segments[1] || "", pageContent: "" };
  return (
    <NoSsr>
      <PageForm
        page={page}
        onSuccess={(pageName) => router.push(`/wiki/${pageName}`)}
      />
    </NoSsr>
  );
}
