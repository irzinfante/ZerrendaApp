import { useState } from 'react'
import reactLogo from './assets/react.svg'
import zerrendaAppLogo from '/zerrenda-app.svg'
import { Button } from '@chakra-ui/react'
import { FaPlus } from "react-icons/fa6";
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <div>
        <a href="http://localhost:5173" target="_blank" rel="noreferrer">
          <img src={zerrendaAppLogo} className="logo" alt="ZerrendaApp logo" />
        </a>
        <a href="https://react.dev" target="_blank" rel="noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <Button rightIcon={<FaPlus />} colorScheme='teal' variant='outline' onClick={() => { setCount((count) => count + 1) }}>
          count is {count}
        </Button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
