import { DriverClient, DriverDetailsClient, ExecutionClient, BindClient } from './service_grpc_web_pb';
import { ListResolverRequest, DriverDetailsRequest, ExecutionRequest, LoadDriverRequest, BinaryType, SubmitProgramRequest, BindRequest, ListProgramRequest} from './service_pb';

const API_DOMAIN = 'http://127.0.0.1:8080'

function createDriverClient() {
  return new DriverClient(API_DOMAIN, null, null)
}

function createDriverDetailClient() {
  return new DriverDetailsClient(API_DOMAIN, null, null)
}

function createExecutionClient() {
  return new ExecutionClient(API_DOMAIN, null, null);
}

function createBindClient() {
  return new BindClient(API_DOMAIN, null, null);
}

export const getDriverList = () => {
  return new Promise((resolve, reject) => {
    const driverDetailsClient = createDriverDetailClient()
    const request = new DriverDetailsRequest()
    driverDetailsClient.sendDetails(request, {}, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      resolve(response.toObject())
    })
  })
}

export const getResolverList = () => {
  return new Promise((resolve, reject) => {
    const driverClient = createDriverClient()
    const request = new ListResolverRequest()
    driverClient.listResolver(request, {}, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      resolve(response.toObject())
    })
  })
}

export const executeCommand = (executeData) => {
  return new Promise(async (resolve, reject) => {
    const executeClient = createExecutionClient()
    const request = new ExecutionRequest()
    const { input, programId, metadata = {} } = executeData

    request
      .setInput(input)
      .setProgramId(programId);

      const sanitizedMetadata = {};
      Object.keys(metadata).forEach((key) => {
        sanitizedMetadata[key] = encodeURIComponent(metadata[key]); // Encode non-ASCII values
      });

    executeClient.execute(request, sanitizedMetadata, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      // resolve(response.getMessage())
      resolve(JSON.stringify(response.toObject()))
    })
  })
}

export const loadDriver = (driverData) => {
  return new Promise(async (resolve, reject) => {
    const driverClient = createDriverClient()
    const request = new LoadDriverRequest()
    const { driverName, driverVersion, driverBinary } = driverData

    const buffer = await driverBinary.arrayBuffer();

    request
      .setDriverName(driverName)
      .setDriverVersion(driverVersion)
      .setDriverBinary(new Uint8Array(buffer));

    driverClient.loadDriver(request, driverData, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      // resolve(response.getMessage())
      resolve(JSON.stringify(response.toObject()))
    })
  })
}

export const submit = async ({ name, version, binary }) => {
  return new Promise(async (resolve, reject) => {
    const executeClient = createExecutionClient()
    const request = new SubmitProgramRequest()
    const buffer = await binary.arrayBuffer();
    request
      .setName(name)
      .setVersion(version)
      .setBinary(new Uint8Array(buffer));

    executeClient.submit(request, {}, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      // resolve(response.getMessage())
      resolve(JSON.stringify(response.toObject()))
    })
  })
}

export const bindUser = ( driverName, driverVersion, path, accountInfo ) => {
  return new Promise(async (resolve, reject) => {
    const client = createBindClient();

    const request = new BindRequest();
    request
        .setDriverName(driverName)
        .setDriverVersion(driverVersion)
        .setPath(path)
        .setAccountInfo(accountInfo);

    client.bind(request, {}, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      //resolve(response.getMessage())
      resolve(JSON.stringify(response.toObject()))
    });
  })
}

export const getPrograms = () => {
  return new Promise((resolve, reject) => {
    const client = createExecutionClient()
    const request = new ListProgramRequest()
    client.list(request, {}, (err, response) => {
      if (err) {
        reject(err);
        return
      }
      resolve(response.toObject())
    })
  })
}