import useAuth from "./hooks/useAuth"
import Login from "./components/Login/Login"
import Home from "./components/Home/Home"
import Footer from "./components/Footer/Footer"
import "./App.css"

const App = () => {
  const { isLoggedIn, email, sessionAction, pendingAction, errorMessage, login, register, authorize, clearError } = useAuth()

  return (
    <div id="app-container">
      <div id="app-logo-container">
        <h1 id="app-logo">Zerrenda<span className="charcoal-gray">App</span></h1>
      </div>
      {isLoggedIn && email !== null && sessionAction !== null
        ? <Home email={email} sessionAction={sessionAction} authorize={authorize} />
        : <Login
            pendingAction={pendingAction}
            errorMessage={errorMessage}
            login={login}
            register={register}
            clearError={clearError}
          />}
      <Footer />
    </div>
  )
}

export default App
