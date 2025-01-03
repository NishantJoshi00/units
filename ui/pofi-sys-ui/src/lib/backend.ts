
import { BindRequest, LoadDriverRequest, ExecutionRequest, BinaryType } from '@/proto/finternet_pb';
import { BindClient, DriverClient, ExecutionClient } from '@/proto/FinternetServiceClientPb';

async function load_driver(
    driverName: string,
    driverVersion: string,
    driverType: string,
    driverBinary: File,
): Promise<string> {
    const client = new DriverClient("http://localhost:8080");

    const request = new LoadDriverRequest();

    const file = await driverBinary.arrayBuffer();


    request
        .setDriverName(driverName)
        .setDriverVersion(driverVersion)
        .setDriverType(driverType === "WASM" ? BinaryType.WASM : BinaryType.WAT)
        .setDriverBinary(new Uint8Array(file));

    const response = await client.loadDriver(request, {});

    await new Promise((resolve) => setTimeout(resolve, 1000)); // 1 second delay
    return JSON.stringify(response.toObject());
}

async function bind(
    driverName: string,
    path: string,
    accountInfo: string,
): Promise<string> {
    const client = new BindClient("http://localhost:8080");

    const request = new BindRequest();
    request
        .setDriverName(driverName)
        .setPath(path)
        .setAccountInfo(accountInfo);

    const response = await client.bind(request, {});

    await new Promise((resolve) => setTimeout(resolve, 1000)); // 1 second delay
    return JSON.stringify(response.toObject());
}

async function execute(
    name: string,
    input: string,
    type: string,
    binary: File,
): Promise<string> {
    const client = new ExecutionClient("http://localhost:8080");

    const request = new ExecutionRequest();

    const file = await binary.arrayBuffer();

    request
        .setName(name)
        .setInput(input)
        .setType(type === "WASM" ? BinaryType.WASM : BinaryType.WAT)
        .setBinary(new Uint8Array(file));

    const response = await client.execute(request, {});

    await new Promise(resolve => setTimeout(resolve, 1000)) // 1 second delay
    return JSON.stringify(response.toObject());
}


export { load_driver, bind, execute };
