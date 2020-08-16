export async function getPercentages(tagGroups, projectId) {
  projectId = 8;
  console.log(tagGroups);
  tagGroups = tagGroups.groups.map((g, i) => {
    return {
      id: i,
      tags_names: g.tags.map((t) => t.tagName),
    };
  });

  console.log(tagGroups);

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
