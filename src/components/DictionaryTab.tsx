import { Key } from "react"

interface Workspace {
  fileName: String,
  filePath: String,
}

interface IDictionaryTab {
  workspace: Workspace[]
}

// Displays the workspace and the dictionary tab
const DictionaryTab = ({workspace}:IDictionaryTab) => {

  const mapFiles = () => {
    return workspace.map((fileProps) => {
      return(
        <div className="dic-tab-item"key={fileProps.filePath as Key}>
          <h4>{fileProps.fileName}</h4>
        </div>
      )

    })

  }
  return(
    <div id="dictionary-tab">
      <h4 style={{textAlign: "center", userSelect: "none", MozUserSelect: "none", msUserSelect: "none", cursor: "default"}}>Workspace</h4>
      {mapFiles()}
    </div>
  )
}

export default DictionaryTab;
