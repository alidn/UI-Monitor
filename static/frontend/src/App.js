import React from 'react';
import logo from './logo.svg';
import './App.css';
import Projects from "./components/Projects";
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom';
import Project from "./components/Project";

function App() {
  return (
    <Router>
      <Switch>
        <Route path={"/projects/:name"}>
          <Project/>
        </Route>
        <Route path={"/projects"}>
          <Projects/>
        </Route>
      </Switch>
    </Router>
  );
}

export default App;
