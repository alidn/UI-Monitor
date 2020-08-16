export function doLogin() {
  let myHeaders = new Headers();
  myHeaders.append("Content-Type", "application/json");
  myHeaders.append(
    "Cookie",
    "user_auth=UDinXdmKQFGj+YaYLLkcEL+N0kHOh80QV3HeYXI="
  );

  let raw = JSON.stringify({ email: "zas@gmail.com", password: "asdf" });

  let requestOptions = {
    method: "POST",
    headers: myHeaders,
    body: raw,
    redirect: "follow",
  };

  fetch("/login", requestOptions).catch((error) => console.log("error", error));
}
