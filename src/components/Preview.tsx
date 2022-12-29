import ReactHtmlParser, {processNodes, convertNodeToElement, htmlparser2 } from "react-html-parser";

interface PreviewProps {
  input: String
}
const Preview = ({input}:PreviewProps) => {

  return(
    <div id="preview">
      {ReactHtmlParser(input)}
    </div>
  )
}

export default Preview;
