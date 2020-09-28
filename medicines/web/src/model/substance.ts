import { IProduct } from './product';

export interface ISubstance extends IProduct {
  products?: IProduct[];
}

export function isSubstance(obj: any): obj is ISubstance {
  return obj.products !== undefined && obj.count !== undefined;
}

export function isIndex(obj: any): obj is ISubstance {
  return obj.products === undefined && obj.count === undefined;
}
