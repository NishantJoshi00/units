import { useState } from 'react';
import { loadDriver } from '../../grpcClient';
import FileUpload from './FileUpload';
import {
  Card,
  CardHeader,
  CardBody,
  Form,
  FormGroup,
  Label,
  Input,
  Button,
  Spinner
} from 'reactstrap';

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

const Upload = ({ onUploadSuccess }) => {
  const [driverName, setDriverName] = useState('');
  const [driverVersion, setDriverVersion] = useState('');
  const [driverType, setDriverType] = useState('WASM');
  const [driverBinary, setDriverBinary] = useState(null);
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState(null);
  const [handlerType, setHandlerType] = useState('')
  const [storageType, setStorageType] = useState('')

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);

    try {
      const response = await loadDriver({ driverName, driverVersion, driverBinary });
      setOutput(response);
      // onUploadSuccess()
      window.location.href = '/admin/supported-token-drivers'
    } catch (error) {
      setOutput('An error occurred while loading the driver.');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const resetForm = () => {
    setOutput(null);
    setDriverName('');
    setDriverVersion('');
    setDriverType('WASM');
    setDriverBinary(null);
  };

  console.log("handlerType", handlerType)
  console.log("storageType", storageType)

  return (
    <Card className="shadow upload-container">
      <CardHeader>
        <h3 className="mb-0">
          Upload
          <Button className='backButton' onClick={() => window.history.back()}>
            <i class="fa-solid fa-left-long"></i>
          </Button>
        </h3>
      </CardHeader>
      <CardBody>
        {loading ? (
          <div className="text-center py-5">
            <Spinner color="primary" style={{ width: '3rem', height: '3rem' }} />
          </div>
        ) : output ? (
          <div className="space-y-4">
            <JsonPrettifier output={output} />
            <Button color="primary" onClick={resetForm}>
              Onboard another token driver
            </Button>
          </div>
        ) : (
          <Form onSubmit={handleSubmit}>
            <FormGroup>
              <Label for="driverName">Token Driver Name</Label>
              <Input
                id="driverName"
                value={driverName}
                onChange={(e) => setDriverName(e.target.value)}
                required
                placeholder="Enter token handler name"
              />
            </FormGroup>

            {/* <FormGroup>
              <Label htmlFor="handlerType">Token Handler Type</Label>

              <Input
                type="select"
                id="handlerType"
                value={handlerType}
                onChange={(e) => setHandlerType(e.target.value)}
                required
              >
                <option value="" disabled>
                  Please Select Token Handler Type
                </option>
                <option value="Native">Native</option>
                <option value="Custodial">Custodial</option>
                <option value="Proxy">Proxy</option>
              </Input> */}
            {/* </FormGroup> */}

            {handlerType === "Native" && (
              <FormGroup>
                <Label htmlFor="storageType">Storage Type</Label>

                <Input
                  type="select"
                  id="storageType"
                  value={storageType}
                  onChange={(e) => setStorageType(e.target.value)}
                  required
                >
                  <option value="" disabled>
                    Please Select Storage Type
                  </option>
                  <option value="Redis">Redis</option>
                  <option value="Solana">Solana</option>
                </Input>
              </FormGroup>
            )}

            <FormGroup>
              <Label for="driverVersion">Token Driver Version</Label>
              <Input
                id="driverVersion"
                value={driverVersion}
                onChange={(e) => setDriverVersion(e.target.value)}
                required
                placeholder="Enter version"
              />
            </FormGroup>

            <FormGroup>
              <Label for="driverBinary">Token Driver Binary</Label>
              <FileUpload fileName={driverBinary?.name} onUpload={(e) => setDriverBinary(e.target.files?.[0] || null)} fileType={driverType} />
            </FormGroup>

            <Button color="primary" type="submit" block style={{ marginTop: '3rem' }}>
              Setup Token Driver
            </Button>
          </Form>
        )}
      </CardBody>
    </Card>
  );
};

export default Upload;
