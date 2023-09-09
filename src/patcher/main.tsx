import React from "react";
import ReactDOM from "react-dom/client";
import Patcher from "./patcher";
//import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Patcher />
  </React.StrictMode>
);
