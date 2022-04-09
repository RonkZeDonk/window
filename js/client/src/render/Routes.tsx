import React from "react";
import {
  Route,
  HashRouter,
  Routes as Switch,
  Navigate,
} from "react-router-dom";

import * as Pages from "./routes/";

const Routes = () => {
  return (
    <>
      <HashRouter>
        <Switch>
          <Route path="/" element={<Navigate replace to="/home" />} /> {/* Redirect "/" to "/home" */}
          <Route path="/home" element={<Pages.Home />} />
          <Route path="/about" element={<Pages.About />} />
          <Route path="/contact" element={<Pages.Contact />} />
          <Route path="/todo" element={<Pages.Todo />} />
        </Switch>
      </HashRouter>
    </>
  );
};

export default Routes;
