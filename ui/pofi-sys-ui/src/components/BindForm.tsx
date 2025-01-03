"use client";

import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";
import { Textarea } from "@/components/ui/textarea";
import { bind } from "@/lib/backend";
import { JsonPrettifier } from "./JsonPrettify";

export default function BindForm() {
  const [driverName, setDriverName] = useState("");
  const [path, setPath] = useState("");
  const [accountInfo, setAccountInfo] = useState("");
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState<string | null>(null);

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
              <Input
                id="driverName"
                value={driverName}
                onChange={(e) => setDriverName(e.target.value)}
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
