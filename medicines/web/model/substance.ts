export interface IProduct {
  name: string;
  count?: number;
}

export interface ISubstance extends IProduct {
  products: IProduct[];
}
