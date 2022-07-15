import { invoke } from "@tauri-apps/api";
import { Item } from "../types";

const getTables = async (): Promise<string[]> => {
  const result = (await invoke("list_tables")) as string[];
  return result;
};

const describeTable = async (tableName: string): Promise<string[]> => {
  const result = (await invoke("describe_table")) as string[];
  return result;
};

const getItems = async (tableName: string): Promise<Item[]> => {
  const items = (await invoke("list_items", {
    tableName: tableName,
  })) as Item[];
  return items;
};

export { getTables, getItems, describeTable };
