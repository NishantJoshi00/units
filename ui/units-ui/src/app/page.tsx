"use client";

import LoadDriverForm from "@/components/LoadDriverForm";
import BindForm from "@/components/BindForm";
import ExecuteForm from "@/components/ExecuteForm";

import { useState } from "react";

export default function Home() {
  const [bindCount, setBindCount] = useState<number>(0);
  const [ExecuteCount, setExecuteCount] = useState<number>(0);
  const [drivers,setDrivers]= useState(false);

  return (
    <main className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">UNITS POC</h1>
      <div className="space-y-8">
        <LoadDriverForm drivers={drivers} setDrivers={setDrivers}/>
        <BindForm drivers={drivers}/>
        <ExecuteForm />
      </div>
    </main>
  );
}
