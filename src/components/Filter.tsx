import { Button, Grid, Popover, TextInput, Title } from "@mantine/core";
import { useEffect, useState } from "react";
import JSONPretty from "react-json-pretty";
import { describeTable, getItem } from "../backend/dydb";
import { Item, Table } from "../types";

type FilterProps = {
  tableName: string;
  onData: (item: Item) => void;
};

const Filter = ({ tableName, onData }: FilterProps) => {
  const [opened, setOpened] = useState(false);
  const [partionKey, setPartionKey] = useState("");
  const [sortKey, setSortKey] = useState("");
  const [table, setTable] = useState<Table | null>(null);

  useEffect(() => {
    if (tableName && tableName.length > 1) {
      describeTable(tableName).then((t) => {
        console.log(t);
        setTable(t);
      });
    }
  }, [tableName]);

  const get = () => {
    if (table && partionKey.length > 0 && sortKey.length > 0) {
      const partionKeyName =
        table.keySchema.find((k) => k.keyType === "HASH")?.attributeName || "";
      const sortKeyName =
        table.keySchema.find((k) => k.keyType === "RANGE")?.attributeName || "";
      getItem(
        tableName,
        { name: partionKeyName, value: partionKey },
        { name: sortKeyName, value: sortKey }
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
            onChange={(e) => setPartionKey(e.currentTarget.value)}
          />
        </Grid.Col>
        <Grid.Col span={6}>
          <TextInput
            placeholder="Sort key"
            value={sortKey}
            onChange={(e) => setSortKey(e.currentTarget.value)}
          />
        </Grid.Col>
        <Grid.Col span={3}>
          <Button variant="outline" onClick={get}>
            Get
          </Button>
        </Grid.Col>
        <Grid.Col span={3}>
          <Popover
            //@ts-ignore
            opened={opened}
            onClose={() => setOpened(false)}
            target={
              <Button variant="outline" onClick={() => setOpened(!opened)}>
                Info
              </Button>
            }
            width={860}
            position="bottom"
            withArrow
          >
            <JSONPretty
              data={JSON.stringify(table)}
              style={{ background: "transparent " }}
            />
          </Popover>
        </Grid.Col>
      </Grid>
    </div>
  );
};

export default Filter;
