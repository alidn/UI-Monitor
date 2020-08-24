export function doLogin(email, pass) {
  let myHeaders = new Headers();
  myHeaders.append("Content-Type", "application/json");
  myHeaders.append(
    "Cookie",
    "user_auth=UDinXdmKQFGj+YaYLLkcEL+N0kHOh80QV3HeYXI="
  );

  let raw = JSON.stringify({ email: email, password: pass });

  let requestOptions = {
    method: "POST",
    // headers: myHeaders,
    headers: {
      'Content-Type': 'application/json',
    },
    body: raw,
    redirect: "follow",
  };

  return fetch("/login", requestOptions).then((resp) => resp.status === 200).catch((error) => false);
}
