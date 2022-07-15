import { Anchor, Button, Navbar, Title, useMantineTheme } from "@mantine/core";

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
        {/* <ul> */}
        {tables.map((t) => (
          // <li key={t}>
          <Anchor
            href="#"
            key={t}
            onClick={(e: {
              preventDefault: () => void;
              stopPropagation: () => void;
            }) => {
              e.preventDefault();
              e.stopPropagation();
              onTableSelected(t);
            }}
          >
            {t}
          </Anchor>
          // </li>
        ))}
        {/* </ul> */}
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
