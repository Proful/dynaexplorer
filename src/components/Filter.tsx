import { Button, Grid, TextInput, Title } from "@mantine/core";
import { useEffect, useState } from "react";
import { describeTable } from "../backend/dydb";

const Filter = ({ tableName }: { tableName: string }) => {
  const [primaryKey, setPrimaryKey] = useState("");
  const [sortKey, setSortKey] = useState("");

  useEffect(() => {
    describeTable(tableName).then((t) => {
      console.log(t);
    });
  }, []);

  const getItem = () => {
    console.log(primaryKey, sortKey);
  };

  return (
    <div>
      <Title order={3}>Filter</Title>
      <Grid>
        <Grid.Col span={6}>
          <TextInput
            placeholder="Primary key"
            value={primaryKey}
            onChange={(e) => setPrimaryKey(e.currentTarget.value)}
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
          <Button variant="outline" onClick={getItem}>
            Get
          </Button>
        </Grid.Col>
      </Grid>
    </div>
  );
};

export default Filter;
