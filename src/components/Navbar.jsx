import { useState } from 'react';
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Input } from "@/components/ui/input";
import { PasswordInput } from "@/components/ui/password";
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

const Navbar = () => {
  const [showPasswordFieldCreate, setShowPasswordFieldCreate] = useState(false);
  const [showPasswordFieldAdd, setShowPasswordFieldAdd] = useState(false);

  const [clipboardName, setClipboardName] = useState('');
  const [clipboardPassword, setClipboardPassword] = useState('');
  const [addClipboardId, setAddClipboardId] = useState('');
  const [addClipboardPassword, setAddClipboardPassword] = useState('');

  const openPasswordCreate = () => {
    setShowPasswordFieldCreate(!showPasswordFieldCreate);
  };

  const openPasswordAdd = () => {
    setShowPasswordFieldAdd(!showPasswordFieldAdd);
  };

  return (
    <nav className='flex flex-row justify-between p-2'>
      <p className='text-xl m-2'>CopyBridge</p>
      <Dialog>
        <DialogTrigger asChild>
          <Button size="sm" variant="" className='order-last m-1'> + New Clipboard</Button>
        </DialogTrigger>
        <DialogContent className="w-[400px]">
          <Tabs defaultValue="create-clipboard" className="w-[350px]">
            <TabsList>
              <TabsTrigger value="create-clipboard">Create Clipboard</TabsTrigger>
              <TabsTrigger value="add-clipboard">Add Clipboard</TabsTrigger>
            </TabsList>

            <TabsContent value="create-clipboard">
              <div className="space-y-1">
                <Label htmlFor="name">Clipboard Name</Label>
                <Input
                  id="name"
                  value={clipboardName}
                  onChange={(e) => setClipboardName(e.target.value)}
                />
              </div>
              {showPasswordFieldCreate && (
                <div className="space-y-1">
                  <Label htmlFor="password">Password</Label>
                  <PasswordInput
                    id="password"
                    value={clipboardPassword}
                    onChange={(e) => setClipboardPassword(e.target.value)}
                  />
                </div>
              )}
              <div className="flex items-center mt-3.5 justify-between">
                <div className='flex items-center space-x-2 mt-3.5'>
                  <label
                    htmlFor="encrypted"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                  >
                    Encrypt it
                  </label>
                  <Checkbox
                    id="encrypted"
                    onClick={openPasswordCreate}
                    checked={showPasswordFieldCreate}
                  />
                </div>
                <Button size="sm" className='order-last m-1'>Create</Button>
              </div>
            </TabsContent>
            <TabsContent value="add-clipboard">
              <div className="space-y-1">
                <Label htmlFor="clipboard-id">Clipboard ID</Label>
                <Input
                  id="clipboard-id"
                  value={addClipboardId}
                  onChange={(e) => setAddClipboardId(e.target.value)}
                />
              </div>
              {showPasswordFieldAdd && (
                <div className="space-y-1">
                  <Label htmlFor="add-password">Password</Label>
                  <PasswordInput
                    id="add-password"
                    value={addClipboardPassword}
                    onChange={(e) => setAddClipboardPassword(e.target.value)}
                  />
                </div>
              )}
              <div className="flex items-center mt-3.5 justify-between">
                <div className='flex items-center space-x-2 mt-3.5'>
                  <label
                    htmlFor="encrypted-add"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                  >
                    Is Encrypted
                  </label>
                  <Checkbox
                    id="encrypted-add"
                    onClick={openPasswordAdd}
                    checked={showPasswordFieldAdd}
                  />
                </div>
                <Button size="sm" className='order-last m-1'>Add</Button>
              </div>
            </TabsContent>
          </Tabs>
        </DialogContent>
      </Dialog>
    </nav>
  );
};

export default Navbar;
