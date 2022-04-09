import * as React from "react";
import { Link } from "react-router-dom";

const Contact = () => {
  document.title = "Contact";

  return (
    <div>
      <Link to="/home">&lt;Go back home</Link>
      <h1>Contact Page</h1>
    </div>
  );
};

export default Contact;
