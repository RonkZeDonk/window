/**
 * This file will automatically be loaded by webpack and run in the "renderer" context.
 * To learn more about the differences between the "main" and the "renderer" context in
 * Electron, visit:
 *
 * https://electronjs.org/docs/tutorial/application-architecture#main-and-renderer-processes
 */

import "./render/index.css";
import ReactDOM from "react-dom";
import App from "./render/App";

console.log("👋 This message is being logged by \"renderer.ts\", included via webpack");

function render() {
  ReactDOM.render(App(), document.getElementById("root"));
}

render();
