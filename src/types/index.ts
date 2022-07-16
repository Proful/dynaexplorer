export interface Attribute {
  name: string;
  value: string;
  kind: string;
}

export interface Item {
  primaryKey: Attribute;
  sortKey: Attribute;
  attributes: Attribute[];
}

export interface Table {
  name: string;
  primaryKeyName: string;
  sortKeyName: string;
  itemCount: number;
}
