import { useEffect, useState } from "react";
import { executeCommand } from '../../grpcClient';
import {
  Card,
  CardHeader,
  CardBody,
  Form,
  FormGroup,
  Label,
  Input,
  Button,
  Spinner,
  FormText,
} from "reactstrap";
import ActivityLog from "./ActivityLog";
// import { execute } from "@/lib/backend";

// JSON Prettifier component converted to work with Reactstrap
const JsonPrettifier = ({ output }) => {
  try {
    const prettyJson = typeof output === 'string' ?
      JSON.stringify(JSON.parse(output), null, 2) :
      JSON.stringify(output, null, 2);

    return (
      <pre className="bg-light p-3 rounded">
        <code>{prettyJson}</code>
      </pre>
    );
  } catch {
    return <pre className="bg-light p-3 rounded"><code>{output}</code></pre>;
  }
};

const Execute = () => {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState([])

  const getOutput = (getByProgramId) => {
    const output = localStorage.getItem("output")
    let outputObj = {}
  
    if(output) {
      outputObj = JSON.parse(output)
    }

    if(getByProgramId) {
      const selectedProgramId = localStorage.getItem("selectedProgramId")
      return outputObj[selectedProgramId] || []
    }
  
    return outputObj
  }

  const handleSubmit = async (e) => {
    e.preventDefault();
    const selectedProgramId = localStorage.getItem("selectedProgramId")
    const output = localStorage.getItem("output")
    let outputObj = getOutput()

    try {
      const response = await executeCommand({ input, programId: selectedProgramId });
      if(!outputObj[selectedProgramId]) {
        outputObj[selectedProgramId] = []
      }
      outputObj[selectedProgramId].push(response)
      setOutput(outputObj[selectedProgramId])
      localStorage.setItem('output', JSON.stringify(outputObj))
    } catch (error) {
      // setOutput("An error occurred during execution.");
      console.error(error);
    }
  };
  
  useEffect(() => {
    const output = getOutput(true)
    setOutput(output)
  }, [])

  console.log("output", output)
  return (
      <Card className="shadow execute-card">
        <Form className="execute-form" onSubmit={handleSubmit}>
          <FormGroup>
            <h3>Input</h3>
            <Input
              id="executeName"
              type="textarea"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              required
              style={{ marginTop: '12%'}}
              placeholder="Enter input"
            />
          </FormGroup>

          <Button className="execute-button" color="primary" type="submit" block>
            Execute
          </Button>
        </Form>
        <div>
          <h3 style={{marginLeft: '10px'}}> Activity Log</h3>
          {output && output.map(res => {
            return (
              <div style={{ background: "#f9f2f2", marginBottom: "10px", marginLeft: "8px"}}>{res}</div>
            )
          })}
        </div>
      </Card>
  );
};

export default Execute;
