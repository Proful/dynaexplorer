import { useEffect, useState } from "react";
import { AppShell, Navbar, Header, Title } from "@mantine/core";
import { getItems, getTables } from "./backend/dydb";
import ItemList from "./components/ItemList";
import TableList from "./components/TableList";
import type { Item } from "./types";
import Filter from "./components/Filter";

function App() {
  const [tables, setTables] = useState<string[]>([]);
  const [items, setItems] = useState<Item[]>([]);
  const [selectedTable, setSelectedTable] = useState<string>("");

  useEffect(() => {
    getTables().then((ts) => setTables(ts));
  }, []);

  const onTableSelected = (tableName: string) => {
    setSelectedTable(tableName);
    getItems(tableName).then((rows) => setItems(rows));
  };

  return (
    <AppShell
      padding="md"
      navbar={
        <Navbar width={{ base: 300 }} height={500} p="xs">
          <TableList
            tables={tables}
            onTableSelected={onTableSelected}
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
      <Filter
        tableName={selectedTable}
        onData={(item) => {
          if (item) {
            setItems([item]);
          } else {
            setItems([]);
          }
        }}
      />
      <ItemList items={items} />
    </AppShell>
  );
}

export default App;
