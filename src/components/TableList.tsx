import { Anchor, Button, Navbar, Title, useMantineTheme } from "@mantine/core";
import { Database } from "tabler-icons-react";

type TableListProps = {
  tables: string[];
  onTableSelected: (tableName: string) => void;
  onClear: () => void;
};

const TableList = ({ tables, onTableSelected, onClear }: TableListProps) => {
  return (
    <>
      <Navbar.Section>
        <Title order={4}>Tables</Title>
      </Navbar.Section>
      <Navbar.Section grow mt="md">
        {tables.map((t) => (
          <Button
            key={t}
            leftIcon={<Database size={14} />}
            onClick={() => onTableSelected(t)}
          >
            {t}
          </Button>
        ))}
      </Navbar.Section>
      <Navbar.Section>
        <Button variant="outline" onClick={onClear}>
          Clear
        </Button>
        <Button variant="outline" onClick={() => location.reload()}>
          Reload
        </Button>
      </Navbar.Section>
    </>
  );
};

export default TableList;
