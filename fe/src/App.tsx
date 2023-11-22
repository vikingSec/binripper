import IMAGE from "./images/datboi.gif";
import "./assets/tailwind.css"
import { useEffect, useState } from "react";

export const App = () => {
    const [fileState, changeFs] = useState<File | null>();
    const [fileContent, changeFc] = useState<ArrayBuffer | null>();
    const handleFcChange = async () => {
        if(fileState != null){
            let content = await await fileState!.arrayBuffer();
            console.log(content)
            // var usernameView = new Uint8Array(content, 0, 16);
            // for(var i = 0; i < usernameView.length; i++){
            //     console.log("["+i+"]"+usernameView[i].toString(16))
            // }
            //TODO: Get the file content sent off to the API in a POST request
         } else {
            console.log("file state not set!");
         }
    }
    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        console.log(e.target.files![0]!);
        changeFs(e.target.files![0]!);
        
    }
    useEffect(() => {
        handleFcChange();
    }, [fileState])
    return (
        <div className="flex flex-row bg-black h-screen font-mono">
            <div className="flex flex-col mx-auto">

                <img src={IMAGE} alt="" />
                <label htmlFor="file" className="text-terminalgreen text-4xl text-center">Upload file...</label>
                <input id="file" type="file" onChange={handleFileChange} />
            </div>
            {fileState && (
                
                <div className="text-terminalgreen text-2xl text-center">
                <h3>Name: {fileState.name}</h3>
                <h3>Type: {fileState.type}</h3>
                <h3>Size: {fileState.size}</h3>

                </div>
            )}
        </div>
    );
}