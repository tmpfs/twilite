import React from 'react';

export function WithTableOfContents({children, contents}: {children: React.ReactNode, contents?: React.ReactNode}) {
  if (!contents) return children;
  return (
    <div className="grid grid-cols-1 lg:grid-cols-[300px_1fr] gap-2">
      {contents}
      {children}
    </div>
  )
}
