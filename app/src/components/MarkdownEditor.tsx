'use client';

import { MinimalTiptap } from '@/components/ui/shadcn-io/minimal-tiptap';
import NoSsr from '@/components/NoSsr';
import * as React from 'react';
import { useState } from 'react';

export interface MarkdownInputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string
}

const MarkdownInput = React.forwardRef<HTMLInputElement, MarkdownInputProps>(
  ({ label, className, value, onChange, ...props }, ref) => {
    return (
      <MarkdownEditor value={value} onChange={onChange} />
    )
  }
)
MarkdownInput.displayName = "MarkdownInput"

function MarkdownEditor({value, onChange}: {value: any, onChange: any}) {
  return (
    <div className="flex items-center justify-center">
      <div className="w-full">
        <NoSsr>
          <MinimalTiptap
            content={value}
            onChange={onChange}
            placeholder="Start typing your content here..."
            className="min-h-[400px]"
          />
        </NoSsr>
      </div>
    </div>
  );
}

export { MarkdownInput, MarkdownEditor }

