import * as React from "react";
import { Link } from "react-router-dom";

const About = () => {
  document.title = "About";

  return (
    <div>
      <Link to="/home">&lt;Go back home</Link>
      <h1>About Page</h1>
    </div>
  );
};

export default About;
