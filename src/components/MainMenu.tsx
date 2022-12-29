import notalLogo from "../assets/NotalLogo.svg";

const MainMenu = () => {
  return(
    <div id="menu">
      <img id="notal-logo"  src={notalLogo}/>
      <h2>Welcome to Notal!</h2>
      <div id="button-container">
        <button className="menu-button">New File</button>
        <button className="menu-button">Open File</button>
        <button className="menu-button">Open Folder</button>
      </div>
    </div>
  )
}


export default MainMenu;
