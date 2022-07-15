export interface Attribute {
  name: string;
  value: string;
}

export interface Item {
  primaryKey: Attribute;
  sortKey: Attribute;
  attributes: Attribute[];
}
