import React from "react";
import ProjectCard from "./ProjectCard";
import styles from "./Projects.module.css";
import TopAppBar from "./TopAppBar";
import { useQuery } from "react-query";
import { fetchProjects } from "../api/projects";
import LinearProgress from "@material-ui/core/LinearProgress";

export default function Projects() {
  const { isLoading, error, data: projects } = useQuery(
    "projects",
    fetchProjects
  );

  if (isLoading) {
    return <LinearProgress />;
  }

  return (
    <div>
      <TopAppBar pageName={"Projects"} />
      <ProjectsGrid projects={projects} />
    </div>
  );
}

function ProjectsGrid({ projects }) {
  return (
    <div className={styles.projectsGrid}>
      {projects.map((p) => (
        <ProjectCard
          key={p.access_key}
          name={p.name}
          sessions={p.sessions}
          accessKey={p.access_key}
        />
      ))}
    </div>
  );
}
