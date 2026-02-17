import { useEffect } from 'react';

function App() {
  useEffect(() => {
    async function startEngine() {
      const wasm = await import('./engine/engine.js');
      await wasm.default();
      await wasm.init();
    }
    startEngine();
  }, []);

  return (
    <canvas id="canvas" width={800} height={600}></canvas>
  );
}

export default App;
