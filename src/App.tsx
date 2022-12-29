import Buffer from "./components/Editor";
import "./App.css";
import Tab from "./components/Tab";
import DictionaryTab from "./components/DictionaryTab";
import { useState } from "react";
import MainMenu from "./components/MainMenu";

function App() {
  const [folderSelected, setFolderSelected] = useState(false);

  const setMenuState = () => {
    if(!folderSelected) {
      return(
       <MainMenu/>
      )
    }
    else {
      return(
      <div>
        <ul id="tabs">
          <Tab tabName={"first_file.txt"}/>
          <Tab tabName={"second_file.txt"}/>
        </ul>
        <DictionaryTab workspace={[{filePath: "src/assets/first_file.txt", fileName: "first_file.txt"},{filePath: "src/assets/second_file.txt", fileName: "second_file.txt"}]}/>
        <Buffer fileName={"untitled.txt"} filePath={"/src/assets/untitled.txt"}/>
      </div>
      )

    }
  }

  return (
    <div className="container">
    {setMenuState()}
    </div>
  );
}

export default App;
