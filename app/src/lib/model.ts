export type Page = {
  pageUuid?: string;
  pageName: string;
  pageContent: string;
  updatedAt: string;
};

export type PagePreview = {
  pageUuid?: string;
  pageName: string;
  previewText: string;
  updatedAt: string;
};
