import { invoke } from "@tauri-apps/api/core";
import { AlertCircle } from "lucide-react";
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/components/ui/alert";
import ClipBoard from "./components/ClipBoard";
import Navbar from "./components/Navbar";
import Settings from "./components/Settings"; // Make sure to import Settings
import { useEffect, useState } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

function App() {
  const defaultConfig = {
    server: "",
    clipboards: []
  };

  const [config, setConfig] = useState(defaultConfig);
  const [error, setError] = useState(null);

  useEffect(() => {
    invoke("read_config")
      .then((res) => {
        setConfig(res);
        setError(null);
        console.log("read config");
      })
      .catch((err) => {
        setError(err);
      });
  }, []);

  return (
    <BrowserRouter>
      <Navbar config={config} setConfig={setConfig} setError={setError} />
      {error && (
        <Alert variant="destructive" className="text-red-400">
          <AlertCircle className="h-5 w-5" />
          <AlertTitle>Error</AlertTitle>
          <AlertDescription>
            <span className="bold">{error.title}</span>: {error.message}
          </AlertDescription>
        </Alert>
      )}
      <Routes>
        <Route 
          path="/" 
          element={<ClipBoard config={config} setConfig={setConfig} setError={setError} />} 
        />
        <Route 
          path="/settings" 
          element={<Settings />} 
        />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
