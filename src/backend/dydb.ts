import { invoke } from "@tauri-apps/api";
import { Item, Table } from "../types";

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

export { getTables, getItems, describeTable };
