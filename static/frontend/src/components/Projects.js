import React, { useState } from "react";
import ProjectCard from "./ProjectCard";
import styles from "./Projects.module.css";
import TopAppBar from "./TopAppBar";
import { useQuery } from "react-query";
import { fetchProjects, saveProject } from "../api/projects";
import LinearProgress from "@material-ui/core/LinearProgress";
import { Button, Dialog, DialogContent, DialogActions } from "@material-ui/core";
import DialogTitle from "@material-ui/core/DialogTitle";
import TextField from '@material-ui/core/TextField';

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
      <NewProjectButton />
      <ProjectsGrid projects={projects} />
    </div>
  );
}

function NewProjectButton() {
  let [isModalOpen, setModalOpen] = useState(false);
  let [name, setName] = useState("");
  let [isSaving, setSaving] = useState(false);

  const handleSave = () => {
    if (name === "") {
      return alert("Name cannot be empty");
    }
    setSaving(true);
    saveProject(name).then(result => result ? window.location.reload() : alert("failed"));
  };

  const handleNameChange = (event) => {
    setName(event.target.value);
  };

  return (
    <div>
      <Button onClick={() => setModalOpen(true)}>New Project</Button>
      <Dialog onClose={() => setModalOpen(false)} open={isModalOpen}>
        {isSaving && <LinearProgress />}
        <DialogTitle>Create New Project</DialogTitle>
        <DialogContent>
          <TextField onChange={handleNameChange} id="outlined-basic" label="name" variant="outlined" />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setModalOpen(false)}>Cancel</Button>
          <Button onClick={() => handleSave()}>Save</Button>
        </DialogActions>
      </Dialog>
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
