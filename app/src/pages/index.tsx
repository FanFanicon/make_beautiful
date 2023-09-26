import { useEffect,useState } from "react";
import requests from "../utils/request";


export default function HomePage() {

  const [data, setData] = useState('')
  const getData=async()=>{
    const res =await requests.get("/api/hello")
    console.log(res);
    setData(res)
  }
  useEffect(() => {
    getData()
  }, [])
  
  return (
    <div>
      <span>{data}</span>
    </div>
  );
}
