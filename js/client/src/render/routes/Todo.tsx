import * as React from "react";
import { Link } from "react-router-dom";

const Todo = () => {
  document.title = "Todo";

  return (
    <div>
      <Link to="/home">&lt;Go back home</Link>
      <h1>Todo</h1>
    </div>
  );
};

export default Todo;
