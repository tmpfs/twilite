"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { useState } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { MarkdownInput } from "@/components/MarkdownEditor";
import { toFormData } from "@/lib/utils";
import type { Page } from "@/lib/model";

const formSchema = z.object({
  pageName: z.string().regex(/^[A-Z][a-zA-Z0-9]*$/, {
    message: "Page name must be in CamelCase (e.g., MyPageName).",
  }),
  pageContent: z.string().refine((val) => val.trim().length > 0, {
    message: "Page content must not be empty or whitespace.",
  }),
});

export function PageForm({
  onSuccess,
  page,
  edit,
}: {
  page: Page;
  edit?: boolean;
  onSuccess: (pageName: string) => void;
}) {
  const [status, setStatus] = useState<
    "idle" | "loading" | "success" | "error"
  >("idle");
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      pageName: page.pageName,
      pageContent: page.pageContent,
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setStatus("loading");
    const formData = toFormData(values);
    try {
      const res = await fetch("/api/page", {
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
        <div className={`flex ${edit ? "justify-between" : "justify-end"}`}>
          {edit && <Button variant="destructive">Delete</Button>}
          <Button type="submit">Save</Button>
        </div>
      </form>
    </Form>
  );
}
