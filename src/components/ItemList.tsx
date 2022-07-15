import { Table, Title, useMantineTheme } from "@mantine/core";
import { Item } from "../types";

const ItemList = ({ items }: { items: Item[] }) => {
  const theme = useMantineTheme();
  return (
    <>
      <Title order={1}>Items</Title>
      <Table>
        <tbody>
          {items.map((item) => (
            <tr key={item.sortKey.value}>
              <td>
                <p>{item.primaryKey.name}</p>
                <p style={{ color: theme.colors.blue[2] }}>
                  {item.primaryKey.value}
                </p>
              </td>
              <td>
                <p>{item.sortKey.name}</p>
                <p>{item.sortKey.value}</p>
              </td>

              {item.attributes.map((attr) => (
                <td key={attr.value}>
                  <p>{attr.name}</p>
                  <p>{attr.value}</p>
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </Table>
    </>
  );
};

export default ItemList;
