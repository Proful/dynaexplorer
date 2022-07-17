import { Popover, Table, Title, useMantineTheme } from "@mantine/core";
import { useState } from "react";
import { Item } from "../types";
import JSONPretty from "react-json-pretty";
import "react-json-pretty/themes/monikai.css";

const ItemList = ({ items }: { items: Item[] }) => {
  const theme = useMantineTheme();
  const [opened, setOpened] = useState({});

  const togglePopover = (key: string) => {
    //@ts-ignore
    setOpened({ ...opened, [key]: !opened[key] });
  };

  return (
    <>
      <Title order={3}>Items</Title>
      <Table>
        <tbody>
          {items.map((item, i) => (
            <tr key={item.sortKey.value}>
              <td>
                <p>{item.partionKey.name}</p>
                <p style={{ color: theme.colors.blue[2] }}>
                  {item.partionKey.value}
                </p>
              </td>
              <td>
                <p>{item.sortKey.name}</p>
                <p>{item.sortKey.value}</p>
              </td>

              {item.attributes.map((attr, j) => (
                <td key={attr.value}>
                  <p>{attr.name}</p>

                  {attr.value.length <= 20 && <p>{attr.value}</p>}

                  {attr.value.length > 20 && (
                    <Popover
                      //@ts-ignore
                      opened={!!opened[i + "~" + j]}
                      onClose={() => setOpened({})}
                      target={
                        <p onClick={() => togglePopover(i + "~" + j)}>
                          {attr.value.substring(0, 20)}...
                        </p>
                      }
                      width={360}
                      position="bottom"
                      withArrow
                    >
                      <JSONPretty
                        data={attr.value}
                        style={{ background: "transparent " }}
                      />
                    </Popover>
                  )}
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
