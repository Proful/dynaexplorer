import React from "react";
import ReactDOM from "react-dom/client";
import { MantineProvider } from "@mantine/core";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <MantineProvider theme={{ colorScheme: "dark" }}>
      <App />
    </MantineProvider>
  </React.StrictMode>
);
