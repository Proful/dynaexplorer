import { useEffect, useState } from "react";
import { getItems, getTables } from "./backend/dydb";
import ItemList from "./components/ItemList";
import TableList from "./components/TableList";
import type { Item } from "./types";

function App() {
  const [tables, setTables] = useState<string[]>([]);
  const [items, setItems] = useState<Item[]>([]);

  useEffect(() => {
    getTables().then((ts) => setTables(ts));
  }, []);

  const listItems = (tableName: string) => {
    getItems(tableName).then((rows) => setItems(rows));
  };

  return (
    <div>
      <TableList tables={tables} onTableSelected={listItems} />
      <ItemList items={items} />
    </div>
  );
}

export default App;
