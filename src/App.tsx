import { invoke } from "@tauri-apps/api";
import { SetStateAction, useEffect, useState } from "react";

function App() {
  const [tables, setTables] = useState<string[]>([]);

  useEffect(() => {
    invoke("list_tables").then((t) => setTables(t as SetStateAction<string[]>));
  }, []);

  return (
    <div>
      <h1>Tables</h1>
      <ul>
        {tables.map((t) => (
          <li key={t}>
            <a href="#">{t}</a>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
