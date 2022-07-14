type TableListProps = {
  tables: string[];
  onTableSelected: (tableName: string) => void;
};

const TableList = ({ tables, onTableSelected }: TableListProps) => {
  return (
    <>
      <h1>Tables</h1>
      <ul>
        {tables.map((t) => (
          <li key={t}>
            <a
              href="#"
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                onTableSelected(t);
              }}
            >
              {t}
            </a>
          </li>
        ))}
      </ul>
    </>
  );
};

export default TableList;
