import React from "react";
import { useParams, useHistory } from "react-router-dom";
import TopAppBar from "./TopAppBar";
import { useQuery } from "react-query";
import {
  fetchAverageSessionDuration,
  fetchSessionsCount,
} from "../api/projects";
import LinearProgress from "@material-ui/core/LinearProgress";
import { Analytics } from "./Analytics";
import { Overview } from "./ProjectOverview";

export function LoadingBar({ isLoading }) {
  return isLoading ? (
    <LinearProgress
      style={{
        width: "100%",
        position: "absolute",
        backgroundColor: "white",
      }}
    />
  ) : null;
}

export default function Project() {
  let history = useHistory();
  let { name, accessKey } = useParams();
  let { isLoading: isDurationLoading, data: avgDuration } = useQuery(name, () =>
    fetchAverageSessionDuration(accessKey)
  );
  let { isLoading: isSessionsLoading, data: sessionsCount } = useQuery(
    name,
    () => fetchSessionsCount(accessKey)
  );

  return (
    <React.Fragment>
      <LoadingBar
        isLoading={[isDurationLoading, isSessionsLoading].includes(true)}
      />
      <TopAppBar pageName={name} />
      <Overview accessKey={accessKey} avgDuration={avgDuration} sessionsCount={sessionsCount} />
      <Analytics accessKey={accessKey} />
    </React.Fragment>
  );
}
