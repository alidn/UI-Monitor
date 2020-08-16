import React, { useState } from "react";
import styles from "./Project.module.css";
import { Typography } from "@material-ui/core";
import Select from "@material-ui/core/Select";
import MenuItem from "@material-ui/core/MenuItem";

export function Overview({ avgDuration, sessionsCount }) {
  return (
    <div className={styles.layout}>
      <Typography
        className={styles.title}
        color={"primary"}
        component={"h1"}
        variant={"h4"}
      >
        Overview
      </Typography>
      <Typography variant={"h6"}>Sessions: {sessionsCount}</Typography>
      <SessionDuration avgDuration={avgDuration} />
    </div>
  );
}

function SessionDuration({ avgDuration }) {
  const [unit, setUnit] = useState("s");

  const handleChange = (e) => {
    setUnit(e.target.value);
  };

  const convertDuration = (duration, unit) => {
    switch (unit) {
      case "s":
        return duration;
      case "m":
        return (duration / 60).toFixed(1);
      case "h":
        return (duration / 3600).toFixed(2);
      default:
    }
  };

  return (
    <div className={styles.sessionDuration}>
      <Typography variant={"h6"}>
        Average Session Duration: {convertDuration(avgDuration, unit)}
      </Typography>
      <Select
        value={unit}
        className={styles.sessionUnitSelect}
        labelId="demo-customized-select-label"
        id="demo-customized-select"
        onChange={handleChange}
      >
        <MenuItem value={"s"}>Seconds</MenuItem>
        <MenuItem value={"m"}>Minutes</MenuItem>
        <MenuItem value={"h"}>Hours</MenuItem>
      </Select>
    </div>
  );
}
