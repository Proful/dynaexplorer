import { invoke } from "@tauri-apps/api";
import { Item } from "../types";

const getTables = async (): Promise<string[]> => {
  const result = (await invoke("list_tables")) as string[];
  return result;
};

const getItems = async (tableName: string): Promise<Item[]> => {
  const rows = (await invoke("list_items", {
    tableName: tableName,
  })) as string[];

  let items: Item[] = [];

  for (const row of rows) {
    const kv_list = row.split("~~");
    let row_data: Item = {};
    for (const kv of kv_list) {
      const [k, v] = kv.split("::");
      row_data[k] = v;
    }
    items.push(row_data);
  }
  return items;
};

export { getTables, getItems };
