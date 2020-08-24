export async function getPercentages(tagGroups, projectId) {
  // FIXME:
  projectId = 8;
  tagGroups = tagGroups.groups.map((g, i) => {
    return {
      id: i,
      tags_names: g.tags.map((t) => t.tagName),
    };
  });

  let path = `/projects/${projectId}/percentages`;
  return await fetch(path, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(tagGroups),
  }).then((resp) => resp.json());
}

export async function getSessionsAnalysis(tagGroups, projectId) {
  // FIXME:
  projectId = 8;
  tagGroups = tagGroups.groups.map((g, i) => {
    return {
      id: i,
      tags_names: g.tags.map((t) => t.tagName),
    };
  });

  let path = `/projects/${projectId}/analysis`;
  return await fetch(path, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(tagGroups),
  }).then((resp) => resp.json());
}
