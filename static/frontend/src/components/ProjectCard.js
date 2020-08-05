import React from "react";
import Card from "@material-ui/core/Card";
import {CardContent} from "@material-ui/core";
import Typography from "@material-ui/core/Typography";
import CardActions from "@material-ui/core/CardActions";
import Button from "@material-ui/core/Button";
import styles from './ProjectCard.module.css';
import {Link} from "react-router-dom";

export default function ProjectCard({name, sessions}) {
  return <Card style={{
    maxWidth: "30vw",
    // height: "140px",
    margin: "20px"
  }}
               variant={"outlined"}>
    <CardContent>
      <Typography variant={"h5"} color={"primary"}>
        {name}
      </Typography>
      <Typography color={"textSecondary"}>
        Sessions: {sessions}
      </Typography>
    </CardContent>
    <CardActions>
      <Link to={`/projects/${name}`} style={{textDecoration: 'none'}}>
        <Button color={"default"} size={"small"}>Go to project</Button>
      </Link>
    </CardActions>
  </Card>
}