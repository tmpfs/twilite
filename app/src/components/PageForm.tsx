"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { useState } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { MarkdownInput } from "@/components/MarkdownEditor";
import { toFormData } from "@/lib/utils";
import type { Page } from "@/lib/model";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import {
  Dropzone,
  DropzoneContent,
  DropzoneEmptyState,
} from "@/components/ui/shadcn-io/dropzone";

const formSchema = z.object({
  pageName: z.string().regex(/^[A-Z][a-zA-Z0-9]*$/, {
    message: "Page name must be in CamelCase (e.g., MyPageName).",
  }),
  pageContent: z.string().refine((val) => val.trim().length > 0, {
    message: "Page content must not be empty or whitespace.",
  }),
  files: z.array(z.string()),
});

export function PageForm({
  onSuccess,
  page,
  edit,
  onDelete,
  onCancel,
}: {
  page: Page;
  edit?: boolean;
  onDelete?: () => void;
  onCancel?: () => void;
  onSuccess: (pageName: string) => void;
}) {
  const [status, setStatus] = useState<
    "idle" | "loading" | "success" | "error"
  >("idle");
  const [files, setFiles] = useState<File[]>([]);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      pageName: page.pageName,
      pageContent: page.pageContent,
      files: [],
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setStatus("loading");
    const formData = toFormData(values);

    files.forEach((file) => {
      formData.append("uploads", file, file.name);
    });

    try {
      const url = edit ? `/api/page/${page.pageUuid}` : "/api/page";
      const res = await fetch(url, {
        method: edit ? "PUT" : "POST",
        body: formData,
      });

      if (!res.ok) throw new Error("Network error");

      console.log(res);

      setStatus("success");
      onSuccess(values.pageName);
    } catch (err) {
      console.error(err);
      setStatus("error");
    }
  }

  const cancel = (e: any) => {
    e.preventDefault();
    if (onCancel) onCancel();
  };

  const deletePage = async (e: any) => {
    e.preventDefault();
    try {
      const res = await fetch(`/api/page/${page.pageUuid}`, {
        method: "DELETE",
      });

      if (!res.ok) throw new Error("Network error");

      console.log(res);

      setStatus("success");
      if (onDelete) onDelete();
    } catch (err) {
      console.error(err);
      setStatus("error");
    }
  };

  const handleDrop = (files: File[]) => {
    console.log(files);
    setFiles(files);
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8 m-4">
        <FormField
          control={form.control}
          name="pageName"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <Input
                  placeholder="Name of the wiki page, eg: MyWikiPage"
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="pageContent"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <MarkdownInput {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="files"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <Dropzone
                  accept={{ "image/*": [] }}
                  maxFiles={10}
                  maxSize={1024 * 1024 * 10}
                  minSize={1024}
                  onDrop={handleDrop}
                  onError={console.error}
                  src={files}
                >
                  <DropzoneEmptyState />
                  <DropzoneContent />
                </Dropzone>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <div className={`flex ${edit ? "justify-between" : "justify-end"}`}>
          {edit && (
            <AlertDialog>
              <AlertDialogTrigger>
                <Button type="button" variant="destructive">
                  Delete
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                  <AlertDialogDescription>
                    This action cannot be undone. This will permanently delete
                    the wiki page.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogCancel>Cancel</AlertDialogCancel>
                  <AlertDialogAction onClick={deletePage}>
                    Delete
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          )}
          <div className="space-x-4">
            {edit && (
              <Button type="button" variant="secondary" onClick={cancel}>
                Cancel
              </Button>
            )}
            <Button type="submit">Save</Button>
          </div>
        </div>
      </form>
    </Form>
  );
}
