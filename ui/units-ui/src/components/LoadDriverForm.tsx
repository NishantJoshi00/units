'use client'

import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Button } from '@/components/ui/button'
import { JsonPrettifier } from '@/components/JsonPrettify';
import { load_driver } from '@/lib/backend';
import { Loader2 } from 'lucide-react'


type LoadDriverFormProps = {
    drivers: boolean;
    setDrivers: React.Dispatch<React.SetStateAction<boolean>>;
};


export default function LoadDriverForm({ drivers, setDrivers }: LoadDriverFormProps) {
    const [driverName, setDriverName] = useState('')
    const [driverVersion, setDriverVersion] = useState('')
    const [driverType, setDriverType] = useState('WASM')
    const [driverBinary, setDriverBinary] = useState<File | null>(null)
    const [loading, setLoading] = useState(false)
    const [handlerType, setHandlerType] = useState('')
    const [storageType, setStorageType] = useState('')
    const [output, setOutput] = useState<string | null>(null)

    const handleToggle = () => {
        setDrivers(!drivers); // Toggle the state
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        setLoading(true)

        try {
            const response = await load_driver(driverName, driverVersion, driverType, driverBinary!);
            setOutput(response)
            handleToggle();
        } catch (error) {
            setOutput('An error occurred while loading the driver.')
            console.error(error);
        } finally {
            setLoading(false)
        }
    }

    const resetForm = () => {
        setOutput(null)
        setDriverName('')
        setDriverVersion('')
        setDriverType('WASM')
        setDriverBinary(null)
        setHandlerType('')
        setStorageType('')
    }

    return (
        <Card>
            <CardHeader>
                <CardTitle>Token Definition</CardTitle>
            </CardHeader>
            <CardContent>
                {loading ? (
                    <div className="flex justify-center items-center h-64">
                        <Loader2 className="h-8 w-8 animate-spin" />
                    </div>
                ) : output ? (
                    <div className="space-y-4">
                        <JsonPrettifier output={output} />
                        <Button onClick={resetForm}>Onboard another workflow</Button>
                    </div>
                ) : (
                    <form onSubmit={handleSubmit} className="space-y-4">
                        <div>
                            <Label htmlFor="driverName">Token Type Name</Label>
                            <Input
                                id="driverName"
                                value={driverName}
                                onChange={(e) => setDriverName(e.target.value)}
                                required
                            />
                        </div>

                        <div>
                            <Label htmlFor="handlerType">Token Handler Type</Label>
                            <Select value={handlerType} onValueChange={setHandlerType}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select token type" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Native">Native</SelectItem>
                                    <SelectItem value="Custodial">Custodial</SelectItem>
                                    <SelectItem value="Proxy">Proxy</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        {handlerType === "Native" && (
                            <div>
                                <Label htmlFor="storageType">Storage Type</Label>
                                <Select value={storageType} onValueChange={setStorageType}>
                                    <SelectTrigger>
                                        <SelectValue placeholder="Select storage type" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="Redis">Redis</SelectItem>
                                        <SelectItem value="Solana">Solana</SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>
                        )}


                        <div>
                            <Label htmlFor="driverVersion">Token Handler Version</Label>
                            <Input
                                id="driverVersion"
                                value={driverVersion}
                                onChange={(e) => setDriverVersion(e.target.value)}
                                required
                            />
                        </div>
                        <div>
                            <Label htmlFor="driverType">Binary Type</Label>
                            <Select value={driverType} onValueChange={setDriverType}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select driver type" />
                                </SelectTrigger> <SelectContent>
                                    <SelectItem value="WAT">WAT</SelectItem>
                                    <SelectItem value="WASM">WASM</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                        <div>
                            <Label htmlFor="driverBinary">Token Handler Binary</Label>
                            <Input
                                id="driverBinary"
                                type="file"
                                className='hover:bg-secondary cursor-pointer'
                                accept={driverType === 'WAT' ? '.wat' : driverType === 'WASM' ? '.wasm' : ''}
                                onChange={(e) => setDriverBinary(e.target.files?.[0] || null)}
                                required
                            />
                        </div>
                        <Button type="submit">Setup Token Handler</Button>
                    </form>
                )}
            </CardContent>
        </Card>
    )
}

