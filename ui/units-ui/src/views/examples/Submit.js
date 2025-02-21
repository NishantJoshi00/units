import { useState } from 'react'
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
import { submit } from '../../grpcClient';

const Submit = () => {

    const [name, setName] = useState("");
    const [version, setVersion] = useState("");
    const [loading, setLoading] = useState(false);
    const [driverBinary, setDriverBinary] = useState(null);

    const handleSubmit = async (e) => {
        e.preventDefault();
        // setLoading(true);
        console.log("name", name)
        console.log("version", version)
        try {
            const response = await submit({ name, version, binary: driverBinary });
            console.log("response", response)
            window.location.href = '/admin/programs'
            // setOutput(response);
        } catch (error) {
            // setOutput('An error occurred while loading the driver.');
            console.error(error);
        } finally {
            setLoading(false);
        }
    }

    return (
        <Card className="shadow">
            <CardHeader>
                <h3 className="mb-0">
                    Add Program
                </h3>
                <Button className='backButton' onClick={() => window.history.back()}>
                    <i class="fa-solid fa-left-long"></i>
                </Button>
            </CardHeader>
            <CardBody>
                {loading ? (
                    <div className="text-center py-5">
                        <Spinner color="primary" style={{ width: '3rem', height: '3rem' }} />
                    </div>
                ) : (
                    <Form onSubmit={handleSubmit}>
                        <FormGroup>
                            <Label for="name">Name</Label>
                            <Input
                                id="name"
                                value={name}
                                onChange={(e) => setName(e.target.value)}
                                required
                                placeholder="Enter name"
                            />
                        </FormGroup>

                        <FormGroup>
                            <Label for="driverVersion">Version</Label>
                            <Input
                                id="version"
                                value={version}
                                onChange={(e) => setVersion(e.target.value)}
                                required
                                placeholder="Enter version"
                            />
                        </FormGroup>

                        <FormGroup>
                            <Label for="driverBinary">Program Binary</Label>
                            <FileUpload fileName={driverBinary?.name} onUpload={(e) => setDriverBinary(e.target.files?.[0] || null)} fileType={'WASM'} />
                        </FormGroup>


                        <Button color="primary" type="submit" block>
                            Upload
                        </Button>
                    </Form>
                )}
            </CardBody>
        </Card>
    );
}

export default Submit;
