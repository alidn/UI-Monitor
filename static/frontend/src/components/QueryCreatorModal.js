import DialogContent from "@material-ui/core/DialogContent";
import { Typography } from "@material-ui/core";
import Autocomplete from "@material-ui/lab/Autocomplete";
import Button from "@material-ui/core/Button";
import React, { useEffect, useState } from "react";
import Paper from "@material-ui/core/Paper";
import Chip from "@material-ui/core/Chip";
import { useQuery } from "react-query";
import { getProjectTags } from "../api/projects";
import Dialog from "@material-ui/core/Dialog";
import DialogTitle from "@material-ui/core/DialogTitle";
import { LoadingBar } from "./Project";
import TextField from "@material-ui/core/TextField";
import { getTagGroupsSize, saveTagGroup } from "../api/tagGroups";

export function QueryCreatorModal({ open, handleClose, accessKey }) {
  let { data: tagNames } = useQuery(`${accessKey}-tags`, () =>
    getProjectTags(accessKey)
  );
  let [tags, setTags] = useState(
    tagNames
      ? tagNames.map((tag) => {
          return { tagName: tag, selected: false };
        })
      : []
  );
  let [someSelected, setSomeSelected] = useState(false);
  let [groups, setGroups] = useState([]);
  const [value, setValue] = React.useState(groups[0]);
  const [queryName, setQueryName] = useState(
    "query #" + getTagGroupsSize() + 1
  );
  let { name: projectName } = useParams();

  useEffect(() => {
    setTags(
      tagNames
        ? tagNames.map((tag) => {
            return { tagName: tag, selected: false };
          })
        : []
    );
  }, [tagNames]);

  useEffect(() => {
    setSomeSelected(() => tags.some((tag) => tag.selected));
  }, [tags]);

  const selectTag = (index) => {
    setTags((prevTags) => {
      let newTags = [...prevTags];
      newTags[index].selected = !newTags[index].selected;
      return newTags;
    });
  };

  const moveTags = () => {
    let selectedTags = tags.filter((tag) => tag.selected);
    setTags((prevState) => prevState.filter((t) => !t.selected));
    let group = {
      name: value,
      tags: selectedTags,
    };

    if (groups.find((g) => g.name === value)) {
      setGroups((prev) => {
        let newGroup = [...prev];
        for (let i = 0; i < newGroup.length; i++) {
          if (newGroup[i].name === value) {
            newGroup[i].tags = newGroup[i].tags.concat(selectedTags);
            return newGroup;
          }
        }
      });
    } else {
      setGroups((prev) => prev.concat(group));
    }
  };

  const removeTagFromGroup = (groupIndex, tagIndex, tagName) => {
    setGroups((prev) => {
      let newGroups = [...prev];
      setTags((prev) => prev.concat({ selected: false, tagName: tagName }));
      newGroups[groupIndex].tags.splice(tagIndex, tagIndex + 1);
      return newGroups;
    });
  };

  const saveTagGroupAndClose = () => {
    saveTagGroup({
      name: queryName,
      groups,
    });
    handleClose();
  };

  const handleQueryNameChange = (event) => {
    setQueryName(event.target.value);
  };

  return (
    <Dialog maxWidth={"md"} fullWidth={true} open={open}>
      <DialogTitle id="alert-dialog-title">{"Create new query"}</DialogTitle>
      <TextField
        style={{ width: "70%", margin: "1rem" }}
        label={"query #" + getTagGroupsSize() + 1}
        variant="outlined"
        onChange={handleQueryNameChange}
      />
      <ModalContent
        tags={tags}
        mapTagToChip={({ selected, tagName }, index) => (
          <Chip
            color={selected ? "primary" : "default"}
            onClick={() => selectTag(index)}
            style={{ margin: "0.5rem" }}
            label={tagName}
          />
        )}
        onInputChange={(e, newValue) => setValue(newValue)}
        renderInput={(params) => (
          <TextField {...params} label="group name" variant="outlined" />
        )}
        options={groups.map((g) => g.name)}
        onClick={moveTags}
        someSelected={someSelected}
        value={value}
      />
      <DialogContent>
        <Typography variant={"h6"}>Tag Groups</Typography>
        <div>
          {groups.map((g, groupIndex) => (
            <Group
              {...g}
              handleDelete={(tagIndex, tagName) =>
                removeTagFromGroup(groupIndex, tagIndex, tagName)
              }
            />
          ))}
        </div>
      </DialogContent>
      <DialogContent dividers>
        <Button
          onClick={saveTagGroupAndClose}
          color={"primary"}
          style={{ marginLeft: "0.5rem" }}
        >
          Save
        </Button>
        <Button
          onClick={handleClose}
          color={"secondary"}
          style={{ marginLeft: "0.5rem" }}
        >
          Cancel
        </Button>
      </DialogContent>
    </Dialog>
  );
}

function ModalContent(props) {
  return (
    <DialogContent dividers>
      <Typography variant={"h6"}>Tags </Typography>
      <div style={{ marginTop: "0.5rem" }}>
        {props.tags.map(props.mapTagToChip)}
      </div>
      <div style={{ marginTop: "1rem" }}>
        <Autocomplete
          onInputChange={props.onInputChange}
          freeSolo
          style={{ maxWidth: "80%", marginBottom: "0.5rem" }}
          renderInput={props.renderInput}
          options={props.options}
        />
        <Button
          onClick={props.onClick}
          disabled={!props.someSelected || !props.value || props.value === ""}
          color={"primary"}
          variant={"text"}
        >
          Move selected tags to a group
        </Button>
      </div>
    </DialogContent>
  );
}

function Group({ name, tags, handleDelete }) {
  return (
    <Paper variant={"outlined"} style={{ margin: "0.5rem" }}>
      <Typography style={{ margin: "0.5rem" }} variant={"h6"}>
        {name}
      </Typography>
      {tags.map((t, index) => (
        <Chip
          onDelete={() => handleDelete(index, t.tagName)}
          style={{ margin: "0.5rem" }}
          label={t.tagName}
        />
      ))}
    </Paper>
  );
}
