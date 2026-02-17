import { useState } from 'react'
import { useEffect } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  useEffect(() => {
    async function startEngine() {
      const wasm = await import('./engine/engine.js');
      await wasm.default();
      wasm.init();
    }
    startEngine();
  }, []);
  
  return(
    <canvas id="canvas" width={800} height={600}></canvas>
  )
}

export default App
