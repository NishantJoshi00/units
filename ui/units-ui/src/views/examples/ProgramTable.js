import { useEffect, useState } from "react";
import {
    Card,
    CardHeader,
    CardBody,
    Table,
    Badge,
    Button
} from 'reactstrap';
import { getPrograms } from '../../grpcClient'

const sanitizeLocalStorage = (programList) => { 
    const output = localStorage.getItem("output")
    let outputObj = {}
    if(output) {
        outputObj = JSON.parse(output)
    }
    const preProgramIds = Object.keys(outputObj)
    preProgramIds.forEach((preProgramId) => {
        if(!programList.some(obj => obj.programId === preProgramId)) {
            delete outputObj[preProgramId]
        }
    })
    localStorage.setItem("output", JSON.stringify(outputObj))
}

const ProgramTable = ({ onRowSelect, selectedProgramId, onExecuteCTAClick }) => {
    const [list, setList] = useState([{
        programId: 1,
        name: "Program 1",
        version: '1.0.0'
    }, {
        programId: 2,
        name: "Program 2",
        version: '1.0.0'
    }, {
        programId: 3,
        name: "Program 3",
        version: '1.0.0'
    }, {
        programId: 4,
        name: "Program 4",
        version: '1.0.0'
    }])

    const fetchList = async () => {
        try {
            const res = await getPrograms()
            console.log(res)
            const { programList } = res
            setList(programList)
            sanitizeLocalStorage(programList)
        } catch (error) {
            console.log(error)
        }
    }

    useEffect(() => {
        fetchList()
    }, [])

    const rowClickHandler = (programId) => () => {
        onRowSelect(programId)
    }

    return (
        <Card className="shadow programs">
            <CardHeader>
                <h3 className="mb-0">Program Details</h3>
                <Button className='navigateToBindCTA' color="primary" type="submit" block onClick={() => window.location.href = '/admin/programs/upload'}>
                    Add
                </Button>
            </CardHeader>
            <CardBody>
                <Table responsive hover className="align-items-center">
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Version</th>
                        </tr>
                    </thead>
                    <tbody>
                        {list.map((driver, index) => (
                            <tr key={index} className={`cursor-pointer ${selectedProgramId === driver.programId ? 'selected-program-row' : ''}`} onClick={onExecuteCTAClick(driver.programId, driver.name)}>
                                <td>
                                    <Badge color="info" className="badge-lg">
                                        {driver.name}
                                    </Badge>
                                </td>
                                <td>
                                    <Badge color="success" className="badge-lg">
                                        {driver.version}
                                    </Badge>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </Table>
            </CardBody>
        </Card>
    )
}

export default ProgramTable;
