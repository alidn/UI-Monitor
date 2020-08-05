import React from "react";
import AppBar from "@material-ui/core/AppBar";
import {Toolbar} from "@material-ui/core";
import IconButton from "@material-ui/core/IconButton";
import MenuIcon from '@material-ui/icons/Menu';
import Typography from "@material-ui/core/Typography";
import ProjectCard from "./ProjectCard";
import styles from './Projects.module.css';
import TopAppBar from "./TopAppBar";

const projectsData = [
  {
    name: 'Actix',
    sessions: 142331
  },
  {
    name: 'The Alchemist IDE',
    sessions: 94211231,
  },
  {
    name: 'Phoenix',
    sessions: 5125234
  },
  {
    name: 'React View',
    sessions: 10312
  }
]
export default function Projects() {
  return (
    <div>
      <TopAppBar pageName={'Projects'}/>
      <ProjectsGrid/>
    </div>
  );
}

function ProjectsGrid() {
  return (
    <div className={styles.projectsGrid}>
      {projectsData.map(p => <ProjectCard name={p.name} sessions={p.sessions}/>)}
    </div>
  );
}