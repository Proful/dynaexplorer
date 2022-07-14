import { Item } from "../types";

const ItemList = ({ items }: { items: Item[] }) => {
  return (
    <>
      <h1>Items</h1>
      <table>
        <thead>
          <tr>
            <th>PK</th>
            <th>SK</th>
            <th>title</th>
            <th>tags</th>
            <th>data</th>
            <th>notes</th>
            <th>shapes</th>
          </tr>
        </thead>
        {items.map((item, i) => (
          <tr key={i}>
            <td>{item["PK"]}</td>
            <td>{item["SK"]}</td>
            <td>{item["title"]}</td>
            <td>{item["tags"]}</td>
            <td>{item["data"]}</td>
            <td>{item["notes"]}</td>
            <td>{item["shapes"]}</td>
          </tr>
        ))}
      </table>
    </>
  );
};

export default ItemList;
