import React, { useRef } from "react";
import { doLogin } from "../api/auth";

export default function LoginPage() {
  let emailRef = useRef(null);
  let passRef = useRef(null);

  const handleLogin = () => {
    let email = emailRef.current.value;
    let pass = passRef.current.value;
    doLogin(email, pass).then(result => result ? window.location.pathname = "/projects" : alert("Failed"));
  };

  return (
    <div>
      <input ref={emailRef} placeholder={"email"} />
      <input ref={passRef} placeholder={"password"} />
      <button onClick={handleLogin}>login</button>
    </div>
  );
}
