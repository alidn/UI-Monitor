export async function fetchProjects() {
  let response = await fetch("/projects", {
    credentials: "same-origin",
  });
  return await response.json();
}

export async function getProjectIdFromAccessKey(accessKey) {}

export async function fetchAverageSessionDuration(accessKey) {
  let response = await fetch(`/projects/${accessKey}/avg-duration`, {
    credentials: "same-origin",
  });
  return await response.json();
}

export async function fetchSessionsCount(accessKey) {
  let response = await fetch(`/projects/${accessKey}/session-counts`, {
    credentials: "same-origin",
  });
  return await response.json();
}

export async function getProjectTags(accessKey) {
  let response = await fetch(`/projects/${accessKey}/tags`, {
    credentials: "same-origin",
  });
  return await response.json();
}

export async function saveProject(name) {
  let response = await fetch(`/projects/${name}`, {
    method: "POST",
    credentials: "same-origin"
  });
  return response.status === 200;
}