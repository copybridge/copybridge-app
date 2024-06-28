import { invoke } from "@tauri-apps/api/core"
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
import { Button } from "@/components/ui/button"
import { 
    MoreHorizontal, 
    Copy, 
    ClipboardPaste,
    AlertCircle,
    Loader2,
    Check,
} from "lucide-react"
import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import {
    Alert,
    AlertDescription,
    AlertTitle,
} from "@/components/ui/alert"
import { useEffect, useState } from "react"


function ClipBoard() {
    let defaultConfig = {
        server: "",
        clipboards: []
    };
    const [config, setConfig] = useState(defaultConfig)
    const [error, setError] = useState(null);
    const [copyingStates, setCopyingStates] = useState({});
    const [pastingStates, setPastingStates] = useState({});

    useEffect(() => {
        invoke("read_config")
            .then((res) => {
                setConfig(res);
                setError(null);
                console.log("read config")
            })
            .catch((err) => {
                setError(err);
            });
    } , [])

    const onCopy = (id) => {
        setCopyingStates(prev => ({ ...prev, [id]: 'copying' }));
        // await new Promise((r) => setTimeout(() => r(), 2000));
        invoke("get_content", { config: config, args: {id: id, echo: false} })
            .then((content) => {
                setError(null);
                if (content.type === "text/plain") {
                    return writeText(content.data);
                } else {
                    throw new Error("Unsupported clipboard content type.");
                }
            })
            .then(() => {
                setCopyingStates(prev => ({ ...prev, [id]: 'copied' }));
                setTimeout(() => {
                    setCopyingStates(prev => ({ ...prev, [id]: null }));
                }, 2000);
            })
            .catch((err) => {
                setError(err);
                setCopyingStates(prev => ({ ...prev, [id]: null }));
            });
    }

    const onPaste = (id) => {
        setPastingStates(prev => ({ ...prev, [id]: 'pasting' }));
        readText()
            .then((content) => {
                setError(null);
                invoke("set_content", { config: config, args: {id: id, content: content} })
                    .then(() => {
                        setError(null);
                    })
                    .then(() => {
                        setPastingStates(prev => ({ ...prev, [id]: 'pasted' }));
                        setTimeout(() => {
                            setPastingStates(prev => ({ ...prev, [id]: null }));
                        }, 2000);
                    })
                    .catch((err) => {
                        setError(err);
                    });
            })
            .catch((err) => {
                setError({ title: "Failed to paste", message: err });
            });
    }

    return (
        <div>
        {error && (
            <Alert variant="destructive" className="text-red-400">
                <AlertCircle className="h-5 w-5" />
                <AlertTitle>Error</AlertTitle>
                <AlertDescription>
                <span className="bold">{error.title}</span>: {error.message}
                </AlertDescription>
            </Alert>
        )}
        <Table>
            <TableCaption>Your current active clipboards.</TableCaption>
            <TableBody>
                {config.clipboards.map((clipboard) => (
                    <TableRow key={clipboard.id}>
                        <TableCell className="font-medium text-base text-center">{clipboard.name}</TableCell>
                        <TableCell className="text-right">
                            <Button
                                onClick={() => onPaste(clipboard.id)}
                                size="sm"
                                className="mx-1"
                                variant='outline'
                                disabled={pastingStates[clipboard.id] === 'pasting'}
                            >
                                Paste
                                {pastingStates[clipboard.id] === 'pasting' && <Loader2 className="ml-2 h-4 w-4 animate-spin" />}
                                {pastingStates[clipboard.id] === 'pasted' && <Check className="ml-2 h-4 w-4" />}
                                {!pastingStates[clipboard.id] && <ClipboardPaste className="ml-2 h-4 w-4" />}
                            </Button>
                            <Button 
                                onClick={() => onCopy(clipboard.id)} 
                                size="sm" 
                                className="mx-2" 
                                variant='outline'
                                disabled={copyingStates[clipboard.id] === 'copying'}
                            >
                                Copy
                                {copyingStates[clipboard.id] === 'copying' && <Loader2 className="ml-2 h-4 w-4 animate-spin" />}
                                {copyingStates[clipboard.id] === 'copied' && <Check className="ml-2 h-4 w-4" />}
                                {!copyingStates[clipboard.id] && <Copy className="ml-2 h-4 w-4" />}
                            </Button>
                            <DropdownMenu className=''>
                                <DropdownMenuTrigger asChild>
                                    <Button variant="ghost" className="h-8 w-8 p-0">
                                        <span className="sr-only">Open menu</span>
                                        <MoreHorizontal className="h-4 w-4" />
                                    </Button>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent align="end">
                                    <DropdownMenuLabel>Actions</DropdownMenuLabel>
                                    <DropdownMenuItem>Copy ID</DropdownMenuItem>
                                    <DropdownMenuItem>Rename</DropdownMenuItem>
                                    <DropdownMenuItem>Remove</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem className="text-red-500 hover:text-red-600">
                                            Delete Clipboard
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
        </div>
    );
}

export default ClipBoard;