export interface IProduct {
  name: string;
  count?: number;
}

export interface ISubstance extends IProduct {
  products?: IProduct[];
}

export function isSubstance(obj: any): obj is ISubstance {
  return obj.products !== undefined && obj.count !== undefined;
}

export function isIndex(obj: any): obj is ISubstance {
  return obj.products === undefined && obj.count === undefined;
}

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
