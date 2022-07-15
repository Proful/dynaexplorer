import { Item } from "../types";

const ItemList = ({ items }: { items: Item[] }) => {
  return (
    <>
      <h1>Items</h1>
      <table>
        <tbody>
          {items.map((item) => (
            <tr key={item.sortKey.value}>
              <td>{`${item.primaryKey.name} => ${item.primaryKey.value}`}</td>
              <td>{`${item.sortKey.name} => ${item.sortKey.value}`}</td>

              {item.attributes.map((attr) => (
                <td key={attr.value}>{`${attr.name} => ${attr.value}`}</td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
};

export default ItemList;
