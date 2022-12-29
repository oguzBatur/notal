import React, { useState } from "react";
import Preview from "./Preview";
import { invoke } from "@tauri-apps/api/tauri";

interface BufferProps {
  filePath: String,
  fileName: String,
}


/**
 * Displays the text and preview areas, also the tabs.
 *
 *  */
const Buffer = ({filePath}:BufferProps) => {
  const [input, setInput] = useState<String>("") // Store the input gathered from textarea here.
  const getInput = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
     invoke("format_text", {input: e.currentTarget.value}).then((val) => {
      setInput(val as String); 
     });
  }



  return(
  <div id="buffer">
    <div id="buffercontainer">
    
    <textarea onChange={getInput} id="editor"/>
    <Preview input={input}/>
    </div>
  </div>

  )
}


export default Buffer;
