import React, {useEffect, useState} from "react";
import {useParams} from 'react-router-dom';
import TopAppBar from "./TopAppBar";
import {Typography} from "@material-ui/core";
import styles from './Project.module.css';
import Select from "@material-ui/core/Select";
import MenuItem from "@material-ui/core/MenuItem";
import Button from "@material-ui/core/Button";
import AddIcon from '@material-ui/icons/Add';
import Dialog from "@material-ui/core/Dialog";
import DialogTitle from "@material-ui/core/DialogTitle";
import DialogContent from "@material-ui/core/DialogContent";
import Paper from "@material-ui/core/Paper";
import Chip from "@material-ui/core/Chip";
import Autocomplete from '@material-ui/lab/Autocomplete';
import TextField from "@material-ui/core/TextField";
import * as PropTypes from "prop-types";

const projectInfo = {
  sessions: 79132,
  averageSessionDuration: 152
}

export default function Project() {
  const {name} = useParams();
  return <React.Fragment>
    <TopAppBar pageName={name}/>
    <Overview/>
    <Analytics/>
  </React.Fragment>
}

function Analytics() {
  const [modalOpen, setModalOpen] = useState(false);

  const handleModalOpen = () => {
    setModalOpen(true);
  };

  const handleModalClose = () => {
    setModalOpen(false);
  };

  return (
    <div className={styles.layout}>
      <Typography className={styles.title} color={"primary"} component={'h1'} variant={'h4'}>Analytics{' '}
        <Button
          onClick={handleModalOpen} startIcon={<AddIcon/>} variant={"outlined"} size={"medium"} color={"primary"}>New
          Query</Button>
      </Typography>
      <QueryCreatorModal open={modalOpen} onClose={handleModalClose} handleClose={handleModalClose}/>
    </div>
  );
}

const tagNames = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla auctor neque tincidunt enim posuere ultrices quis sit amet nisl. Morbi vitae posuere purus. Pellentesque sodales est sit'.split(' ');

function ModalContent(props) {
  return (
    <DialogContent dividers>
      <Typography variant={"h6"}>
        Tags{" "}
      </Typography>
      <div style={{marginTop: "0.5rem"}}>
        {props.tags.map(props.mapTagToChip)}
      </div>
      <div style={{marginTop: "1rem"}}>
        <Autocomplete
          onInputChange={props.onInputChange}
          freeSolo
          style={{maxWidth: "80%", marginBottom: "0.5rem"}}
          renderInput={props.renderInput}
          options={props.options}/>
        <Button onClick={props.onClick} disabled={!props.someSelected || !props.value || props.value === ""}
                color={"primary"}
                variant={"text"}>Move selected tags to a
          group</Button>
      </div>
    </DialogContent>
  );
}

ModalContent.propTypes = {
  tags: PropTypes.arrayOf(PropTypes.shape({tagName: PropTypes.string, selected: PropTypes.bool})),
  mapTagToChip: PropTypes.func,
  onInputChange: PropTypes.func,
  renderInput: PropTypes.func,
  options: PropTypes.arrayOf(PropTypes.string),
  onClick: PropTypes.func,
  someSelected: PropTypes.bool,
  value: PropTypes.any
};

function QueryCreatorModal({open, handleClose}) {
  let [tags, setTags] = useState(tagNames.map(tag => {
    return {tagName: tag, selected: false}
  }));
  let [someSelected, setSomeSelected] = useState(false);
  let [groups, setGroups] = useState([]);
  const [value, setValue] = React.useState(groups[0]);

  useEffect(() => {
    setSomeSelected(() => tags.some(tag => tag.selected));
  }, [tags]);

  const selectTag = (index) => {
    setTags(prevTags => {
      let newTags = [...prevTags];
      newTags[index].selected = !newTags[index].selected;
      return newTags;
    })
  };

  const moveTags = () => {
    let selectedTags = tags.filter(tag => tag.selected);
    setTags(prevState => prevState.filter(t => !t.selected))
    let group = {
      name: value,
      tags: selectedTags
    };

    if (groups.find(g => g.name === value)) {
      setGroups(prev => {
        let newGroup = [...prev];
        for (let i = 0; i < newGroup.length; i++) {
          if (newGroup[i].name === value) {
            newGroup[i].tags = newGroup[i].tags.concat(selectedTags);
            return newGroup;
          }
        }
      })
    } else {
      setGroups(prev => prev.concat(group));
    }
  }

  const removeTagFromGroup = (groupIndex, tagIndex, tagName) => {
    setGroups(prev => {
      let newGroups = [...prev];
      setTags(prev => prev.concat({selected: false, tagName: tagName}));
      newGroups[groupIndex].tags.splice(tagIndex, tagIndex + 1);
      return newGroups;
    })
  };

  return (
    <Dialog maxWidth={"md"} fullWidth={true} open={open}>
      <DialogTitle id="alert-dialog-title">{'Create new query'}</DialogTitle>
      <ModalContent tags={tags} mapTagToChip={({selected, tagName}, index) =>
        <Chip color={selected ? "primary" : "default"}
              onClick={() => selectTag(index)} style={{margin: '0.5rem'}}
              label={tagName}/>} onInputChange={(e, newValue) => setValue(newValue)}
                    renderInput={(params) => <TextField {...params} label="group name" variant="outlined"/>}
                    options={groups.map(g => g.name)} onClick={moveTags} someSelected={someSelected} value={value}/>
      <DialogContent>
        <Typography variant={"h6"}>Tag Groups</Typography>
        <div>
          {groups.map((g, groupIndex) => <Group {...g}
                                                handleDelete={(tagIndex, tagName) => removeTagFromGroup(groupIndex, tagIndex, tagName)}/>)}
        </div>
      </DialogContent>
      <DialogContent dividers>
        <Button onClick={handleClose} variant={"outlined"} color={"primary"}>Ok</Button>
        <Button onClick={handleClose} color={"primary"} style={{marginLeft: "0.5rem"}}>Cancel</Button>
      </DialogContent>
    </Dialog>
  );
}

function Group({name, tags, handleDelete}) {
  return (
    <Paper variant={"outlined"} style={{margin: '0.5rem'}}>
      <Typography style={{margin: '0.5rem'}} variant={"h6"}>{name}</Typography>
      {tags.map((t, index) => <Chip onDelete={() => handleDelete(index, t.tagName)} style={{margin: '0.5rem'}}
                                    label={t.tagName}/>)}
    </Paper>
  );
}

function Overview() {
  return (
    <div className={styles.layout}>
      <Typography className={styles.title} color={"primary"} component={'h1'} variant={'h4'}>Overview</Typography>
      <Typography variant={"h6"}>Sessions: {projectInfo.sessions}</Typography>
      <SessionDuration/>
    </div>
  );
}


function SessionDuration() {
  const [unit, setUnit] = useState('s');

  const handleChange = (e) => {
    setUnit(e.target.value);
  };

  const convertDuration = (duration, unit) => {
    switch (unit) {
      case 's':
        return duration;
      case 'm':
        return (duration / 60).toFixed(1);
      case 'h':
        return (duration / 3600).toFixed(2);
      default:
    }
  };

  return (
    <div className={styles.sessionDuration}>
      <Typography variant={"h6"}>Average Session
        Duration: {convertDuration(projectInfo.averageSessionDuration, unit)}</Typography>
      <Select
        value={unit}
        className={styles.sessionUnitSelect}
        labelId="demo-customized-select-label"
        id="demo-customized-select"
        onChange={handleChange}
      >
        <MenuItem value={'s'}>Seconds</MenuItem>
        <MenuItem value={'m'}>Minutes</MenuItem>
        <MenuItem value={'h'}>Hours</MenuItem>
      </Select>
    </div>
  );
}