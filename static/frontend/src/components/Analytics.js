import React, { useEffect, useState } from "react";
import styles from "./Project.module.css";
import { Typography, jssPreset } from "@material-ui/core";
import Button from "@material-ui/core/Button";
import AddIcon from "@material-ui/icons/Add";
import Tabs from "@material-ui/core/Tabs";
import Tab from "@material-ui/core/Tab";
import { QueryCreatorModal } from "./QueryCreatorModal";
import { getTagGroups } from "../api/tagGroups";
import CardContent from "@material-ui/core/CardContent";
import Card from "@material-ui/core/Card";
import CardActions from "@material-ui/core/CardActions";
import { getPercentages, getSessionsAnalysis } from "../api/queries";
import { LoadingBar } from "./Project";
import Dialog from "@material-ui/core/Dialog";
import DialogTitle from "@material-ui/core/DialogTitle";
import DialogContent from "@material-ui/core/DialogContent";
import Backdrop from "@material-ui/core/Backdrop";
import CircularProgress from "@material-ui/core/CircularProgress";
import Divider from "@material-ui/core/Divider";

export function Analytics({ accessKey }) {
  const [modalOpen, setModalOpen] = useState(false);
  let [tab, setTab] = useState(0);
  let [percentagesResult, setPercentagesResult] = useState(null);
  let [analysisResult, setanalysisResult] = useState(null);
  let [query, setQuery] = useState(null);
  let [isLoading, setLoading] = useState(false);

  const handleModalOpen = () => {
    setModalOpen(true);
  };

  const handleModalClose = () => {
    setModalOpen(false);
  };

  const changeTabTo = (newTab) => setTab(newTab);

  const handleRun = async (qry) => {
    setQuery(qry);
    setLoading(true);
    if (tab === 0) {
      let percentages = await getPercentages(qry);
      setPercentagesResult(percentages);
    } else {
      let analysisResult = await getSessionsAnalysis(qry);
      setanalysisResult(analysisResult);
    }
    setLoading(false);
  };

  return (
    <div className={styles.layout}>
      <Backdrop style={{ zIndex: "10" }} open={isLoading}>
        <CircularProgress color="inherit" />
      </Backdrop>
      {/*<div style={{ padding: "1rem 0" }}>*/}
      {/*  <LoadingBar isLoading={isLoading} />*/}
      {/*</div>*/}
      <Typography
        className={styles.title}
        color={"primary"}
        component={"h1"}
        variant={"h4"}
      >
        Analytics{" "}
        <Button
          onClick={handleModalOpen}
          startIcon={<AddIcon />}
          variant={"outlined"}
          size={"medium"}
          color={"primary"}
        >
          New Query
        </Button>
      </Typography>
      <QueryCreatorModal
        accessKey={accessKey}
        open={modalOpen}
        onClose={handleModalClose}
        handleClose={handleModalClose}
      />
      <Queries handleRun={handleRun} />
      <QueryResult
        query={query}
        percentagesResult={percentagesResult}
        analysisResult={analysisResult}
        setTab={changeTabTo}
      />
    </div>
  );
}

function Queries({ handleRun }) {
  let queries = getTagGroups().map((q) => JSON.parse(q));
  let [selectedQuery, setSelectedQuery] = useState(0);

  const handleSelected = (id) => {
    setSelectedQuery(id);
  };

  return (
    <div style={{ marginBottom: "2rem" }}>
      <Typography variant={"h5"}>
        Queries{" "}
        <Button
          style={{ marginLeft: "1rem" }}
          color={"primary"}
          variant={"outlined"}
          disabled={!queries}
          onClick={() => handleRun(queries[selectedQuery])}
        >
          Run
        </Button>
      </Typography>
      {queries.length === 0 ? (
        <Typography variant={"body1"}>
          No query found, create a new one
        </Typography>
      ) : (
        <div style={{ display: "flex", flexDirection: "row" }}>
          {queries.map((query, index) => (
            <Query
              key={index}
              handleSelected={handleSelected}
              isSelected={index === selectedQuery}
              id={index}
              {...query}
            />
          ))}
        </div>
      )}
    </div>
  );
}

function Query({ id, name, groups, isSelected, handleSelected }) {
  let [isModalOpen, setModalOpen] = useState(false);

  return (
    <Card
      onClick={() => handleSelected(id)}
      variant={"outlined"}
      style={{
        cursor: "pointer",
        width: "200px",
        margin: "1rem",
        // zIndex: "-1",
        backgroundColor: isSelected ? "#f0f6ff" : "",
      }}
    >
      <CardContent>
        <Typography variant={"h6"}>{name}</Typography>
      </CardContent>
      <CardActions>
        <Button
          size="small"
          color={"primary"}
          onClick={() => setModalOpen(true)}
        >
          See Groups
        </Button>
      </CardActions>
      <Dialog
        fullWidth={true}
        // maxWidth={"lg"}
        onClose={() => setModalOpen(false)}
        open={isModalOpen}
      >
        <DialogTitle>{name}</DialogTitle>
        <DialogContent>
          {groups.map((g, i) => (
            <div key={i}>
              <Typography color={"primary"} variant={"h6"}>
                {g.name}
              </Typography>
              {g.tags.map((t) => (
                <Typography variant={"body1"}>{t.tagName}</Typography>
              ))}
              <Divider style={{ margin: "1rem" }} />
            </div>
          ))}
        </DialogContent>
      </Dialog>
    </Card>
  );
}

function QueryResult({ setTab, percentagesResult, analysisResult, query }) {
  const [value, setValue] = React.useState(0);

  const handleChange = (_event, newValue) => {
    setValue(newValue);
    setTab(newValue);
  };

  return (
    <div style={{ marginTop: "2rem" }}>
      <Typography style={{ marginBottom: "1rem" }} variant={"h5"}>
        Query Result
      </Typography>
      <Tabs indicatorColor={"primary"} value={value} onChange={handleChange}>
        <Tab label="Percentages" />
        <Tab label="Session analytics" />
      </Tabs>
      {value === 0 ? (
        <PercentagesTab result={percentagesResult} query={query} />
      ) : (
        <SessionAnalyticsTab result={analysisResult} query={query} />
      )}
    </div>
  );
}

function PercentagesTab({ result, query }) {
  if (!query || !result) {
    return (
      <Typography color={"secondary"}>
        You have to run the query first
      </Typography>
    );
  }

  console.log(result);
  return (
    <div>
      <Typography style={{ margin: "1rem" }} variant={"h5"}>
        Query {query.name}
      </Typography>
      {query.groups.map((g, i) => {
        return (
          <div>
            <div key={g.name}>
              <Typography color={"primary"}>
                {g.name}:{result[i]}%
              </Typography>
              <Typography>
                {g.tags.map((tag) => (
                  <span key={tag.tagName}>
                    {" | "} {tag.tagName}
                  </span>
                ))}
              </Typography>
            </div>
            <Divider style={{ margin: "0.5rem 0" }} />
          </div>
        );
      })}
    </div>
  );
}

function SessionAnalyticsTab({ query, result }) {
  if (!query || !result) {
    return (
      <Typography color={"secondary"}>
        You have to run the query first
      </Typography>
    );
  }

  console.log(result);
  // return <div>{JSON.stringify(result)}</div>;
  return (
    <ol>
      {result.map((step) => (
        <StepAnalysis key={step.step_number} step={step} />
      ))}
    </ol>
  );
}

function StepAnalysis({ step }) {
  return (
    <li>
      <Typography>Average Duration: {step.average_duration / 1000}</Typography>
      <div>
        <b>tags sorted: </b>
        {step.tag_groups_sorted.map((t) =>
          t.tags_names.map((tagName) => <span>{tagName}, </span>)
        )}
      </div>
      <Divider style={{ margin: "1rem" }} />
    </li>
  );
}
