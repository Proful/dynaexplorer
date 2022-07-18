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
  attributeDefinitions: AttributeDefinition[];
  tableName: string;
  keySchema: KeySchema[];
  tableStatus: string;
  creationDateTime: string;
  provisionedThroughput: ProvisionedThroughput;
  tableSizeBytes: number;
  itemCount: number;
  tableArn: string;
}

export interface AttributeDefinition {
  attributeName: string;
  attributeType: string;
}

export interface KeySchema {
  attributeName: string;
  keyType: string;
}

export interface ProvisionedThroughput {
  lastIncreaseDateTime: string;
  lastDecreaseDateTime: string;
  numberOfDecreasesToday: number;
  readCapacityUnits: number;
  writeCapacityUnits: number;
}
