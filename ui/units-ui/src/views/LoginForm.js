import React, { useState } from "react";
import {
  Form,
  FormGroup,
  Label,
  Input,
  Button,
  Card,
  CardBody,
  Alert,
} from "reactstrap";
import { login } from '../grpcClient'
// import "bootstrap/dist/css/bootstrap.min.css";
// import "bootstrap/dist/css/bootstrap.min.css";

const LoginForm = ({setIsLoggedIn,setTheUser,handleCheck}) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false)
  const [error, setError] = useState(null);

  const handleLogin = async (e) => {
    e.preventDefault();
    if (!username || !password) {
      setError("Please fill in both fields.");
      return;
    }
    setError(null);
    const res = await login({username, password})
    if(res.message=="Error retrieving user"){
      setError("Invalid username or password");
    }
    else{
      console.log(res.message)
      localStorage.setItem("jwtToken",JSON.stringify(res.message)) 
      // localStorage.setItem("theUser",JSON.stringify(username)) 
      setTheUser(username);
      setIsLoggedIn(true);
    }
    
  };

  const toggleShowPassword = () => {
    setShowPassword(preVal => !preVal)
  }

  return (
    <div className="d-flex justify-content-center align-items-center bg-light" style={{ minHeight: "100vh", marginTop: "-100px" }}>
      <Card className="p-4 shadow-lg" style={{ width: "400px" }}>
        <CardBody className="login-body">
          <h3 className="text-center mb-4">Login</h3>
          {error && <Alert color="danger">{error}</Alert>}
          <Form onSubmit={handleLogin}>
            <FormGroup>
              <Label for="username">User Name</Label>
              <Input
                type="text"
                id="username"
                placeholder="Enter your username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
              />
            </FormGroup>
            <FormGroup>
              <Label for="password">Password</Label>
              <Input
                type={showPassword ? "text" : "password"}
                id="password"
                placeholder="Enter your password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
              <i id="passwordEye" class={`fa-regular ${showPassword ? 'fa-eye' : 'fa-eye-slash'}`} onClick={toggleShowPassword}></i>
            </FormGroup>
            <Button className="loginCTA" color="primary" block>
              Login
            </Button>
          </Form>
        </CardBody>
      </Card>
    </div>
  );
};

export default LoginForm;
