"use client";

import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";
import { execute } from "@/lib/backend";
import { JsonPrettifier } from "@/components/JsonPrettify";

export default function ExecuteForm() {
  const [name, setName] = useState("");
  const [input, setInput] = useState("");
  const [type, setType] = useState("WASM");
  const [binary, setBinary] = useState<File | null>(null);
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    // Simulating an API call
    try {
      const response = await execute(name, input, type, binary!);
      setOutput(response);
    } catch (error) {
      setOutput("An error occurred during execution.");
    } finally {
      setLoading(false);
    }
  };

  const resetForm = () => {
    setOutput(null);
    setName("");
    setInput("");
    setType("WAT");
    setBinary(null);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Execute Program (app triggered)</CardTitle>
      </CardHeader>
      <CardContent>
        {loading ? (
          <div className="flex justify-center items-center h-64">
            <Loader2 className="h-8 w-8 animate-spin" />
          </div>
        ) : output ? (
          <div className="space-y-4">
            <JsonPrettifier output={output} />
            <Button onClick={resetForm}>Execute Another</Button>
          </div>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <Label htmlFor="name">Program Name</Label>
              <Input
                id="name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                required
              />
            </div>
            <div>
              <Label htmlFor="input">Input</Label>
              <Textarea
                id="input"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                className="font-mono resize-none"
                required
              />
              <p className="text-sm text-muted-foreground">
                The input is passed to the WebAssembly module as a JSON.
              </p>
            </div>
            <div>
              <Label htmlFor="type">Type</Label>
              <Select value={type} onValueChange={setType}>
                <SelectTrigger>
                  <SelectValue placeholder="Select type" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="WAT">WAT</SelectItem>
                  <SelectItem value="WASM">WASM</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div>
              <Label htmlFor="binary">Binary</Label>
              <Input
                id="binary"
                type="file"
                className='hover:bg-secondary cursor-pointer'
                accept={
                  type === "WAT" ? ".wat" : type === "WASM" ? ".wasm" : ""
                }
                onChange={(e) => setBinary(e.target.files?.[0] || null)}
                required
              />
            </div>
            <Button type="submit">Execute</Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
