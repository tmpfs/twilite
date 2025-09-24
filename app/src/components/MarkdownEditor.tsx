
"use client";

import React, { useState, useEffect } from "react";
import NoSsr from "@/components/NoSsr";
import MDEditor from '@uiw/react-md-editor';

export function MarkdownEditor() {
  const [value, setValue] = useState<undefined | string>("");

  return (
    <div className="w-full max-w-2xl mx-auto">
      <NoSsr>
        <MDEditor
          value={value}
          onChange={setValue}
        />
      </NoSsr>
    </div>
  );
}
