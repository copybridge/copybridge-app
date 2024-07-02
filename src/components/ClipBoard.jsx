import { invoke } from "@tauri-apps/api/core"
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
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
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
  } from "@/components/ui/dialog"
  
import { useEffect, useState } from "react"


function ClipBoard({config, setConfig, setError}) {
    const [copyingStates, setCopyingStates] = useState({});
    const [pastingStates, setPastingStates] = useState({});
    const [showingContent, setShowingContent] = useState({});

    const onCopy = (id) => {
        setCopyingStates(prev => ({ ...prev, [id]: 'copying' }));
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
                        setPastingStates(prev => ({ ...prev, [id]: null }));
                    });
            })
            .catch((err) => {
                setError({ title: "Failed to paste", message: err });
                setPastingStates(prev => ({ ...prev, [id]: null }));
            });
    }

    const onShow = (id) => {
        invoke("get_content", { config: config, args: {id: id, echo: false} })
            .then((content) => {
                setError(null);
                if (content.type === "text/plain") {
                    setShowingContent(prev => ({ ...prev, [id]: content.data }));
                } else {
                    throw new Error("Unsupported clipboard content type.");
                }
            })
            .catch((err) => {
                setError(err);
            });
    }
    
    const onShowCopy = (id) => {
        if (!showingContent[id]) {
            onCopy(id);
        }
        setCopyingStates(prev => ({ ...prev, [id]: 'copying' }));
        writeText(showingContent[id])
            .then(() => {
                setError(null);
                setCopyingStates(prev => ({ ...prev, [id]: 'copied' }));
                setTimeout(() => {
                    setCopyingStates(prev => ({ ...prev, [id]: null }));
                }, 2000);
            })
            .catch((err) => {
                setError({ title: "Failed to copy content", message: err });
                setCopyingStates(prev => ({ ...prev, [id]: null }));
            });
    }

    const onCopyID = (id) => {
        writeText(String(id))
            .then(() => {
                setError(null);
            })
            .catch((err) => {
                setError({ title: "Failed to copy ID", message: err });
            });
    }

    const onRemove = (id) => {
        invoke("remove", { config: config, args: {id: id} })
            .then((config) => {
                setError(null);
                setConfig(config);
            })
            .catch((err) => {
                setError(err);
            });
    }

    const onDelete = (id) => {
        invoke("delete", { config: config, args: {id: id, force: false} })
            .then((config) => {
                setError(null);
                setConfig(config);
            })
            .catch((err) => {
                setError(err);
            });
    }

    return (
        <div>
        <Table>
            <TableCaption>Your current active clipboards.</TableCaption>
            <TableBody>
                {config.clipboards.map((clipboard) => (
                    <TableRow key={clipboard.id}>
                        <Dialog>
                        <DialogTrigger asChild>
                            <TableCell
                                className="font-medium text-base text-center cursor-pointer"
                                onClick={() => onShow(clipboard.id)}
                            >
                                {clipboard.name}
                            </TableCell>
                        </DialogTrigger>
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
                                <DialogContent className="w-[400px]">
                                    <DialogHeader>
                                    <DialogTitle></DialogTitle>
                                    <div className="grid w-full gap-1.5">
                                        <Label htmlFor="show">Content</Label>
                                        <Textarea 
                                            id="show"
                                            value={showingContent[clipboard.id]}
                                            placeholder="Loading..."
                                            className="select-text"
                                            disabled
                                        />
                                        <Button 
                                            onClick={() => onShowCopy(clipboard.id)} 
                                            size="sm" 
                                            className="mx-2" 
                                            variant=""
                                            disabled={copyingStates[clipboard.id] === 'copying'}
                                        >
                                            Copy
                                            {copyingStates[clipboard.id] === 'copying' && <Loader2 className="ml-2 h-4 w-4 animate-spin" />}
                                            {copyingStates[clipboard.id] === 'copied' && <Check className="ml-2 h-4 w-4" />}
                                            {!copyingStates[clipboard.id] && <Copy className="ml-2 h-4 w-4" />}
                                        </Button>
                                    </div>
                                    </DialogHeader>
                                </DialogContent>

                            <DropdownMenu className=''>
                                <DropdownMenuTrigger asChild>
                                    <Button variant="ghost" className="h-8 w-8 p-0">
                                        <span className="sr-only">Open menu</span>
                                        <MoreHorizontal className="h-4 w-4" />
                                    </Button>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent align="end">
                                    <DropdownMenuLabel>Actions</DropdownMenuLabel>
                                    <DropdownMenuItem onClick={() => {onCopyID(clipboard.id)}} >Copy ID</DropdownMenuItem>
                                    <DropdownMenuItem>Rename</DropdownMenuItem>
                                    <DropdownMenuItem onClick={() => {onRemove(clipboard.id)}}>Remove</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem
                                        className="text-red-500 hover:text-red-600"
                                        onClick={() => {onDelete(clipboard.id)}}
                                    >
                                            Delete Clipboard
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </TableCell>
                        </Dialog>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
        </div>
    );
}

export default ClipBoard;