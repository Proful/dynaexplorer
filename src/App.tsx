import { invoke } from "@tauri-apps/api";
import { SetStateAction, useEffect, useState } from "react";

function App() {
  const [tables, setTables] = useState<string[]>([]);
  const [items, setItems] = useState<object[]>([]);

  useEffect(() => {
    invoke("list_tables").then((t) => setTables(t as SetStateAction<string[]>));
  }, []);

  const listItems = function (
    e: React.MouseEvent<HTMLAnchorElement, MouseEvent>,
    tableName: string
  ) {
    e.preventDefault();
    e.stopPropagation();
    invoke("list_items", { tableName: tableName }).then((rows) => {
      let items = [];
      for (const row of rows as string[]) {
        const kv_list = row.split("~~");
        let row_data = {};
        for (const kv of kv_list) {
          const [k, v] = kv.split("::");
          //@ts-ignore
          row_data[k] = v;
        }
        items.push(row_data);
      }
      setItems(items);
    });
  };

  return (
    <div>
      <h1>Tables</h1>
      <ul>
        {tables.map((t) => (
          <li key={t}>
            <a href="#" onClick={(e) => listItems(e, t)}>
              {t}
            </a>
          </li>
        ))}
      </ul>
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
    </div>
  );
}

export default App;
