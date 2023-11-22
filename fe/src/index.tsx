import ReactDOM  from "react-dom";
import { App } from "./App";
import "./assets/tailwind.css"


import { createRoot } from 'react-dom/client';
const container = document.getElementById('root');
const root = createRoot(container!); // createRoot(container!) if you use TypeScript
root.render(<App />);

