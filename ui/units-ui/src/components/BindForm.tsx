"use client";

import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";
import { Textarea } from "@/components/ui/textarea";
import { bind } from "@/lib/backend";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuCheckboxItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu"


import { JsonPrettifier } from "./JsonPrettify";
import { getDriverList } from "@/utils/grpcClient";

export default function BindForm() {
  const [driverName, setDriverName] = useState("");
  const [path, setPath] = useState("");
  const [accountInfo, setAccountInfo] = useState("");
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState<string | null>(null);
  const [driverList, setDriverList] = useState([])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    // Simulating an API call
    try {
      const response = await bind(driverName, path, accountInfo);
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
    setPath("");
    setAccountInfo("");
  };

  const driverSelectHandler = (driverName: string) => () => {
    setDriverName(driverName)
  }

  const fetchDrivers = async () => {
    try {
      const res = await getDriverList()
      console.log(res)
    } catch (error) {
      console.log(error)
    }
    const mockRes = {
      "success": true,
      "driverData": [
        {
          "name": "mono",
          "version": "0.1.0"
        },
        {
          "name": "beta",
          "version": "0.1.0"
        }
      ]
    }

    const driverList = mockRes.driverData.map(driverObj => driverObj.name)
    setDriverList(driverList)
  }

  useEffect(() => {
    fetchDrivers()
  }, [])

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
