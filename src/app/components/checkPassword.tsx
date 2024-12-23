import Button from "./common/button";
import { useState } from "react";
import { Check, X } from "@geist-ui/icons";
import { invoke } from "@tauri-apps/api/core";

import { useModal } from "../hooks/useModal";

interface CheckPasswordProps {
  path: string;
  mode: string;
}

const CheckPassword = ({ path, mode }: CheckPasswordProps) => {
  const [verifyPassField, setVerifyPassField] = useState("");
  const [showIncorrectPass, setShowIncorrectPass] = useState(false);

  const { close } = useModal("checkPass");

  const handleLockUnlock = async () => {
    console.log("Check password");
    const command = `${mode}_vault`; // unlock_vault or lock_vault
    const params = { path: path, password: verifyPassField };

    console.log(command);

    try {
      await invoke(command, params);
    } catch {
      setShowIncorrectPass(true);
    } finally {
      close();
    }
  };

  const title = mode === "unlock" ? "Unlock vault" : "Lock vault";

  return (
    <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-20 bg-white/10">
      <form className="flex flex-col relative w-[400px] h-[147px] bg-black rounded-lg">
        {showIncorrectPass && <IncorrectPassword />}
        <div className="flex flex-col top-4 left-4 pt-4 px-4">
          <h1 className="text-2xl font-bold">{title}</h1>
          <input
            className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
            id="password"
            type="password"
            placeholder="Password"
            onChange={(e) => setVerifyPassField(e.target.value)}
          />
        </div>
        <div className="flex flex-row justify-end items-center w-full py-1 px-4">
          <Button onClick={async () => handleLockUnlock()}>
            <Check />
          </Button>
          <Button
            onClick={() => {
              console.log("Close check modal");
              close();
            }}
          >
            <X />
          </Button>
        </div>
      </form>
    </div>
  );
};

const IncorrectPassword: React.FC = () => {
  return (
    <div className="text-red-500 text-center py-2">
      Incorrect password. Please try again.
    </div>
  );
};

export default CheckPassword;
