import React from "react";
import { Button, FormGroup, Label, Input } from "reactstrap";

const FileUpload = ({onUpload, fileType, fileName}) => {

  return (
    <FormGroup className="text-center">
      <Label
        for="fileUpload"
        className="d-block border rounded p-4 fileUploadLabel"
        style={{
          cursor: "pointer",
          borderStyle: "dashed",
          background: "#f8f9fa",
        }}
      >
        <i class="fa-solid fa-arrow-up-from-bracket"  style={{ fontSize: "2rem" }}></i>
        <div className="mt-2">Click to Upload files</div>
        <span style={{color: 'blue'}} color="link" className="p-0">
          browse
        </span>
        {fileName && <div style={{color: 'green'}} className="mt-2">{fileName}</div>}
      </Label>
      <Input id="fileUpload" type="file" hidden onChange={onUpload} accept={fileType === 'WASM' ? '.wasm' : ''}/>
    </FormGroup>
  );
};

export default FileUpload;
