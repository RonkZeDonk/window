import React from "react";
import { Link } from "react-router-dom";

const NavBar = () => {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-evenly",
        alignItems: "center",
        backgroundColor: "#a1a1a1",
        borderRadius: "8px",
        height: "32px",
      }}
    >
      <Link to="/about" style={{ color: "inherit", textDecoration: "none" }}>
        About
      </Link>
      <Link to="/contact" style={{ color: "inherit", textDecoration: "none" }}>
        Contact
      </Link>
      <Link to="/todo" style={{ color: "inherit", textDecoration: "none" }}>
        Todo
      </Link>
    </div>
  );
};

export default NavBar;
