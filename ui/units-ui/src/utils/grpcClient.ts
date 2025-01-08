import {DriverDetailsClient} from '../service_grpc_web_pb';
import {DriverDetailsRequest} from '../service_pb';


// const API_DOMAIN = 'http://localhost:8080'
const API_DOMAIN = 'http://l127.0.0.1:8080'

function createDriverDetailClient() {
    return new DriverDetailsClient(API_DOMAIN, null, null)
}

export const getDriverList = () => {
    return new Promise((resolve, reject)=>{
        const driverDetailsClient = createDriverDetailClient()
        const request = new DriverDetailsRequest()
        driverDetailsClient.sendDetails(request, {}, (err, response) => {
            if(err) {
                reject(err);
                return
            }
            resolve(response.getMessage())
        })
    })
}
