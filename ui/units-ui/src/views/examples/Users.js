import { getResolverList, getDriverList } from '../../grpcClient'
import { useEffect, useState } from "react";
import AccountInfoTable from './UsersTable'

const Users = () => {
  const [userList, setUserList] = useState([])
  const [drivers, setDrivers] = useState([])
  const [selectedDrivers, setSelectedDrivers] = useState([])

  const fetchUsers = async () => {
    try {
      const usersList = await getResolverList()
      console.log(usersList)
      const { pathMappingList } = usersList
      setUserList(pathMappingList)
    } catch (error) {
      console.log(error)
    }
  }
  
  const fetchDrivers = async () => {
    try {
      const driverList = await getDriverList()
      const { driverDataList } = driverList
      console.log(driverDataList)
      // setTokenList([...driverDataList, {
      //   name: "mono2",
      //   version: "0.1.0"
      // },{
      //   name: "mono3",
      //   version: "0.1.0"
      // }
      // ])
      setDrivers(driverDataList)
    } catch (error) {
      console.log(error)
    }
    
  }

  useEffect(() => {
    fetchUsers()
    fetchDrivers()
  }, [])

  const multiSelectChangeHandler = (selected) => {
    setSelectedDrivers(selected)
  }

  const handleSubmit = async (e) => {
    console.log("submit")
  }

  return (
    <div className='support-tokens-container'>
      <AccountInfoTable data={userList} />
    </div>
  );
};

export default Users;
