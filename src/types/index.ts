export interface Attribute {
  name: string;
  value: string;
}

export interface Item {
  partionKey: Attribute;
  sortKey: Attribute;
  attributes: Attribute[];
}

export interface Table {
  name: string;
  partionKeyName: string;
  sortKeyName: string;
  itemCount: number;
}
