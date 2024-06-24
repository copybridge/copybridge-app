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
  
  

function ClipBoard() {
    return (
        <div>
        <Table>
            <TableCaption>Your current active clipboards.</TableCaption>
            <TableBody>
                <TableRow>
                <TableCell className="font-medium text-base text-center">CN Lab Answers</TableCell>
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
                            <DropdownMenuSeparator />
                            <DropdownMenuItem>Delete Clipboard</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </TableCell>
                </TableRow>
            </TableBody>
        </Table>

        </div>
    );
}

export default ClipBoard;