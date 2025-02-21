import React, { useState } from "react";
import { Route, Routes } from "react-router-dom";
import './program.css';
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
import ProgramTable from "./ProgramTable";
import Execute from "views/examples/Execute.js";


const Program = () => {
    const [input, setInput] = useState("");
    const [selectedProgramId, setSelectedProgramId] = useState('');

    const onExecuteCTAClick = (programId, name) => () => {
        console.log("onExecuteCTAClick", programId)
        setSelectedProgramId(programId);
        localStorage.setItem("selectedProgramId", programId)
        window.location.href = `/admin/programs/execute`
    }

    console.log("selectedProgramId", selectedProgramId)

    return (
        <ProgramTable onExecuteCTAClick={onExecuteCTAClick} selectedProgramId={selectedProgramId} />
    );
};

export default Program;

