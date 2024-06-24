import { invoke } from "@tauri-apps/api/core";
import ClipBoard from "./components/ClipBoard";
import Navbar from "./components/Navbar";

function App() {
  return (
    <>
      <Navbar />
      <ClipBoard />
    </>
  )
}

export default App;