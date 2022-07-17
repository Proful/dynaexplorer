import { invoke } from "@tauri-apps/api";
import { Attribute, Item, Table } from "../types";

const getTables = async (): Promise<string[]> => {
  const result = (await invoke("list_tables")) as string[];
  return result;
};

const describeTable = async (tableName: string): Promise<Table> => {
  const result = (await invoke("describe_table", {
    tableName: tableName,
  })) as Table;
  return result;
};

const getItems = async (tableName: string): Promise<Item[]> => {
  const items = (await invoke("list_items", {
    tableName: tableName,
  })) as Item[];
  return items;
};

const getItem = async (
  tableName: string,
  partionKey: Attribute,
  sortKey: Attribute
): Promise<Item> => {
  const item = (await invoke("get_item", {
    tableName,
    partionKey,
    sortKey,
  })) as Item;
  console.log("item: ", item);
  return item;
};

export { getTables, getItems, getItem, describeTable };
