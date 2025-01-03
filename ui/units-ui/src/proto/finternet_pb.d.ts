import * as jspb from 'google-protobuf'



export class LoadDriverRequest extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): LoadDriverRequest;

  getDriverVersion(): string;
  setDriverVersion(value: string): LoadDriverRequest;

  getDriverType(): BinaryType;
  setDriverType(value: BinaryType): LoadDriverRequest;

  getDriverBinary(): Uint8Array | string;
  getDriverBinary_asU8(): Uint8Array;
  getDriverBinary_asB64(): string;
  setDriverBinary(value: Uint8Array | string): LoadDriverRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoadDriverRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LoadDriverRequest): LoadDriverRequest.AsObject;
  static serializeBinaryToWriter(message: LoadDriverRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoadDriverRequest;
  static deserializeBinaryFromReader(message: LoadDriverRequest, reader: jspb.BinaryReader): LoadDriverRequest;
}

export namespace LoadDriverRequest {
  export type AsObject = {
    driverName: string,
    driverVersion: string,
    driverType: BinaryType,
    driverBinary: Uint8Array | string,
  }
}

export class LoadDriverResponse extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): LoadDriverResponse;

  getDriverVersion(): string;
  setDriverVersion(value: string): LoadDriverResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoadDriverResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LoadDriverResponse): LoadDriverResponse.AsObject;
  static serializeBinaryToWriter(message: LoadDriverResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoadDriverResponse;
  static deserializeBinaryFromReader(message: LoadDriverResponse, reader: jspb.BinaryReader): LoadDriverResponse;
}

export namespace LoadDriverResponse {
  export type AsObject = {
    driverName: string,
    driverVersion: string,
  }
}

export class UnloadDriverRequest extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): UnloadDriverRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UnloadDriverRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UnloadDriverRequest): UnloadDriverRequest.AsObject;
  static serializeBinaryToWriter(message: UnloadDriverRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UnloadDriverRequest;
  static deserializeBinaryFromReader(message: UnloadDriverRequest, reader: jspb.BinaryReader): UnloadDriverRequest;
}

export namespace UnloadDriverRequest {
  export type AsObject = {
    driverName: string,
  }
}

export class UnloadDriverResponse extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): UnloadDriverResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UnloadDriverResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UnloadDriverResponse): UnloadDriverResponse.AsObject;
  static serializeBinaryToWriter(message: UnloadDriverResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UnloadDriverResponse;
  static deserializeBinaryFromReader(message: UnloadDriverResponse, reader: jspb.BinaryReader): UnloadDriverResponse;
}

export namespace UnloadDriverResponse {
  export type AsObject = {
    driverName: string,
  }
}

export class BindRequest extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): BindRequest;

  getPath(): string;
  setPath(value: string): BindRequest;

  getAccountInfo(): string;
  setAccountInfo(value: string): BindRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BindRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BindRequest): BindRequest.AsObject;
  static serializeBinaryToWriter(message: BindRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BindRequest;
  static deserializeBinaryFromReader(message: BindRequest, reader: jspb.BinaryReader): BindRequest;
}

export namespace BindRequest {
  export type AsObject = {
    driverName: string,
    path: string,
    accountInfo: string,
  }
}

export class BindResponse extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): BindResponse;

  getPath(): string;
  setPath(value: string): BindResponse;

  getAccountInfo(): string;
  setAccountInfo(value: string): BindResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BindResponse.AsObject;
  static toObject(includeInstance: boolean, msg: BindResponse): BindResponse.AsObject;
  static serializeBinaryToWriter(message: BindResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BindResponse;
  static deserializeBinaryFromReader(message: BindResponse, reader: jspb.BinaryReader): BindResponse;
}

export namespace BindResponse {
  export type AsObject = {
    driverName: string,
    path: string,
    accountInfo: string,
  }
}

export class UnbindRequest extends jspb.Message {
  getPath(): string;
  setPath(value: string): UnbindRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UnbindRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UnbindRequest): UnbindRequest.AsObject;
  static serializeBinaryToWriter(message: UnbindRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UnbindRequest;
  static deserializeBinaryFromReader(message: UnbindRequest, reader: jspb.BinaryReader): UnbindRequest;
}

export namespace UnbindRequest {
  export type AsObject = {
    path: string,
  }
}

export class UnbindResponse extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): UnbindResponse;

  getAccountInfo(): string;
  setAccountInfo(value: string): UnbindResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UnbindResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UnbindResponse): UnbindResponse.AsObject;
  static serializeBinaryToWriter(message: UnbindResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UnbindResponse;
  static deserializeBinaryFromReader(message: UnbindResponse, reader: jspb.BinaryReader): UnbindResponse;
}

export namespace UnbindResponse {
  export type AsObject = {
    driverName: string,
    accountInfo: string,
  }
}

export class ExecutionRequest extends jspb.Message {
  getName(): string;
  setName(value: string): ExecutionRequest;

  getInput(): string;
  setInput(value: string): ExecutionRequest;

  getType(): BinaryType;
  setType(value: BinaryType): ExecutionRequest;

  getBinary(): Uint8Array | string;
  getBinary_asU8(): Uint8Array;
  getBinary_asB64(): string;
  setBinary(value: Uint8Array | string): ExecutionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecutionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ExecutionRequest): ExecutionRequest.AsObject;
  static serializeBinaryToWriter(message: ExecutionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecutionRequest;
  static deserializeBinaryFromReader(message: ExecutionRequest, reader: jspb.BinaryReader): ExecutionRequest;
}

export namespace ExecutionRequest {
  export type AsObject = {
    name: string,
    input: string,
    type: BinaryType,
    binary: Uint8Array | string,
  }
}

export class ExecutionResponse extends jspb.Message {
  getOutput(): string;
  setOutput(value: string): ExecutionResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecutionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ExecutionResponse): ExecutionResponse.AsObject;
  static serializeBinaryToWriter(message: ExecutionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecutionResponse;
  static deserializeBinaryFromReader(message: ExecutionResponse, reader: jspb.BinaryReader): ExecutionResponse;
}

export namespace ExecutionResponse {
  export type AsObject = {
    output: string,
  }
}

export enum BinaryType { 
  WAT = 0,
  WASM = 1,
}
