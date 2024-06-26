import { Button } from "@/components/ui/button"
import { 
    MoreHorizontal, 
    Copy, 
    ClipboardPaste,
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

const Config = {
    server: "",
    clipboards: [
      {
        id: 100004,
        name: "test",
        is_encrypted: true,
        password: "bXlwYXNzCg==" // base64 encoded for "mypass"
      },
      {
        id: 100005,
        name: "another test",
        is_encrypted: false,
        password: null // No password since is_encrypted is false
      }
    ]
  };

function ClipBoard() {
    return (
        <div>
        <Table>
            <TableCaption>Your current active clipboards.</TableCaption>
            <TableBody>
                {Config.clipboards.map((clipboard) => (
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
                                    <DropdownMenuItem>Copy Clipboard ID</DropdownMenuItem>
                                    <DropdownMenuItem>Rename Clipboard</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem>Remove Clipboard</DropdownMenuItem>
                                    <DropdownMenuItem>
                                        <Button variant="destructive">
                                            Delete Clipboard
                                        </Button>
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </TableCell>
                    </TableRow>
                )
            </TableBody>
        </Table>

        </div>
    );
}

export default ClipBoard;