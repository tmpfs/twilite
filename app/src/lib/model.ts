export type Page = {
  pageUuid?: string;
  pageName: string;
  pageContent: string;
  pageToc: string;
  updatedAt: string;
};

export type PagePreview = {
  pageUuid?: string;
  pageName: string;
  previewText: string;
  updatedAt: string;
};

export type SearchResult = {
  rowId: number;
  title: string;
  body: string;
};
