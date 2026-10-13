import "./Footer.css"
import { GITHUB_TAG } from "../../constants/Environment"

const Footer = () => {
  return (
    <footer className="footer">
      <div className="footer-content">
        <span><a href={`https://github.com/irzinfante/ZerrendaApp/tree/${GITHUB_TAG}`}>{GITHUB_TAG}</a> - Copyright &copy; 2024-2026 Iker Ruiz de Infante Gonzalez <a href="mailto:iker@irzinfante.dev">iker@irzinfante.dev</a></span>
      </div>
    </footer>
  )
}

export default Footer
