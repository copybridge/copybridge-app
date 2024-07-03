import { invoke } from "@tauri-apps/api/core";
import { AlertCircle } from "lucide-react";
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/components/ui/alert";
import ClipBoard from "./components/ClipBoard";
import Navbar from "./components/Navbar";
import { useEffect, useState } from "react";

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
    <>
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
      <ClipBoard config={config} setConfig={setConfig} setError={setError} /> 
    </>
  );
}

export default App;
