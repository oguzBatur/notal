
interface TabProps {
  tabName: String
}


const Tab = ({tabName}:TabProps) => {

  return(
    <li className="tab">{tabName}</li>
  )
}

export default Tab;
