"use client";

import { useEffect, useState ,useRef} from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";
import { Textarea } from "@/components/ui/textarea";
import { bind,createDriverDetailClient  } from "@/lib/backend";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuCheckboxItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu"


import { JsonPrettifier } from "./JsonPrettify";
// import { getDriverList } from "@/utils/grpcClient";


type BindFormProps = {
  drivers: boolean;
};

export default function BindForm({ drivers }: BindFormProps) {
  const [driverName, setDriverName] = useState("");
  const [driverVersion, setDriverVersion] = useState("");
  const [path, setPath] = useState("");
  const [accountInfo, setAccountInfo] = useState("");
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState<string | null>(null);
  const [driverList, setDriverList] = useState<string[]>(["No Token Handler"])
  const prevDriverListRef = useRef<string[]>([]);

  interface DriverDetail {
    name: string;
    version: string;
  }
  
  interface DriverDetailResponse {
    message: string;
    driverDataList: DriverDetail[];
  }


  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    // Simulating an API call
    try {
      const response = await bind(driverName,driverVersion, path, accountInfo);
      setOutput(response);
    } catch (error) {
      setOutput("An error occurred while binding the driver.");
    } finally {
      setLoading(false);
    }
  };

  const resetForm = () => {
    setOutput(null);
    setDriverName("");
    setDriverVersion("");
    setPath("");
    setAccountInfo("");
  };

  const driverSelectHandler = (driverName: string) => () => {
    setDriverName(driverName)
  }

  const fetchDrivers = async () => {
    try {
      const res = await createDriverDetailClient()
      // console.log("Raw response:", res);

    // Parse the response into a JSON object
    const response = JSON.parse(res) as DriverDetailResponse;
    // console.log("Parsed response:", res);
      // console.log("Response:", response);

      const { driverDataList } = response || {};
      // console.log("Driver Data List after destructuring:", driverDataList);

      // const mockRes = {
      //   "success": true,
      //   "driverData": [
      //     {
      //       "name": "mono",
      //       "version": "0.1.0"
      //     },
      //     {
      //       "name": "beta",
      //       "version": "0.1.0"
      //     }
      //   ]
      // }
      
      const driverList = response.driverDataList.map((driverObj) => driverObj.name);
      // console.log("Driver Names:", driverList);
      if (driverList.length > 0) {
        setDriverList(driverList);
      }
      
    } catch (error) {
      console.log(error)
    }
  }

  useEffect(() => {
    fetchDrivers()
  }, [drivers])

  return (
    <Card>
      <CardHeader>
        <CardTitle>User onboarding</CardTitle>
      </CardHeader>
      <CardContent>
        {loading ? (
          <div className="flex justify-center items-center h-64">
            <Loader2 className="h-8 w-8 animate-spin" />
          </div>
        ) : output ? (
          <div className="space-y-4">
            <JsonPrettifier output={output} />
            <Button onClick={resetForm}>Add asset account</Button>
          </div>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <Label htmlFor="driverName">Token Handler Name</Label>
              <br />
              <DropdownMenu>
                <DropdownMenuTrigger className="w-full" asChild>
                  <Button className="text-left justify-start" variant="outline">{driverName || 'Select Handler'}</Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent className="w-56">
                  {Array.isArray(driverList) && driverList.map((driver: string) => {
                    return (
                      <DropdownMenuCheckboxItem
                        key={driver}
                        checked={driverName === driver}
                        onCheckedChange={driverSelectHandler(driver)}
                      >
                        {driver}
                      </DropdownMenuCheckboxItem>
                    )
                  })}
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
            <div>
              <Label htmlFor="version">Driver Version</Label>
              <Input
                id="version"
                value={driverVersion}
                onChange={(e) => setDriverVersion(e.target.value)}
                required
              />
            </div>
            <div>
              <Label htmlFor="path">Token Path</Label>
              <Input
                id="path"
                value={path}
                onChange={(e) => setPath(e.target.value)}
                required
              />
            </div>
            <div>
              <Label htmlFor="accountInfo">Account Info</Label>
              <Textarea
                id="accountInfo"
                value={accountInfo}
                onChange={(e) => setAccountInfo(e.target.value)}
                className="font-mono resize-none"
                required
              />
              <p className="text-sm text-muted-foreground">
                The input is passed to the WebAssembly module as a JSON.
              </p>
            </div>
            <Button type="submit">Onboard</Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
