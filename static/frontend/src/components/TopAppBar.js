import React from "react";
import {Toolbar} from "@material-ui/core";
import IconButton from "@material-ui/core/IconButton";
import MenuIcon from "@material-ui/icons/Menu";
import Typography from "@material-ui/core/Typography";
import AppBar from "@material-ui/core/AppBar";

export default function TopAppBar({pageName}) {
  return <AppBar position={"static"}>
    <Toolbar>
      <IconButton edge="start" color="inherit" aria-label="menu">
        <MenuIcon/>
      </IconButton>
      <Typography variant={"h6"}>
        {pageName}
      </Typography>
    </Toolbar>
  </AppBar>
}