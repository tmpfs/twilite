"use client";

import { PageForm } from "@/components/PageForm";
import { NotFound } from "@/components/NotFound";
import NoSsr from "@/components/NoSsr";
import { usePathname, useRouter } from "next/navigation";
import type { Page } from "@/lib/model";
import { toast } from "sonner";
import { useFlashToast } from "@/context/toast";
import { LoadingScreen } from "@/components/LoadingIndicator";
import { useFetchWithDelay } from "@/hooks/fetch";
import { ErrorScreen } from "@/components/ErrorScreen";

export default function EditPage() {
  const { flashToastAndNavigate } = useFlashToast();
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const router = useRouter();
  const pageName = segments[1];

  const state = useFetchWithDelay(
    () =>
      fetch(`/api/page/${pageName}`, {
        headers: { Accept: "application/json" },
      }).then((res) => {
        if (!res.ok) {
          throw new Error(`HTTP request failed with status code ${res.status}`);
        }
        return res.json();
      }),
    [pageName],
  );

  const onDelete = () => {
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
    router.push(`/wiki/${pageName}`);
  };

  /*
  if (segments.length < 2) {
    return <NotFound />;
  }
  */

  if (state.status === "loading") {
    return <LoadingScreen />;
  } else if (state.status === "error") {
    return (
      <ErrorScreen title="Network error">{state.error.message}</ErrorScreen>
    );
  } else if (state.status === "success") {
    const page = state.data as Page;
    return (
      <NoSsr>
        <PageForm
          page={page}
          edit
          onDelete={onDelete}
          onCancel={onCancel}
          onSuccess={onSuccess}
        />
      </NoSsr>
    );
  } else {
    throw new Error("unsupported fetch loader state");
  }
}
