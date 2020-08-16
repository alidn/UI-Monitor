import React from "react";
import Projects from "./components/Projects";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import Project from "./components/Project";
import { doLogin } from "./api/auth";

doLogin();

function App() {
  return (
    <Router>
      <Switch>
        <Route path={"/projects/:name/:accessKey"}>
          <Project />
        </Route>
        <Route path={"/projects"}>
          <Projects />
        </Route>
      </Switch>
    </Router>
  );
}

export default App;
