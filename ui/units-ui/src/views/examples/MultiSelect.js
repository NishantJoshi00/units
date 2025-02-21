import React, { useState } from "react";
import {
  Dropdown,
  DropdownToggle,
  DropdownMenu,
  DropdownItem,
  FormGroup,
  Input,
  Label,
} from "reactstrap";
import "bootstrap/dist/css/bootstrap.min.css";

const MultiSelect = ({ options = [], onChange, label = "Select Items" }) => {
  const [selectedItems, setSelectedItems] = useState([]);
  const [dropdownOpen, setDropdownOpen] = useState(false);

  const toggleDropdown = () => setDropdownOpen((prevState) => !prevState);

  const handleSelect = (option) => {
    let newSelectedItems;

    if (selectedItems.includes(option)) {
      newSelectedItems = selectedItems.filter((item) => item !== option);
    } else {
      newSelectedItems = [...selectedItems, option];
    }

    setSelectedItems(newSelectedItems);
    onChange?.(newSelectedItems);
  };

  return (
    <FormGroup className="position-relative w-100">
      <Label>{label}</Label>
      <Dropdown isOpen={dropdownOpen} toggle={toggleDropdown} className="w-100">
        <DropdownToggle caret color="light" className="w-100 text-left">
          {selectedItems.length === 0
            ? "Select items..."
            : `${selectedItems.length} selected`}
        </DropdownToggle>
        <DropdownMenu className="w-100">
          {options.map((option, index) => (
            <DropdownItem
              key={index}
              toggle={false}
              onClick={() => handleSelect(option)}
            >
              <Input
                type="checkbox"
                checked={selectedItems.includes(option)}
                onChange={() => {}}
                className="me-2"
              />
              {option}
            </DropdownItem>
          ))}
        </DropdownMenu>
      </Dropdown>
    </FormGroup>
  );
};

export default MultiSelect;