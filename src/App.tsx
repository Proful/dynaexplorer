import { useEffect, useState } from "react";
import { AppShell, Navbar, Header, Title } from "@mantine/core";
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
    <AppShell
      padding="md"
      navbar={
        <Navbar width={{ base: 300 }} height={500} p="xs">
          <TableList
            tables={tables}
            onTableSelected={listItems}
            onClear={() => setItems([])}
          />
        </Navbar>
      }
      header={
        <Header height={60} p="xs">
          <Title order={6}>Dynaexplorer</Title>
        </Header>
      }
      styles={(theme) => ({
        root: {
          backgroundColor: theme.colors.dark[8],
          color: theme.colors.gray[6],
        },
        main: {
          backgroundColor: theme.colors.dark[8],
        },
      })}
    >
      <ItemList items={items} />
    </AppShell>
  );
}

export default App;
