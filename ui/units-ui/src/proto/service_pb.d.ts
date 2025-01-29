import * as jspb from 'google-protobuf'



export class ListResolverRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListResolverRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListResolverRequest): ListResolverRequest.AsObject;
  static serializeBinaryToWriter(message: ListResolverRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListResolverRequest;
  static deserializeBinaryFromReader(message: ListResolverRequest, reader: jspb.BinaryReader): ListResolverRequest;
}

export namespace ListResolverRequest {
  export type AsObject = {
  }
}

export class ListResolverResponse extends jspb.Message {
  getPathMappingList(): Array<PathMapping>;
  setPathMappingList(value: Array<PathMapping>): ListResolverResponse;
  clearPathMappingList(): ListResolverResponse;
  addPathMapping(value?: PathMapping, index?: number): PathMapping;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListResolverResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListResolverResponse): ListResolverResponse.AsObject;
  static serializeBinaryToWriter(message: ListResolverResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListResolverResponse;
  static deserializeBinaryFromReader(message: ListResolverResponse, reader: jspb.BinaryReader): ListResolverResponse;
}

export namespace ListResolverResponse {
  export type AsObject = {
    pathMappingList: Array<PathMapping.AsObject>,
  }
}

export class PathMapping extends jspb.Message {
  getPath(): string;
  setPath(value: string): PathMapping;

  getDriverName(): string;
  setDriverName(value: string): PathMapping;

  getDriverVersion(): string;
  setDriverVersion(value: string): PathMapping;

  getAccountInfo(): string;
  setAccountInfo(value: string): PathMapping;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PathMapping.AsObject;
  static toObject(includeInstance: boolean, msg: PathMapping): PathMapping.AsObject;
  static serializeBinaryToWriter(message: PathMapping, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PathMapping;
  static deserializeBinaryFromReader(message: PathMapping, reader: jspb.BinaryReader): PathMapping;
}

export namespace PathMapping {
  export type AsObject = {
    path: string,
    driverName: string,
    driverVersion: string,
    accountInfo: string,
  }
}

export class LoadDriverRequest extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): LoadDriverRequest;

  getDriverVersion(): string;
  setDriverVersion(value: string): LoadDriverRequest;

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

  getDriverVersion(): string;
  setDriverVersion(value: string): UnloadDriverRequest;

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
    driverVersion: string,
  }
}

export class UnloadDriverResponse extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): UnloadDriverResponse;

  getDriverVersion(): string;
  setDriverVersion(value: string): UnloadDriverResponse;

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
    driverVersion: string,
  }
}

export class BindRequest extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): BindRequest;

  getDriverVersion(): string;
  setDriverVersion(value: string): BindRequest;

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
    driverVersion: string,
    path: string,
    accountInfo: string,
  }
}

export class BindResponse extends jspb.Message {
  getDriverName(): string;
  setDriverName(value: string): BindResponse;

  getDriverVersion(): string;
  setDriverVersion(value: string): BindResponse;

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
    driverVersion: string,
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

  getDriverVersion(): string;
  setDriverVersion(value: string): UnbindResponse;

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
    driverVersion: string,
    accountInfo: string,
  }
}

export class ExecutionRequest extends jspb.Message {
  getInput(): string;
  setInput(value: string): ExecutionRequest;

  getBinary(): Uint8Array | string;
  getBinary_asU8(): Uint8Array;
  getBinary_asB64(): string;
  setBinary(value: Uint8Array | string): ExecutionRequest;
  hasBinary(): boolean;
  clearBinary(): ExecutionRequest;

  getProgramId(): string;
  setProgramId(value: string): ExecutionRequest;
  hasProgramId(): boolean;
  clearProgramId(): ExecutionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecutionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ExecutionRequest): ExecutionRequest.AsObject;
  static serializeBinaryToWriter(message: ExecutionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecutionRequest;
  static deserializeBinaryFromReader(message: ExecutionRequest, reader: jspb.BinaryReader): ExecutionRequest;
}

export namespace ExecutionRequest {
  export type AsObject = {
    input: string,
    binary?: Uint8Array | string,
    programId?: string,
  }

  export enum BinaryCase { 
    _BINARY_NOT_SET = 0,
    BINARY = 5,
  }

  export enum ProgramIdCase { 
    _PROGRAM_ID_NOT_SET = 0,
    PROGRAM_ID = 6,
  }
}

export class SubmitProgramRequest extends jspb.Message {
  getName(): string;
  setName(value: string): SubmitProgramRequest;

  getVersion(): string;
  setVersion(value: string): SubmitProgramRequest;

  getBinary(): Uint8Array | string;
  getBinary_asU8(): Uint8Array;
  getBinary_asB64(): string;
  setBinary(value: Uint8Array | string): SubmitProgramRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SubmitProgramRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SubmitProgramRequest): SubmitProgramRequest.AsObject;
  static serializeBinaryToWriter(message: SubmitProgramRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SubmitProgramRequest;
  static deserializeBinaryFromReader(message: SubmitProgramRequest, reader: jspb.BinaryReader): SubmitProgramRequest;
}

export namespace SubmitProgramRequest {
  export type AsObject = {
    name: string,
    version: string,
    binary: Uint8Array | string,
  }
}

export class ListProgramRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListProgramRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListProgramRequest): ListProgramRequest.AsObject;
  static serializeBinaryToWriter(message: ListProgramRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListProgramRequest;
  static deserializeBinaryFromReader(message: ListProgramRequest, reader: jspb.BinaryReader): ListProgramRequest;
}

export namespace ListProgramRequest {
  export type AsObject = {
  }
}

export class ListProgramResponse extends jspb.Message {
  getProgramList(): Array<Program>;
  setProgramList(value: Array<Program>): ListProgramResponse;
  clearProgramList(): ListProgramResponse;
  addProgram(value?: Program, index?: number): Program;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListProgramResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListProgramResponse): ListProgramResponse.AsObject;
  static serializeBinaryToWriter(message: ListProgramResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListProgramResponse;
  static deserializeBinaryFromReader(message: ListProgramResponse, reader: jspb.BinaryReader): ListProgramResponse;
}

export namespace ListProgramResponse {
  export type AsObject = {
    programList: Array<Program.AsObject>,
  }
}

export class Program extends jspb.Message {
  getProgramId(): string;
  setProgramId(value: string): Program;

  getName(): string;
  setName(value: string): Program;

  getVersion(): string;
  setVersion(value: string): Program;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Program.AsObject;
  static toObject(includeInstance: boolean, msg: Program): Program.AsObject;
  static serializeBinaryToWriter(message: Program, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Program;
  static deserializeBinaryFromReader(message: Program, reader: jspb.BinaryReader): Program;
}

export namespace Program {
  export type AsObject = {
    programId: string,
    name: string,
    version: string,
  }
}

export class SubmitProgramResponse extends jspb.Message {
  getProgramId(): string;
  setProgramId(value: string): SubmitProgramResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SubmitProgramResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SubmitProgramResponse): SubmitProgramResponse.AsObject;
  static serializeBinaryToWriter(message: SubmitProgramResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SubmitProgramResponse;
  static deserializeBinaryFromReader(message: SubmitProgramResponse, reader: jspb.BinaryReader): SubmitProgramResponse;
}

export namespace SubmitProgramResponse {
  export type AsObject = {
    programId: string,
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

export class DriverDetailsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DriverDetailsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DriverDetailsRequest): DriverDetailsRequest.AsObject;
  static serializeBinaryToWriter(message: DriverDetailsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DriverDetailsRequest;
  static deserializeBinaryFromReader(message: DriverDetailsRequest, reader: jspb.BinaryReader): DriverDetailsRequest;
}

export namespace DriverDetailsRequest {
  export type AsObject = {
  }
}

export class DriverDetail extends jspb.Message {
  getName(): string;
  setName(value: string): DriverDetail;

  getVersion(): string;
  setVersion(value: string): DriverDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DriverDetail.AsObject;
  static toObject(includeInstance: boolean, msg: DriverDetail): DriverDetail.AsObject;
  static serializeBinaryToWriter(message: DriverDetail, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DriverDetail;
  static deserializeBinaryFromReader(message: DriverDetail, reader: jspb.BinaryReader): DriverDetail;
}

export namespace DriverDetail {
  export type AsObject = {
    name: string,
    version: string,
  }
}

export class DriverDetailsResponse extends jspb.Message {
  getMessage(): string;
  setMessage(value: string): DriverDetailsResponse;

  getDriverDataList(): Array<DriverDetail>;
  setDriverDataList(value: Array<DriverDetail>): DriverDetailsResponse;
  clearDriverDataList(): DriverDetailsResponse;
  addDriverData(value?: DriverDetail, index?: number): DriverDetail;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DriverDetailsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DriverDetailsResponse): DriverDetailsResponse.AsObject;
  static serializeBinaryToWriter(message: DriverDetailsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DriverDetailsResponse;
  static deserializeBinaryFromReader(message: DriverDetailsResponse, reader: jspb.BinaryReader): DriverDetailsResponse;
}

export namespace DriverDetailsResponse {
  export type AsObject = {
    message: string,
    driverDataList: Array<DriverDetail.AsObject>,
  }
}

