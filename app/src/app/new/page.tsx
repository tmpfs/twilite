"use client";

import NoSsr from "@/components/NoSsr";
import { PageForm } from "@/components/PageForm";
import { usePathname } from "next/navigation";
import { useFlashToast } from "@/context/toast";

export default function NewPage() {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const page = { pageName: segments[1] || "", pageContent: "", pageToc: "", updatedAt: "", pageFiles: [] };
  const { flashToastAndNavigate } = useFlashToast();

  const onSuccess = (pageName: string) => {
    flashToastAndNavigate(
      {
        type: "success",
        title: "Page created!",
        description: `Wiki page ${pageName} was created`,
      },
      `/wiki/${pageName}`,
    );
  };

  return (
    <NoSsr>
      <PageForm page={page} onSuccess={onSuccess} />
    </NoSsr>
  );
}
