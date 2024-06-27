import { invoke } from "@tauri-apps/api/core"
import { Button } from "@/components/ui/button"
import { 
    MoreHorizontal, 
    Copy, 
    ClipboardPaste,
    AlertCircle,
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

    useEffect(() => {
        invoke("read_config")
            .then((res) => {
                setConfig(res);
                setError(null);
            })
            .catch((err) => {
                setError(err);
            });
    } , [])

    return (
        <div>
        <Table>
            <TableCaption>Your current active clipboards.</TableCaption>
            <TableBody>
                {error && (
                    <Alert variant="destructive" className="text-red-400">
                        <AlertCircle className="h-5 w-5" />
                        <AlertTitle>Error</AlertTitle>
                        <AlertDescription>
                        <span className="bold">{error.title}</span>: {error.message}
                        </AlertDescription>
                    </Alert>
                )}
                {config.clipboards.map((clipboard) => (
                    <TableRow key={clipboard.id}>
                        <TableCell className="font-medium text-base text-center">{clipboard.name}</TableCell>
                        <TableCell className="text-right">
                            <Button size="sm" className="mx-2" variant='outline'>Copy <Copy className="ml-2 h-4 w-4" /></Button>
                            <Button size="sm" className="mx-1" variant='outline'>Paste <ClipboardPaste className="ml-2 h-4 w-4" /></Button>
                        {/* </TableCell>
                        <TableCell> */}
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
                                    <DropdownMenuItem>
                                        <Button variant="destructive">
                                            Delete Clipboard
                                        </Button>
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