import { useEffect, useState } from "react";
import { getDriverList } from "../grpcClient";
import SupportTokenDetails from "./SupportTokenDetails";
import Upload from "./examples/Upload";

const SupportTokens = () => {
  const [tokenList, setTokenList] = useState([])
  // const [showUpload, setShowUpload] = useState(false)

  const fetchDriverList = async () => {
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
      setTokenList(driverDataList)
    } catch (error) {
      console.log(error)
    }
  }

  // const updateShowUpload = (value) => () => {
  //   if (showUpload !== value) {
  //     setShowUpload(value)
  //   }
  // }

  // const onUploadSuccess = () => {
  //   fetchDriverList()
  // }

  useEffect(() => {
    fetchDriverList()
  }, [])

  return (
    <div className='support-tokens-container'>
      <SupportTokenDetails data={tokenList}  />
      {/* <Upload onUploadSuccess={onUploadSuccess}/> */}
    </div>)
};

export default SupportTokens;