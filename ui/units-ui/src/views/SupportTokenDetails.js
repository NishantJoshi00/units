import React from 'react';
import {
  Card,
  CardHeader,
  CardBody,
  Table,
  Badge
} from 'reactstrap';

const SupportTokenDetails = ({ data }) => {

  const addClickHandler = () => {
    window.location.href = '/admin/supported-token-drivers/upload'
  }

  return (
    <Card className="shadow">
      <CardHeader>
        <h3 className="mb-0">Token Driver Details</h3>
        <button onClick={addClickHandler} style={{ position: 'absolute', right: '18px', top: '18px', border: 'none', borderRadius: '10px', background: '#5e72e4', color: '#fff', padding: '6px 12px' }}>Add</button>
      </CardHeader>
      <CardBody>
        <Table responsive hover className="align-items-center">
          <thead>
            <tr>
              <th>Driver Name</th>
              <th>Version</th>
            </tr>
          </thead>
          <tbody>
            {data.map((driver, index) => (
              <tr key={index}>
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
  );
};

export default SupportTokenDetails;
