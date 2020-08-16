const KEY = "tag-groups";

export function saveTagGroup(tagGroup, projectName) {
  let tagGroups = getTagGroups();
  tagGroups.push(JSON.stringify(tagGroup));
  localStorage.setItem(KEY + projectName, JSON.stringify(tagGroups));
}

export function getTagGroups(projectName) {
  return JSON.parse(localStorage.getItem(KEY + projectName)) || [];
}

export function getTagGroupsSize(projectName) {
  let tagGroups = localStorage.getItem(KEY + projectName) || [];
  return tagGroups.length;
}
