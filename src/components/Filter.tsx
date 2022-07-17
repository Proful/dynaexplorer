import { Button, Grid, TextInput, Title } from "@mantine/core";
import { useEffect, useState } from "react";
import { describeTable, getItem } from "../backend/dydb";
import { Item, Table } from "../types";

type FilterProps = {
  tableName: string;
  onData: (item: Item) => void;
};

const Filter = ({ tableName, onData }: FilterProps) => {
  const [partionKey, setpartionKey] = useState("");
  const [sortKey, setSortKey] = useState("");
  const [table, setTable] = useState<Table | null>(null);

  useEffect(() => {
    if (tableName && tableName.length > 1) {
      describeTable(tableName).then(setTable);
    }
  }, [tableName]);

  const get = () => {
    if (table && partionKey.length > 0 && sortKey.length > 0) {
      getItem(
        tableName,
        { name: table.partionKeyName, value: partionKey },
        { name: table.sortKeyName, value: sortKey }
      ).then(onData);
    }
  };

  return (
    <div>
      <Title order={3}>Filter</Title>
      <Grid>
        <Grid.Col span={6}>
          <TextInput
            placeholder="Partition key"
            value={partionKey}
            onChange={(e) => setpartionKey(e.currentTarget.value)}
          />
        </Grid.Col>
        <Grid.Col span={6}>
          <TextInput
            placeholder="Sort key"
            value={sortKey}
            onChange={(e) => setSortKey(e.currentTarget.value)}
          />
        </Grid.Col>
        <Grid.Col span={6}>
          <Button variant="outline" onClick={get}>
            Get
          </Button>
        </Grid.Col>
      </Grid>
    </div>
  );
};

export default Filter;
