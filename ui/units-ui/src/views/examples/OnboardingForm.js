import React, { useState } from "react";
import './onboardingForm.css'
import { Container, Card, CardBody, Form, FormGroup, Input, Button, Label, CustomInput } from "reactstrap";
import { Shield } from "lucide-react";

const OnboardingForm = () => {
    const [mobileNumber, setMobileNumber] = useState("");
    const [acceptedTerms, setAcceptedTerms] = useState(false);

    return (
        <div className="onboarding-form">
            <img className="logo" src="https://finternet-playground.vercel.app/FinternetLogoWhite.png" />
            <header className="welcome-desc">Welcome to Finternet</header>
            <footer>
                <svg height="12" width="12" fill="none" viewBox="0 0 12 12" xmlns="http://www.w3.org/2000/svg"><g clip-path="url(#clip0_1294_5199)"><path d="M6.05182 12C5.97752 12 5.90323 11.9829 5.83618 11.9482L3.67552 10.8298C3.06242 10.5121 2.58281 10.1555 2.2083 9.74013C1.38862 8.83172 0.932566 7.6656 0.925317 6.45595L0.899925 2.47456C0.896927 2.01505 1.19351 1.60262 1.63688 1.44729L5.60423 0.0640729C5.8398 -0.0200617 6.10256 -0.0212384 6.34237 0.0599544L10.3248 1.39611C10.7706 1.54496 11.072 1.95445 11.0744 2.41337L11.0998 6.3977C11.1077 7.60559 10.6673 8.77641 9.8603 9.69483C9.49003 10.1161 9.01465 10.4779 8.40758 10.8015L6.26626 11.9453C6.19981 11.9812 6.12612 11.9994 6.05182 12Z" fill="#FFFFFF" opacity="0.4"></path><path d="M5.59113 7.39216C5.47515 7.39275 5.35918 7.35097 5.26978 7.26507L4.11968 6.15897C3.94209 5.98717 3.94028 5.70829 4.11606 5.53531C4.29184 5.36175 4.57876 5.35998 4.75695 5.5312L5.58448 6.32665L7.60501 4.33448C7.78139 4.16092 8.06831 4.15916 8.24589 4.33037C8.42409 4.50217 8.4259 4.78163 8.25012 4.95402L5.91067 7.26095C5.82248 7.34803 5.70711 7.39157 5.59113 7.39216Z" fill="#FFFFFF"></path></g><defs><clipPath id="clip0_1294_5199"><rect height="12" width="10.2" fill="white" transform="translate(0.899902)"></rect></clipPath></defs></svg>
                <div class="footer-desc">Secured by&nbsp;<span class="finternet">Finternet </span></div>
            </footer>
        </div>
    );
};

export default OnboardingForm;
