import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Button from "./common/button";
import { Check, X } from "@geist-ui/icons";
import { useModal } from "../hooks/useModal";

const EnterCredentials = ({
  userVaultDir, // Take the dir as a parameter since it was entered before this view
}: {
  userVaultDir: string;
}) => {
  // User entered directory, name and password for the vault to be created
  const [userVaultPassword, setUserVaultPassword] = useState("");
  const [userVaultName, setUserVaultName] = useState("");

  const { close } = useModal("enterCredentials");

  return (
    <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
      <form className="flex flex-col relative w-[400px] h-[250px] bg-black rounded-lg">
        <div className="flex flex-col top-4 left-4 pt-4 px-4">
          <h1 className="text-2xl font-bold">Name</h1>
          <input
            className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
            id="username"
            type="text"
            placeholder="Name for the vault"
            onChange={(e) => setUserVaultName(e.target.value)}
          />
        </div>
        <div className="flex flex-col top-4 left-4 pt-4 px-4">
          <h1 className="text-2xl font-bold">Password</h1>
          <input
            className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
            id="password"
            type="password"
            placeholder="Password"
            onChange={(e) => setUserVaultPassword(e.target.value)}
          />
        </div>
        <div className="flex flex-row justify-end items-center w-full py-2 px-4">
          <Button
            onClick={async () => {
              await invoke("create_secure_vault", {
                name: userVaultName,
                path: userVaultDir,
                password: userVaultPassword,
              });
              close();
            }}
          >
            <Check />
          </Button>
          <Button onClick={close}>
            <X />
          </Button>
        </div>
      </form>
    </div>
  );
};

export default EnterCredentials;
