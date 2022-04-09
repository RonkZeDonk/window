import * as React from "react";
import Footer from "../components/Footer";
import NavBar from "../components/NavBar";

const Home = () => {
  document.title = "Home";

  return (
    <>
      <NavBar />
      <div>
        <h1>💖 Hello World!</h1>
        <p>Welcome to your Electron application.</p>
        <hr />
        <h2>Hello from React!</h2>
      </div>
      <Footer />
    </>
  );
};

export default Home;
