import { AuthAction, Authorize } from "../../hooks/useAuth"
import useDummy from "../../hooks/useDummy"
import useButton from "../../hooks/ui/useButton"
import Container from "../Container/Container"
import "./Home.css"

interface HomeProps {
  email: string
  sessionAction: AuthAction
  authorize: Authorize
}

const Home = ({ email, sessionAction, authorize }: HomeProps) => {
  const { isChecking, checkMessage, isCheckError, checkSession } = useDummy(authorize)

  const handleCheckSession = () => {
    void checkSession()
  }

  const { Button: CheckSessionButton } = useButton("Check session", isChecking, handleCheckSession, isChecking, "primary")

  return (
    <section id="home-section">
      <Container id="home-container">
        <h5>{sessionAction === "login" ? `Logged in as ${email}` : `Signed up as ${email}`}</h5>
        {CheckSessionButton}
        {checkMessage !== null && <span className={isCheckError ? "home-error" : "home-success"}>{checkMessage}</span>}
      </Container>
    </section>
  )
}

export default Home
