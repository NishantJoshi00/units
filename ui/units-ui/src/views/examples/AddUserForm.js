import React, { useState } from 'react';
import {
    Card,
    CardBody,
    CardHeader,
    Form,
    FormGroup,
    Label,
    Input,
    Button,
} from "reactstrap";

const AddUserForm = () => {
    const [username, setUsername] = useState('');

    const handleSubmit = (e) => {
        e.preventDefault();
        if (!username.trim()) return;

        const users = JSON.parse(localStorage.getItem('users') || '[]');
        const newUser = {
            id: Date.now().toString(),
            username: username.trim(),
            bindings: []
        };

        users.push(newUser);
        localStorage.setItem('users', JSON.stringify(users));
        window.location.href = '/admin/users';
    };

    return (
        <Card className="shadow">
            <CardHeader>
                <h3>Create New User</h3>
            </CardHeader>
            <CardBody>
                <Form onSubmit={handleSubmit}>
                    <FormGroup>
                        <Label for="username">Username</Label>
                        <Input
                            id="username"
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                            placeholder="Enter username"
                            required
                        />
                    </FormGroup>
                    <Button type="submit" color="primary">
                        Create User
                    </Button>
                </Form>
            </CardBody>
        </Card>
    );
};

export default AddUserForm;
