import React from "react";
import Card from "@material-ui/core/Card";
import { CardContent } from "@material-ui/core";
import Typography from "@material-ui/core/Typography";
import CardActions from "@material-ui/core/CardActions";
import Button from "@material-ui/core/Button";
import styles from "./ProjectCard.module.css";
import { Link } from "react-router-dom";
import { useQuery } from "react-query";
import { fetchAverageSessionDuration } from "../api/projects";
import Project from "./Project";

export default function ProjectCard({ name, sessions, accessKey }) {
  return (
    <Card
      style={{
        maxWidth: "30vw",
        margin: "20px",
      }}
      variant={"outlined"}
    >
      <CardContent>
        <Typography variant={"h5"} color={"primary"}>
          {name}
        </Typography>
        <Typography color={"textSecondary"}>Sessions: {sessions}</Typography>
      </CardContent>
      <CardActions>
        <Link
          to={{ pathname: `/projects/${name}/${accessKey}`, state: {accessKey, name} }}
          style={{ textDecoration: "none" }}
        >
          <Button color={"default"} size={"small"}>
            Go to project
          </Button>
        </Link>
      </CardActions>
    </Card>
  );
}
