export interface IDocument {
  activeSubstances: string[];
  context: string;
  created: string;
  docType: string;
  fileSize: string;
  name: string;
  product: string;
  url: string;
}

export interface IDocuments {
  count: number;
  documents: IDocument[];
}

export interface IBmgfReport {
  activeSubstances: string[];
  context: string;
  fileName: string;
  fileSize: string;
  fileUrl: string;
  title: string;
  products: string[];
  matrices: string[];
  pbpkModels: string[];
  summary: string;
  url: string;
  pregnancyTrimesters: string[];
  plNumbers: string[];
}

export interface IBmgfReports {
  count: number;
  reports: IBmgfReport[];
}
